//! Explicit configuration ID at init, and diagnostic init/reinit, against the
//! real engine (no mocks).
//!
//! Covers parity with the Java/C#/Swift SDKs:
//! - `SzEnvironmentCore::builder().config_id(..)` starts the engine and the
//!   diagnostic with `Sz_initWithConfigID` / `SzDiagnostic_initWithConfigID`.
//! - `get_diagnostic()` runs `SzDiagnostic_init`, and `reinitialize()` runs
//!   `SzDiagnostic_reinit` when the diagnostic is already initialized.
//! - A failed init keeps its native error code and category for every caller.
//!
//! Every test uses the `internal://` in-memory datastore. Each destroys any
//! active singleton first and registers the configurations it needs. Its own
//! test binary, so no other test shares the singleton or the datastore.

use serial_test::serial;
use std::sync::Arc;
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

/// Data source that exists only in the alternate (non-default) configuration.
const ALT_DATA_SOURCE: &str = "CFGID_ALT";

/// A configuration ID no datastore has registered.
const UNREGISTERED_CONFIG_ID: ConfigId = 1_234_567;

/// Destroys the active singleton, if any, so the next environment is fresh.
fn destroy_active_environment() -> SzResult<()> {
    match SzEnvironmentCore::try_get_instance() {
        Some(env) => env.destroy(),
        None => Ok(()),
    }
}

/// Engine settings `ExampleEnvironment` uses (paths + `internal://`), read from
/// a throwaway environment that is destroyed again.
fn example_settings() -> SzResult<String> {
    destroy_active_environment()?;
    let env = ExampleEnvironment::initialize("sz-config-id-settings")?;
    let settings = env.get_ini_params().to_string();
    env.destroy()?;
    Ok(settings)
}

/// Registers the template configuration and makes it the default.
fn register_default_config(config_mgr: &dyn SzConfigManager) -> SzResult<ConfigId> {
    let config = config_mgr.create_config()?;
    let config_id = config_mgr.register_config(&config.export()?, Some("Default configuration"))?;
    config_mgr.set_default_config_id(config_id)?;
    Ok(config_id)
}

/// Registers `base_id` plus `ALT_DATA_SOURCE` as a new, non-default configuration.
fn register_alt_config(config_mgr: &dyn SzConfigManager, base_id: ConfigId) -> SzResult<ConfigId> {
    let config = config_mgr.create_config_from_id(base_id)?;
    config.register_data_source(ALT_DATA_SOURCE)?;
    config_mgr.register_config(&config.export()?, Some("Alternate configuration"))
}

/// Adds one record under `ALT_DATA_SOURCE`, reads it back, then purges the
/// repository.
///
/// The purge matters: `internal://` data outlives `destroy()` within the
/// process, and once `ALT_DATA_SOURCE` has been loaded, every later init with a
/// configuration lacking it fails with SENZ0060 (deleting the record is not
/// enough).
fn add_and_verify_alt_record(env: &SzEnvironmentCore, record_id: &str) -> SzResult<()> {
    let engine = env.get_engine()?;
    engine.add_record(
        ALT_DATA_SOURCE,
        record_id,
        r#"{"NAME_FULL": "Robert Smith", "PHONE_NUMBER": "702-555-0100"}"#,
        None,
    )?;
    let record = engine.get_record(ALT_DATA_SOURCE, record_id, None)?;
    if !record.contains(record_id) {
        return Err(SzError::bad_input(format!(
            "Record {ALT_DATA_SOURCE}/{record_id} was added but get_record returned: {record}"
        )));
    }
    env.get_diagnostic()?.purge_repository()
}

/// Asserts `error` carries a native code from `component`, i.e. it was not
/// flattened into a code-less `Unrecoverable`.
fn assert_native_error(error: &SzError, component: SzComponent, context: &str) {
    assert!(
        error.error_code().is_some(),
        "{context}: expected a native error code, got: {error}"
    );
    assert_eq!(
        error.component(),
        Some(component),
        "{context}: expected component {component:?}, got: {error}"
    );
}

/// `builder().config_id(X)` starts the engine with configuration X, not the
/// datastore default: the active config is X and X's extra data source works.
#[test]
#[serial]
fn test_builder_config_id_starts_engine_with_that_config() -> SzResult<()> {
    let settings = example_settings()?;

    // Learn the alternate configuration's ID from a throwaway environment, so
    // the builder can be given it before the next environment exists. The
    // next environment registers the same JSON again and checks the ID.
    let probe = SzEnvironmentCore::builder()
        .module_name("sz-config-id-probe")
        .settings(&settings)
        .build()?;
    let probe_mgr = probe.get_config_manager()?;
    let probe_default = register_default_config(probe_mgr.as_ref())?;
    let alt_id = register_alt_config(probe_mgr.as_ref(), probe_default)?;
    drop(probe_mgr);
    probe.destroy()?;

    let env = SzEnvironmentCore::builder()
        .module_name("sz-config-id-explicit")
        .settings(&settings)
        .config_id(alt_id)
        .build()?;
    assert_eq!(env.get_config_id(), Some(alt_id));

    // Register both configurations before the engine starts; the default stays
    // the template, so only an explicit-ID init can pick up the alternate.
    let config_mgr = env.get_config_manager()?;
    let default_id = register_default_config(config_mgr.as_ref())?;
    let registered_alt = register_alt_config(config_mgr.as_ref(), default_id)?;
    assert_eq!(
        registered_alt, alt_id,
        "re-registering the same configuration JSON must yield the same ID"
    );
    assert_ne!(default_id, alt_id, "alternate must differ from the default");
    assert_eq!(config_mgr.get_default_config_id()?, default_id);

    assert_eq!(
        env.get_active_config_id()?,
        alt_id,
        "engine must start with the builder's config ID, not the default {default_id}"
    );
    add_and_verify_alt_record(&env, "CFGID_EXPLICIT_1")?;

    // The diagnostic initializes with the same explicit ID.
    let diagnostic = env.get_diagnostic()?;
    let info = diagnostic.get_repository_info()?;
    assert!(
        info.contains("dataStores"),
        "unexpected repository info: {info}"
    );

    drop((config_mgr, diagnostic));
    env.destroy()
}

/// An unregistered config ID fails the engine and diagnostic inits with their
/// native codes, and every later call gets the same code and category.
#[test]
#[serial]
fn test_unregistered_config_id_keeps_native_error() -> SzResult<()> {
    let settings = example_settings()?;
    let env = SzEnvironmentCore::builder()
        .module_name("sz-config-id-unregistered")
        .settings(&settings)
        .config_id(UNREGISTERED_CONFIG_ID)
        .build()?;
    register_default_config(env.get_config_manager()?.as_ref())?;

    let first = env
        .get_engine()
        .err()
        .expect("Sz_initWithConfigID must reject an unregistered config ID");
    eprintln!("unregistered config ID: {first}");
    assert_native_error(&first, SzComponent::Engine, "first get_engine");
    assert!(
        first.to_string().contains("Sz_init failed"),
        "error should name the failed init: {first}"
    );

    // The init ran once; the replayed error must be indistinguishable in kind.
    let replayed = env
        .get_active_config_id()
        .expect_err("engine init failure must be returned to every caller");
    assert_eq!(replayed.error_code(), first.error_code());
    assert_eq!(replayed.component(), first.component());
    assert_eq!(replayed.category(), first.category());
    assert_eq!(replayed.hierarchy(), first.hierarchy());

    let diagnostic_error = env
        .get_diagnostic()
        .err()
        .expect("SzDiagnostic_initWithConfigID must reject an unregistered config ID");
    assert_native_error(&diagnostic_error, SzComponent::Diagnostic, "get_diagnostic");
    assert!(
        diagnostic_error
            .to_string()
            .contains("SzDiagnostic_init failed"),
        "error should name the failed init: {diagnostic_error}"
    );

    env.destroy()
}

/// `get_diagnostic()` runs `SzDiagnostic_init` on its own (no engine needed),
/// and a later `reinitialize()` re-inits the already-initialized diagnostic.
#[test]
#[serial]
fn test_reinitialize_reinits_initialized_diagnostic() -> SzResult<()> {
    destroy_active_environment()?;
    let env = ExampleEnvironment::initialize("sz-config-id-reinit-diag")?;
    assert_eq!(env.get_config_id(), None, "no explicit config ID yet");

    let diagnostic = env.get_diagnostic()?;
    let info = diagnostic.get_repository_info()?;
    assert!(
        info.contains("dataStores"),
        "unexpected repository info: {info}"
    );

    let config_mgr = env.get_config_manager()?;
    let default_id = config_mgr.get_default_config_id()?;
    let alt_id = register_alt_config(config_mgr.as_ref(), default_id)?;

    // Engine starts here (default config), then both subsystems switch to alt.
    env.reinitialize(alt_id)?;
    assert_eq!(env.get_config_id(), Some(alt_id));
    assert_eq!(env.get_active_config_id()?, alt_id);
    add_and_verify_alt_record(&env, "CFGID_REINIT_DIAG_1")?;

    let info = diagnostic.get_repository_info()?;
    assert!(
        info.contains("dataStores"),
        "diagnostic must keep working after SzDiagnostic_reinit: {info}"
    );

    drop((diagnostic, config_mgr));
    env.destroy()
}

/// A diagnostic first requested after `reinitialize()` initializes with the
/// reinitialized config ID (`SzDiagnostic_initWithConfigID`).
#[test]
#[serial]
fn test_diagnostic_after_reinitialize_uses_new_config_id() -> SzResult<()> {
    destroy_active_environment()?;
    let env = ExampleEnvironment::initialize("sz-config-id-diag-after")?;
    let config_mgr = env.get_config_manager()?;
    let default_id = config_mgr.get_default_config_id()?;
    let alt_id = register_alt_config(config_mgr.as_ref(), default_id)?;

    env.reinitialize(alt_id)?;
    assert_eq!(env.get_config_id(), Some(alt_id));

    let diagnostic = env.get_diagnostic()?;
    let info = diagnostic.get_repository_info()?;
    assert!(
        info.contains("dataStores"),
        "unexpected repository info: {info}"
    );
    add_and_verify_alt_record(&env, "CFGID_DIAG_AFTER_1")?;

    // A failed reinitialize leaves the recorded config ID unchanged.
    let error = env
        .reinitialize(UNREGISTERED_CONFIG_ID)
        .expect_err("Sz_reinit must reject an unregistered config ID");
    assert_native_error(&error, SzComponent::Engine, "reinitialize");
    assert_eq!(env.get_config_id(), Some(alt_id));

    drop((diagnostic, config_mgr));
    env.destroy()
}

/// The builder returns the active singleton only when a requested config ID
/// matches it; `get_instance` (no config ID) keeps accepting any.
#[test]
#[serial]
fn test_builder_config_id_must_match_active_environment() -> SzResult<()> {
    destroy_active_environment()?;
    let env = ExampleEnvironment::initialize("sz-config-id-singleton")?;
    let settings = env.get_ini_params().to_string();
    let builder = SzEnvironmentCore::builder()
        .module_name("sz-config-id-singleton-2")
        .settings(&settings)
        .verbose_logging(false);

    let same = builder.clone().config_id(None).build()?;
    assert!(
        Arc::ptr_eq(&env, &same),
        "no config ID: returns the active instance"
    );
    drop(same);

    match builder.clone().verbose_logging(true).build() {
        Err(SzError::Configuration(_)) => {}
        Err(other) => panic!("expected a Configuration error, got: {other}"),
        Ok(_) => panic!("verbose_logging differs from the active instance; must be refused"),
    }

    let default_id = env.get_config_manager()?.get_default_config_id()?;
    let conflict = builder.clone().config_id(default_id).build();
    match conflict {
        Err(SzError::Configuration(_)) => {}
        Err(other) => panic!("expected a Configuration error, got: {other}"),
        Ok(_) => panic!("config ID {default_id} differs from the active None; must be refused"),
    }

    env.reinitialize(default_id)?;
    let pinned = builder.config_id(default_id).build()?;
    assert!(
        Arc::ptr_eq(&env, &pinned),
        "matching config ID: returns the active instance"
    );
    let via_get_instance =
        SzEnvironmentCore::get_instance("sz-config-id-singleton-3", &settings, false)?;
    assert!(Arc::ptr_eq(&env, &via_get_instance));

    drop((pinned, via_get_instance));
    env.destroy()
}

## Initialization Patterns

### ExampleEnvironment (for examples and tests)

The simplest way to get a working environment:

```rust
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

let env = ExampleEnvironment::initialize("my-app")?;
let engine = env.get_engine()?;
```

`ExampleEnvironment` auto-detects Senzing paths, uses an `internal://` in-memory database (v4.3+), sets up initial configuration, and returns an `Arc<SzEnvironmentCore>`. No temp files or schema setup required.

### SenzingGuard (RAII cleanup)

```rust
let env = SenzingGuard::from_env(ExampleEnvironment::initialize("my-app")?);
let engine = env.get_engine()?;
// Cleanup happens automatically when guard drops
```

### Manual Initialization

```rust
let settings = r#"{"PIPELINE": {...}, "SQL": {"CONNECTION": "..."}}"#;
let env = SzEnvironmentCore::get_instance("my-app", settings, false)?;
let engine = env.get_engine()?;
// ...
env.destroy()?;
```

### Explicit Configuration ID (builder)

```rust
let env = SzEnvironmentCore::builder()
    .module_name("my-app")
    .settings(settings)
    .config_id(config_id) // ConfigId or Option<ConfigId>
    .build()?;            // same singleton as get_instance
```

- With a config ID, the engine inits with `Sz_initWithConfigID` and the diagnostic with `SzDiagnostic_initWithConfigID`; without one, `Sz_init` / `SzDiagnostic_init` (datastore default).
- Inits stay lazy: an unregistered ID surfaces at the first `get_engine()` / `get_diagnostic()` as `Configuration` SENZ7221.
- `build()` refuses (`Configuration`) a config ID that differs from the active singleton's `get_config_id()`; switch with `reinitialize(id)`.
- `reinitialize(id)` starts the engine if needed, then `Sz_reinit`, then `SzDiagnostic_reinit` if the diagnostic is initialized; on success `get_config_id()` becomes `Some(id)` and later diagnostic inits use it.

### Subsystem Init Guards

Each native `*_init` (`Sz_init`, `SzConfigMgr_init`, `SzProduct_init`, `SzDiagnostic_init`) runs once behind an `InitGuard` (`src/core/environment.rs`): other threads block until it finishes, and a failure is replayed to every caller with its native code, component and category. `get_diagnostic()` does not start the engine (Java/C# parity).

### Key Points

- `get_instance()` is a singleton — returns the same `Arc` for the same process
- `ExampleEnvironment` is `#[doc(hidden)]` — internal helper, not public API
- Each `get_engine()` call returns a new engine instance suitable for one thread
- Always `destroy()` or use `SenzingGuard` for cleanup

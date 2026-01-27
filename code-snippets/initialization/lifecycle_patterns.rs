//! Environment Lifecycle Patterns
//!
//! This example demonstrates the complete lifecycle of the Senzing environment,
//! showing both RAII (recommended) and manual cleanup patterns.
//!
//! # Key Concepts
//!
//! - **Singleton Pattern**: Only one environment exists per process
//! - **Arc Reference Counting**: `get_instance()` returns Arc with count >= 2
//! - **RAII with SenzingGuard**: Automatic cleanup on scope exit
//! - **Manual with cleanup()/destroy()**: Explicit control over resource release

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    println!("=== Senzing Environment Lifecycle Patterns ===\n");

    // Pattern 1: RAII with SenzingGuard (Recommended)
    pattern_raii_guard()?;

    // Pattern 2: Manual management with cleanup()
    pattern_manual_cleanup()?;

    println!("\n=== All patterns completed successfully ===");
    Ok(())
}

/// Pattern 1: RAII Guard (Recommended)
///
/// Uses SenzingGuard for automatic cleanup when the guard goes out of scope.
/// This is the most idiomatic Rust pattern.
fn pattern_raii_guard() -> SzResult<()> {
    println!("--- Pattern 1: RAII Guard ---");

    // For this example, we use ExampleEnvironment helper which creates an
    // isolated test database. In production, you'd use SenzingGuard::new() directly.
    let env = ExampleEnvironment::initialize("lifecycle-raii")?;

    // Wrap in SenzingGuard for automatic cleanup
    let guard = SenzingGuard::from_env(env);

    // Access SDK components through the guard (Deref makes this seamless)
    let product = guard.get_product()?;
    let version = product.get_version()?;
    println!("  Senzing version: {}", &version[..50.min(version.len())]);

    let engine = guard.get_engine()?;
    println!("  Engine ready: {:p}", &*engine);

    // Drop components before guard
    drop(engine);
    drop(product);

    // When guard drops, cleanup happens automatically via Drop trait
    println!("  Guard going out of scope - automatic cleanup...");
    drop(guard);

    println!("  Done.\n");
    Ok(())
}

/// Pattern 2: Manual Management with cleanup()
///
/// Uses ExampleEnvironment::cleanup(env) for explicit cleanup.
/// cleanup() takes ownership of the Arc, ensuring clean semantics.
fn pattern_manual_cleanup() -> SzResult<()> {
    println!("--- Pattern 2: Manual cleanup() ---");

    // Get environment instance
    let env = ExampleEnvironment::initialize("lifecycle-manual")?;

    // Check the reference count (educational - don't do this in production)
    println!(
        "  Arc strong_count after get_instance: {}",
        std::sync::Arc::strong_count(&env)
    );
    println!("  (>= 2 is expected: singleton storage + caller)");

    // Get SDK components
    let product = env.get_product()?;
    let version = product.get_version()?;
    println!("  Got product version ({} chars)", version.len());

    // Components must be dropped before cleanup
    drop(product);
    println!("  Dropped component references");

    // cleanup() takes ownership of env - consumes it
    ExampleEnvironment::cleanup(env)?;

    println!("  Done.\n");
    Ok(())
}

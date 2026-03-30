## Initialization Patterns

### ExampleEnvironment (for examples and tests)

The simplest way to get a working environment:

```rust
use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

let env = ExampleEnvironment::initialize("my-app")?;
let engine = env.get_engine()?;
```

`ExampleEnvironment` auto-detects Senzing paths, creates an isolated SQLite database, sets up initial configuration, and returns an `Arc<SzEnvironmentCore>`.

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

### Key Points

- `get_instance()` is a singleton — returns the same `Arc` for the same process
- `ExampleEnvironment` is `#[doc(hidden)]` — internal helper, not public API
- Each `get_engine()` call returns a new engine instance suitable for one thread
- Always `destroy()` or use `SenzingGuard` for cleanup

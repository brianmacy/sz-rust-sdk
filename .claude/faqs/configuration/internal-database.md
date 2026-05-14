## internal:// In-Memory Database (v4.3+)

### What is it?

Senzing v4.3 introduced `internal://` as a database connection type. It creates a purely in-memory database that requires no schema setup, no temp files, and no external dependencies like `rusqlite`.

### Connection string

```json
{"SQL": {"CONNECTION": "internal://"}}
```

### Key behaviors

- Each `SzConfigMgr_init` + `Sz_init` pair shares the same in-memory DB **within a single environment lifecycle**
- The DB is ephemeral — destroyed when the environment is destroyed
- `get_default_config_id()` returns `Ok(0)` (not `Err`) when no config exists — check for `id == 0`
- Config must be registered via `get_config_manager()` BEFORE calling `get_engine()` (which triggers `Sz_init`)
- Pre-4.3 SDKs will fail with `SENZ0087: Unrecognized database type`

### Initialization order (critical)

```
1. SzEnvironmentCore::get_instance(...)   // creates struct, no native init yet
2. env.get_config_manager()               // triggers SzConfigMgr_init
3. Register default config if needed      // config_id == 0 means none
4. env.get_engine()                       // triggers Sz_init, finds config
```

Do NOT create a temp environment to register config and then destroy it — with `internal://` the config disappears with the environment.

### When to use sqlite3:// instead

- `test_senz7220.rs` intentionally uses `sqlite3://` to get an empty DB with no config (to trigger the error)
- Production deployments that need persistent data across restarts
- Multi-process scenarios that need shared database access

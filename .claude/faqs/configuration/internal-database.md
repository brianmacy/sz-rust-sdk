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

### Concurrent adds that resolve to one entity fail with SENZ1011 (native race)

On `internal://`, several threads calling `add_record` at the same time for records that resolve to the **same** entity intermittently fail with:

```
SENZ1011|Unique Constraint Violation 'OBS_ENT already exists [obsEntID=N dsrcID=1] - refusing to overwrite a row this operation did not create'
```

Measured 2026-11-22 (4 threads, identical `NAME_FULL`, distinct `RECORD_ID`s):

| Setup                                  | Failures |
| -------------------------------------- | -------- |
| `internal://`, Senzing 4.5.0.26320     | 17/30    |
| `internal://`, 4.6.0.26325 (staging)   | 11/20    |
| `internal://`, 1 thread                | 0/20     |
| `internal://`, 4 distinct people       | 0/30     |
| `sqlite3://` file DB, 4 threads        | 0/20     |

It is a native `internal://` bug, not an SDK one. Tests, doctests and examples that add from several threads must use records that do NOT resolve together (the `SzEnvironmentCore` thread-safety doctest uses four different names for this reason). Use a file-backed database to exercise concurrent same-entity loads.

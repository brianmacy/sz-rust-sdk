## Threading Model

### No Async/Await — Thread Pools Only

The Senzing SDK uses a synchronous design. Do NOT use tokio, async/await, or async runtimes. Use `std::thread` and `mpsc` channels for coordination.

### Why Thread Pools

Each thread gets its own engine instance. The engines are thread-safe at the C library level. This scales linearly with real OS threads.

### Thread Scaling Examples

Examples must demonstrate proper scaling with 4+ OS threads showing per-thread performance metrics. See `code-snippets/loading/load_via_loop_threadpool.rs` and `code-snippets/searching/search_threadpool.rs` for patterns.

### Singleton Pattern

`SzEnvironmentCore` implements a singleton pattern. `get_instance()` returns `Arc<SzEnvironmentCore>` — the same instance is shared across all threads. Individual engine/config/diagnostic instances are obtained per-thread via `env.get_engine()`.

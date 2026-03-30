## Flags and Defaults

### Flag Constants

Flags are defined as `SzFlags` bitflags in `src/flags.rs`. Common patterns from code-snippets:

```rust
SzFlags::ADD_RECORD_DEFAULT        // Default for add_record
SzFlags::DELETE_RECORD_DEFAULT     // Default for delete_record
SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT  // Default for search
SzFlags::SEARCH_BY_ATTRIBUTES_ALL     // All search info
SzFlags::WHY_ENTITY_DEFAULT        // Default for why analysis
SzFlags::EXPORT_DEFAULT            // Default for exports
SzFlags::ENTITY_INCLUDE_RECORD_DATA   // Include raw record data
SzFlags::WITH_INFO                 // Return affected entity info (add/delete/reevaluate)
SzFlags::REEVALUATE_ENTITY_DEFAULT // Default for reevaluate
```

### Using Flags

Most SDK methods take `Option<SzFlags>`. Pass `None` for defaults or `Some(flags)` for specific behavior:

```rust
// Default behavior
engine.add_record("TEST", "1001", record, None)?;

// With info about affected entities
engine.add_record("TEST", "1001", record, Some(SzFlags::WITH_INFO))?;

// Combine flags with bitwise OR
let flags = SzFlags::ENTITY_INCLUDE_RECORD_DATA | SzFlags::WITH_INFO;
```

### Data Source: "TEST"

The "TEST" data source is always available — no configuration setup required. Use it for examples and quick tests.

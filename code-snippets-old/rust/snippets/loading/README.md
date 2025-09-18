# Loading Examples

This directory contains examples demonstrating different approaches to loading records into the Senzing repository for entity resolution.

## Examples

### [load_records](load_records/)
**Purpose**: Demonstrates basic record loading with simple, structured data using in-memory records.

**What it demonstrates**:
- Basic record loading workflow
- JSON record construction with `serde_json`
- Multiple data source handling
- Simple error handling and progress reporting
- Record validation and processing

**Key API calls**:
- `engine.add_record(data_source, record_id, json_data, None)`
- JSON record construction
- Progress tracking with stdout flushing

**Data Sources Used**:
- `CUSTOMERS` - Customer records with names, phones, emails, addresses
- `EMPLOYEES` - Employee records with similar structure

**Usage**:
```bash
cd load_records
cargo run
```

### [load_via_loop](load_via_loop/)
**Purpose**: Advanced record loading from JSONL files with comprehensive error handling, retry logic, and performance tracking.

**What it demonstrates**:
- JSONL file processing line by line
- Comprehensive error handling with detailed logging
- Failed record retry file generation
- Performance statistics tracking with atomic counters
- File I/O operations and buffered reading
- Large-scale data processing patterns

**Key API calls**:
- `engine.add_record()` with error handling
- File I/O with `BufReader` and `BufWriter`
- Atomic counter operations for statistics

**Features**:
- Processes JSONL files (default: `../../resources/data/load-500.jsonl`)
- Creates retry files for failed records
- Tracks success/error/retry counts
- Progress reporting every 100 records
- Detailed error logging with line numbers

**Usage**:
```bash
cd load_via_loop
cargo run [path/to/jsonl/file]
```

## Common Patterns

### Basic Record Loading
```rust
engine.add_record(
    data_source_code,
    record_id,
    &json_record,
    None,  // No flags
)?;
```

### JSON Record Construction
```rust
let record = json!({
    "RECORD_ID": "123",
    "NAME_FIRST": "John",
    "NAME_LAST": "Smith",
    "PHONE_NUMBER": "555-1234",
    "EMAIL_ADDRESS": "john.smith@example.com",
    "ADDR_FULL": "123 Main St, City, ST 12345"
});
```

### Error Handling with Statistics
```rust
match engine.add_record(data_source, record_id, json_data, None) {
    Ok(()) => {
        SUCCESS_COUNT.fetch_add(1, Ordering::Relaxed);
    },
    Err(e) => {
        ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
        eprintln!("Error processing record {}: {}", record_id, e);
    }
}
```

### File Processing Pattern
```rust
let file = File::open(file_path)?;
let reader = BufReader::new(file);

for (line_number, line_result) in reader.lines().enumerate() {
    let line = line_result?;
    if line.trim().is_empty() { continue; }

    // Process JSON line
    let record: Value = serde_json::from_str(&line)?;
    // Extract fields and load record
}
```

## Record Format Requirements

Records must include:
- `RECORD_ID` - Unique identifier within the data source
- At least one identifying feature (name, phone, email, address, etc.)

Common record fields:
- `NAME_FIRST`, `NAME_LAST` - Person names
- `PHONE_NUMBER` - Phone numbers
- `EMAIL_ADDRESS` - Email addresses
- `ADDR_FULL` - Full addresses
- `DATE_OF_BIRTH` - Birth dates
- `SSN_NUMBER` - Social Security Numbers (where applicable)

## Data Sources

Examples use these data sources:
- `CUSTOMERS` - Customer records
- `EMPLOYEES` - Employee records
- `DELETE_TEST` - Test records for deletion examples

## Performance Considerations

### Batch Processing
- Process records in batches for better performance
- Use buffered I/O for large files
- Monitor memory usage with large datasets

### Error Recovery
- Save failed records to retry files
- Track detailed statistics for monitoring
- Log errors with sufficient context for debugging

### Progress Reporting
- Report progress periodically (e.g., every 100 records)
- Use atomic counters for thread-safe statistics
- Flush stdout for real-time feedback

## File Formats

### JSONL (JSON Lines)
Each line contains a complete JSON record:
```
{"RECORD_ID":"1","NAME_FIRST":"John","NAME_LAST":"Smith","PHONE_NUMBER":"555-1234"}
{"RECORD_ID":"2","NAME_FIRST":"Jane","NAME_LAST":"Doe","PHONE_NUMBER":"555-5678"}
```

### Retry Files
Failed records are written to retry files with naming pattern:
- `retry-<original_filename>.jsonl`
- Contains exact JSON that failed to load
- Can be reprocessed after fixing issues

## Best Practices

1. **Validate data** before calling `add_record()`
2. **Use unique record IDs** within each data source
3. **Handle errors gracefully** with retry mechanisms
4. **Monitor performance** with statistics tracking
5. **Process large files** in streaming fashion
6. **Provide progress feedback** for long-running operations
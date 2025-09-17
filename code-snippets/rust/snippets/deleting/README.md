# Deleting Examples

This directory contains examples demonstrating how to safely delete records from the Senzing repository with proper verification and error handling.

## Examples

### [delete_records](delete_records/)
**Purpose**: Demonstrates comprehensive record deletion workflow with verification and error handling.

**What it demonstrates**:
- Record loading for deletion testing
- Safe record deletion operations
- Deletion verification procedures
- Error handling and recovery
- Complete deletion workflow from load to verify

**Key API calls**:
- `engine.add_record()` - Load test records
- `engine.delete_record()` - Delete records
- `engine.get_record()` - Verify deletion
- Comprehensive error handling patterns

**Workflow**:
1. **Load Test Records**: Creates sample records for deletion testing
2. **Delete Records**: Performs deletion operations with error handling
3. **Verify Deletion**: Confirms records no longer exist in repository
4. **Error Recovery**: Handles and reports deletion failures

**Usage**:
```bash
cd delete_records
cargo run
```

## Common Patterns

### Record Deletion
```rust
match engine.delete_record(data_source_code, record_id, None) {
    Ok(()) => {
        println!("✓ Successfully deleted record {}", record_id);
        verify_record_deletion(engine, data_source_code, record_id)?;
    },
    Err(e) => {
        eprintln!("✗ Failed to delete record {}: {}", record_id, e);
        return Err(e);
    }
}
```

### Deletion Verification
```rust
match engine.get_record(data_source_code, record_id, None) {
    Ok(_record_json) => {
        eprintln!("⚠ WARNING: Record {} still exists after deletion!", record_id);
    },
    Err(e) => {
        if e.to_string().contains("record not found") {
            println!("✓ Verified: Record {} no longer exists", record_id);
        } else {
            eprintln!("⚠ Unexpected error during verification: {}", e);
        }
    }
}
```

### Test Data Creation
```rust
let test_records = HashMap::new();
records.insert(
    ("DELETE_TEST".to_string(), "DEL_001".to_string()),
    json!({
        "RECORD_ID": "DEL_001",
        "NAME_FIRST": "Delete",
        "NAME_LAST": "TestRecord",
        "PHONE_NUMBER": "555-0001",
        "EMAIL_ADDRESS": "delete.test1@example.com"
    }).to_string(),
);
```

## Deletion Workflow

### 1. Pre-Deletion Setup
```rust
// Load test records that will be deleted
for ((data_source_code, record_id), record_definition) in &test_records {
    engine.add_record(data_source_code, record_id, record_definition, None)?;
    println!("Loaded record {} from {}", record_id, data_source_code);
}
```

### 2. Safe Deletion
```rust
// Delete each record with error handling
for ((data_source_code, record_id), _) in &test_records {
    println!("Deleting record {} from {}...", record_id, data_source_code);

    engine.delete_record(data_source_code, record_id, None)?;
    println!("  ✓ Successfully deleted record {}", record_id);
}
```

### 3. Verification
```rust
// Verify each deletion was successful
fn verify_record_deletion(
    engine: &Box<dyn SzEngine>,
    data_source_code: &str,
    record_id: &str,
) -> SzResult<()> {
    match engine.get_record(data_source_code, record_id, None) {
        Ok(_) => eprintln!("⚠ Record still exists!"),
        Err(e) if e.to_string().contains("not found") => {
            println!("✓ Verified deletion");
        },
        Err(e) => eprintln!("⚠ Verification error: {}", e),
    }
    Ok(())
}
```

## Test Data Sources

The examples use dedicated test data sources:
- `DELETE_TEST` - Specifically for deletion testing
- Records with predictable naming: `DEL_001`, `DEL_002`, etc.
- Complete test records with multiple identifying features

## Error Handling Scenarios

### Common Deletion Errors
1. **Record Not Found**: Attempting to delete non-existent records
2. **Data Source Invalid**: Using unregistered data sources
3. **Database Connectivity**: Network or database issues
4. **Concurrent Modifications**: Race conditions during deletion

### Error Recovery Patterns
```rust
match engine.delete_record(data_source, record_id, None) {
    Ok(()) => {
        // Success path
        SUCCESS_COUNT.fetch_add(1, Ordering::Relaxed);
    },
    Err(e) => {
        // Error handling
        ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
        eprintln!("Deletion failed for {}: {}", record_id, e);

        // Optional: Add to retry list
        failed_deletions.push((data_source.clone(), record_id.clone()));
    }
}
```

## Verification Strategies

### Immediate Verification
- Verify deletion immediately after each delete operation
- Catch partial failure scenarios quickly
- Provide immediate feedback on operation success

### Batch Verification
- Verify all deletions after batch operations complete
- More efficient for large deletion operations
- Suitable for non-critical verification scenarios

### Comprehensive Verification
```rust
fn comprehensive_verification(
    engine: &Box<dyn SzEngine>,
    deleted_records: &[(String, String)],
) -> SzResult<()> {
    let mut verification_failures = Vec::new();

    for (data_source, record_id) in deleted_records {
        match engine.get_record(data_source, record_id, None) {
            Ok(_) => {
                verification_failures.push((data_source.clone(), record_id.clone()));
            },
            Err(_) => {
                // Expected: record should not exist
            }
        }
    }

    if !verification_failures.is_empty() {
        eprintln!("Verification failed for {} records", verification_failures.len());
        for (ds, id) in verification_failures {
            eprintln!("  - {}: {}", ds, id);
        }
    }

    Ok(())
}
```

## Best Practices

### Safety Measures
1. **Test with Safe Data**: Use dedicated test data sources
2. **Verify Before Production**: Test deletion workflows thoroughly
3. **Backup Critical Data**: Ensure backups before mass deletions
4. **Use Transactions**: Where supported, use transactional operations

### Performance Considerations
1. **Batch Operations**: Group deletions for efficiency
2. **Error Handling**: Handle errors gracefully without stopping entire batch
3. **Progress Reporting**: Provide feedback for long-running operations
4. **Resource Management**: Clean up resources properly

### Monitoring and Logging
1. **Track Success/Failure Rates**: Monitor deletion operation statistics
2. **Log Detailed Errors**: Capture sufficient context for debugging
3. **Audit Trail**: Maintain records of deletion operations
4. **Performance Metrics**: Track deletion operation timing

## Production Considerations

### Pre-Deletion Validation
- Verify record ownership and permissions
- Check for dependent relationships
- Validate deletion authorization

### Deletion Coordination
- Coordinate with other systems using the data
- Handle concurrent access scenarios
- Implement proper locking mechanisms

### Recovery Procedures
- Maintain deletion audit logs
- Implement rollback procedures where possible
- Plan for partial failure recovery scenarios
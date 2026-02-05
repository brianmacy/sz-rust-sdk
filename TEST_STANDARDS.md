# Test Standards and Requirements

## Mandatory Test Failure Conditions

### 1. **No Silent Failures**

- If any core functionality fails, the test MUST return `Err()`
- Never use `println!("❌ Failed")` followed by `Ok(())`
- Every `❌` message must be paired with error propagation

### 2. **Verification Requirements**

- Every "registration" or "creation" operation must be followed by verification
- If verification fails, the entire test fails
- No partial success acceptance

### 3. **Error Handling Patterns**

```rust
// ❌ WRONG - Silent failure
match some_operation() {
    Ok(_) => println!("✅ Success"),
    Err(e) => println!("❌ Failed: {}", e), // Test continues!
}
Ok(()) // Returns success despite failures

// ✅ CORRECT - Fail fast
let mut failures = Vec::new();
match some_operation() {
    Ok(_) => println!("✅ Success"),
    Err(e) => {
        println!("❌ Failed: {}", e);
        failures.push(format!("Operation failed: {}", e));
    }
}

if !failures.is_empty() {
    return Err(SzError::configuration(format!("Test failures: {:?}", failures)));
}
```

### 4. **Data Source Registration Test Pattern**

```rust
// Step 1: Register
let result = config.register_data_source(data_source)?;

// Step 2: Persist
let config_id = config_mgr.register_config(&config.export()?, None)?;

// Step 3: Verify persistence
let retrieved_config = config_mgr.create_config_from_id(config_id)?;
let registry = retrieved_config.get_data_source_registry()?;
if !registry.contains(data_source) {
    return Err(SzError::configuration(format!("Data source {} not found in registry", data_source)));
}

// Step 4: Verify engine functionality
let test_record = format!(r#"{{% raw %}}{{"DATA_SOURCE": "{}", "RECORD_ID": "TEST"}}{{% endraw %}}"#, data_source);
engine.add_record(data_source, "TEST", &test_record, None)?; // Must succeed or fail test
```

### 5. **Mandatory Verification Steps**

For any "register" or "create" operation:

1. ✅ Operation returns success
2. ✅ Data is retrievable immediately
3. ✅ Data persists after save/reload cycle
4. ✅ Data is functionally usable by engine
5. ❌ If ANY step fails → entire test fails

### 6. **Test Naming Convention**

- `test_*` - Unit tests that MUST fail on any error
- `demo_*` - Demonstrations that may show partial failures but document them
- `example_*` - Working examples that MUST succeed completely

### 7. **Integration Test Requirements**

Every integration test must verify the complete workflow:

- Setup → Operation → Persistence → Retrieval → Functional Use → Cleanup

### 8. **Error Message Standards**

- Always include context: "Failed to register data source 'SAMPLE' in configuration ID 123"
- Include expected vs actual: "Expected data source in registry, but registry contains: [list]"
- Provide actionable information: "Check that data source metadata is complete"

## Implementation Checklist

- [ ] Review all existing tests for silent failures
- [ ] Add failure tracking to multi-step operations
- [ ] Implement strict verification after every operation
- [ ] Add integration test coverage for complete workflows
- [ ] Document expected failure modes
- [ ] Set up CI to catch regression in test quality

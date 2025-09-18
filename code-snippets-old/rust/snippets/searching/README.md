# Searching Examples

This directory contains examples demonstrating entity search capabilities and analysis of search results using the Senzing SDK.

## Examples

### [search_records](search_records/)
**Purpose**: Demonstrates basic and advanced entity searching with result processing and analysis.

**What it demonstrates**:
- Basic attribute-based entity searching
- Search result parsing and interpretation
- Detailed entity information retrieval
- Feature analysis and display
- Match score interpretation
- Multiple search strategies

**Key API calls**:
- `engine.search_by_attributes(attributes, None, None)` - Basic search
- `engine.search_by_attributes(attributes, None, Some(flags))` - Advanced search with flags
- JSON result parsing with `serde_json`

**Search Types**:
1. **Basic Search**: Simple attribute matching
2. **Detailed Search**: Includes entity records and features

**Usage**:
```bash
cd search_records
cargo run
```

### [why_search](why_search/)
**Purpose**: Advanced analysis explaining why entities were returned in search results, providing detailed scoring and matching logic.

**What it demonstrates**:
- Search result explanation and analysis
- Entity matching score breakdown
- Feature-level scoring details
- Record-level analysis within entities
- Relationship disclosure information
- Comprehensive "why" analysis workflow

**Key API calls**:
- `engine.search_by_attributes()` - Initial search
- `engine.why_search()` - Detailed analysis of why entities matched
- `engine.get_entity()` - Detailed entity information

**Analysis Components**:
1. **Match Information**: Overall scores and feature contributions
2. **Entity Structure**: Records and relationships within entities
3. **Feature Analysis**: Individual feature scoring
4. **Record Details**: Record-level matching information

**Usage**:
```bash
cd why_search
cargo run
```

## Common Patterns

### Basic Search
```rust
let search_attributes = r#"{
    "NAME_FIRST": "John",
    "NAME_LAST": "Smith"
}"#;

let results = engine.search_by_attributes(
    search_attributes,
    None,           // No search profile
    None,           // No flags
)?;
```

### Advanced Search with Flags
```rust
let results = engine.search_by_attributes(
    search_attributes,
    None,
    Some(SzFlags::SEARCH_BY_ATTRIBUTES_ALL | SzFlags::ENTITY_INCLUDE_RECORD_DATA),
)?;
```

### Result Processing
```rust
let results: Value = serde_json::from_str(&search_results)?;

if let Some(entities) = results.get("RESOLVED_ENTITIES").and_then(|v| v.as_array()) {
    for entity in entities {
        if let Some(entity_id) = entity.get("ENTITY_ID").and_then(|v| v.as_i64()) {
            println!("Entity ID: {}", entity_id);
        }
    }
}
```

### Why Analysis
```rust
let why_results = engine.why_search(
    search_attributes,
    entity_id,
    None,
    Some(SzFlags::WHY_ENTITIES_DEFAULT_FLAGS),
)?;

let why_info: Value = serde_json::from_str(&why_results)?;
// Process why information...
```

## Search Flags

### Search Behavior Flags
- `SzFlags::SEARCH_BY_ATTRIBUTES_ALL` - Include all matching types
- `SzFlags::SEARCH_BY_ATTRIBUTES_STRONG` - Strong matches only
- `SzFlags::SEARCH_BY_ATTRIBUTES_MINIMAL_STRONG` - Minimal strong matches
- `SzFlags::SEARCH_BY_ATTRIBUTES_MINIMAL_ALL` - Minimal matches of all types

### Entity Detail Flags
- `SzFlags::ENTITY_INCLUDE_RECORD_DATA` - Include record details
- `SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY` - Include record summaries
- `SzFlags::ENTITY_INCLUDE_ALL_FEATURES` - Include all feature information
- `SzFlags::ENTITY_INCLUDE_REPRESENTATIVE_FEATURES` - Include key features only

### Why Analysis Flags
- `SzFlags::WHY_ENTITIES_DEFAULT_FLAGS` - Standard why analysis
- `SzFlags::WHY_ENTITY_INCLUDE_ENTITY_NAME` - Include entity names
- `SzFlags::WHY_ENTITY_INCLUDE_RECORD_SUMMARY` - Include record summaries
- `SzFlags::WHY_ENTITY_INCLUDE_ALL_FEATURES` - Include detailed features

## Search Attributes

### Common Search Patterns
```rust
// Name-based search
let name_search = r#"{
    "NAME_FIRST": "John",
    "NAME_LAST": "Smith"
}"#;

// Contact-based search
let contact_search = r#"{
    "EMAIL_ADDRESS": "john.smith@example.com"
}"#;

// Phone-based search
let phone_search = r#"{
    "PHONE_NUMBER": "555-1234"
}"#;

// Address-based search
let address_search = r#"{
    "ADDR_FULL": "123 Main Street, Anytown, TX 12345"
}"#;

// Multi-attribute search
let combined_search = r#"{
    "NAME_FIRST": "John",
    "NAME_LAST": "Smith",
    "PHONE_NUMBER": "555-1234"
}"#;
```

## Result Structure

### Search Results
```rust
{
    "RESOLVED_ENTITIES": [
        {
            "ENTITY_ID": 1,
            "ENTITY_NAME": "John Smith",
            "MATCH_SCORE": 95,
            "RECORDS": [...],
            "FEATURES": {...}
        }
    ]
}
```

### Why Results
```rust
{
    "WHY_RESULTS": [
        {
            "ENTITY": {...},
            "MATCH_INFO": {
                "MATCH_SCORE": 95,
                "FEATURE_SCORES": {
                    "NAME": {"SCORE": 90, "CANDIDATE_FEATURES": [...]},
                    "PHONE": {"SCORE": 100, "CANDIDATE_FEATURES": [...]}
                }
            }
        }
    ]
}
```

## Analysis Components

### Match Scores
- **Overall Score**: Combined entity matching confidence
- **Feature Scores**: Individual attribute matching scores
- **Threshold Interpretation**: Understanding score significance

### Entity Features
- **NAME**: Name variations and standardizations
- **PHONE**: Phone number normalizations
- **ADDRESS**: Address parsing and standardization
- **EMAIL**: Email address variations

### Relationships
- **Disclosed**: Known relationships between entities
- **Possibly Same**: High probability same entity
- **Possibly Related**: Likely related entities
- **Name Only**: Name-based relationships

## Performance Considerations

### Search Optimization
- Use specific search flags for targeted results
- Limit result sets with appropriate thresholds
- Consider search profile usage for performance

### Result Processing
- Parse JSON results efficiently
- Process large result sets in batches
- Cache frequently accessed entity information

## Use Cases

### Identity Resolution
- Customer deduplication
- Employee matching
- Vendor consolidation

### Investigation
- Entity relationship analysis
- Data quality assessment
- Matching confidence evaluation

### Data Analysis
- Feature effectiveness analysis
- Search performance tuning
- Entity relationship mapping
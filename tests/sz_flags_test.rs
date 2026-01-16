//! Senzing Flags Test
//!
//! This module tests bitflag operations and metadata consistency throughout the SDK,
//! mirroring the C# SzFlagsTest.cs test patterns.

use serial_test::serial;
use sz_rust_sdk::prelude::*;

/// Test basic flag constants
/// Mirrors C# TestFlagsConstant()
#[test]
#[serial]
fn test_flags_constant_entity_include_entity_name() {
    let flag = SzFlags::ENTITY_INCLUDE_ENTITY_NAME;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_ENTITY_NAME"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_record_summary() {
    let flag = SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RECORD_SUMMARY"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_record_data() {
    let flag = SzFlags::ENTITY_INCLUDE_RECORD_DATA;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RECORD_DATA"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_record_matching_info() {
    let flag = SzFlags::ENTITY_INCLUDE_RECORD_MATCHING_INFO;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RECORD_MATCHING_INFO"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_related_entity_name() {
    let flag = SzFlags::ENTITY_INCLUDE_RELATED_ENTITY_NAME;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RELATED_ENTITY_NAME"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_related_matching_info() {
    let flag = SzFlags::ENTITY_INCLUDE_RELATED_MATCHING_INFO;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RELATED_MATCHING_INFO"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_related_record_summary() {
    let flag = SzFlags::ENTITY_INCLUDE_RELATED_RECORD_SUMMARY;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RELATED_RECORD_SUMMARY"));
}

#[test]
#[serial]
fn test_flags_constant_entity_include_related_record_data() {
    let flag = SzFlags::ENTITY_INCLUDE_RELATED_RECORD_DATA;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("ENTITY_INCLUDE_RELATED_RECORD_DATA"));
}

#[test]
#[serial]
fn test_flags_constant_export_include_multi_record_entities() {
    let flag = SzFlags::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("EXPORT_INCLUDE_MULTI_RECORD_ENTITIES"));
}

#[test]
#[serial]
fn test_flags_constant_export_include_possibly_same() {
    let flag = SzFlags::EXPORT_INCLUDE_POSSIBLY_SAME;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("EXPORT_INCLUDE_POSSIBLY_SAME"));
}

#[test]
#[serial]
fn test_flags_constant_export_include_possibly_related() {
    let flag = SzFlags::EXPORT_INCLUDE_POSSIBLY_RELATED;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("EXPORT_INCLUDE_POSSIBLY_RELATED"));
}

#[test]
#[serial]
fn test_flags_constant_export_include_name_only() {
    let flag = SzFlags::EXPORT_INCLUDE_NAME_ONLY;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("EXPORT_INCLUDE_NAME_ONLY"));
}

#[test]
#[serial]
fn test_flags_constant_export_include_disclosed() {
    let flag = SzFlags::EXPORT_INCLUDE_DISCLOSED;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation
    let flag_string = format!("{:?}", flag);
    assert!(flag_string.contains("EXPORT_INCLUDE_DISCLOSED"));
}

#[test]
#[serial]
fn test_flags_constant_find_path_include_matching_info() {
    let flag = SzFlags::FIND_PATH_INCLUDE_MATCHING_INFO;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation (may be composite)
    let flag_string = format!("{:?}", flag);
    assert!(!flag_string.is_empty());
}

#[test]
#[serial]
fn test_flags_constant_find_network_include_matching_info() {
    let flag = SzFlags::FIND_NETWORK_INCLUDE_MATCHING_INFO;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation (may be composite)
    let flag_string = format!("{:?}", flag);
    assert!(!flag_string.is_empty());
}

#[test]
#[serial]
fn test_flags_constant_why_entities_default_flags() {
    let flag = SzFlags::WHY_ENTITIES_DEFAULT_FLAGS;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags (per C# SDK)
    assert!(flag.contains(SzFlags::INCLUDE_FEATURE_SCORES));
}

#[test]
#[serial]
fn test_flags_constant_search_by_attributes_all() {
    let flag = SzFlags::SEARCH_BY_ATTRIBUTES_ALL;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags
    assert!(flag.contains(SzFlags::SEARCH_INCLUDE_RESOLVED));
}

#[test]
#[serial]
fn test_flags_constant_search_by_attributes_strong() {
    let flag = SzFlags::SEARCH_BY_ATTRIBUTES_STRONG;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags
    assert!(flag.contains(SzFlags::SEARCH_INCLUDE_RESOLVED));
}

#[test]
#[serial]
fn test_flags_constant_search_by_attributes_minimal_all() {
    let flag = SzFlags::SEARCH_BY_ATTRIBUTES_MINIMAL_ALL;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags
    assert!(flag.contains(SzFlags::SEARCH_INCLUDE_RESOLVED));
}

#[test]
#[serial]
fn test_flags_constant_search_by_attributes_minimal_strong() {
    let flag = SzFlags::SEARCH_BY_ATTRIBUTES_MINIMAL_STRONG;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags
    assert!(flag.contains(SzFlags::SEARCH_INCLUDE_RESOLVED));
}

#[test]
#[serial]
fn test_flags_constant_search_by_attributes_default_flags() {
    let flag = SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation (may be composite)
    let flag_string = format!("{:?}", flag);
    assert!(!flag_string.is_empty());
}

#[test]
#[serial]
fn test_flags_constant_virtual_entity_default_flags() {
    let flag = SzFlags::VIRTUAL_ENTITY_DEFAULT_FLAGS;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test string representation (may be composite)
    let flag_string = format!("{:?}", flag);
    assert!(!flag_string.is_empty());
}

#[test]
#[serial]
fn test_flags_constant_entity_default_flags() {
    let flag = SzFlags::ENTITY_DEFAULT_FLAGS;

    // Test that the flag has a non-zero value
    assert_ne!(flag.bits(), 0);

    // Test that it contains the expected underlying flags
    assert!(flag.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(flag.contains(SzFlags::ENTITY_INCLUDE_RECORD_DATA));
}

/// Test flag combinations using bitwise OR
/// Mirrors C# TestEnumFlag() and flag combination tests
#[test]
#[serial]
fn test_flag_combinations() {
    let combined = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY;

    // Test that both flags are present
    assert!(combined.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(combined.contains(SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY));

    // Test that the combination has a different value than individual flags
    assert_ne!(combined.bits(), SzFlags::ENTITY_INCLUDE_ENTITY_NAME.bits());
    assert_ne!(
        combined.bits(),
        SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY.bits()
    );
}

#[test]
#[serial]
fn test_flag_intersection() {
    let flag1 = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY;
    let flag2 = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_DATA;

    let intersection = flag1 & flag2;

    // Should only contain the common flag
    assert!(intersection.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(!intersection.contains(SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY));
    assert!(!intersection.contains(SzFlags::ENTITY_INCLUDE_RECORD_DATA));
}

#[test]
#[serial]
fn test_flag_difference() {
    let combined = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY;
    let difference = combined - SzFlags::ENTITY_INCLUDE_ENTITY_NAME;

    // Should only contain the remaining flag
    assert!(!difference.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(difference.contains(SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY));
}

/// Test empty flags
/// Mirrors C# TestZeroToString()
#[test]
#[serial]
fn test_empty_flags() {
    let empty = SzFlags::empty();

    assert_eq!(empty.bits(), 0);
    assert!(empty.is_empty());

    let flag_string = format!("{:?}", empty);
    // Different debug formats are acceptable for empty flags
    assert!(
        flag_string.contains("empty")
            || flag_string.contains("(empty)")
            || flag_string.contains("0x0")
    );
}

/// Test flags to long conversion
/// Mirrors C# TestFlagsToLong()
#[test]
#[serial]
fn test_flags_to_long() {
    let flag = SzFlags::ENTITY_INCLUDE_ENTITY_NAME;
    let bits = flag.bits();

    // Test that we can convert back and forth
    let reconstructed = SzFlags::from_bits_truncate(bits);
    assert_eq!(flag, reconstructed);
}

#[test]
#[serial]
fn test_flags_to_long_combined() {
    let combined = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY;
    let bits = combined.bits();

    // Test that we can convert back and forth
    let reconstructed = SzFlags::from_bits_truncate(bits);
    assert_eq!(combined, reconstructed);
}

/// Test null flags handling
/// Mirrors C# TestNullFlagsToLong()
#[test]
#[serial]
fn test_null_flags_to_long() {
    let empty = SzFlags::empty();
    assert_eq!(empty.bits(), 0);
}

/// Test default flag values
/// Tests commonly used default flag combinations
#[test]
#[serial]
fn test_default_flag_values() {
    // Test that default flags are non-empty
    assert!(!SzFlags::ENTITY_DEFAULT_FLAGS.is_empty());
    assert!(!SzFlags::VIRTUAL_ENTITY_DEFAULT_FLAGS.is_empty());
    assert!(!SzFlags::WHY_ENTITIES_DEFAULT_FLAGS.is_empty());
    assert!(!SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS.is_empty());
    assert!(!SzFlags::FIND_PATH_DEFAULT_FLAGS.is_empty());
    assert!(!SzFlags::FIND_NETWORK_DEFAULT_FLAGS.is_empty());
}

/// Test flag group consistency
/// Test that related flags work together properly
#[test]
#[serial]
fn test_flag_group_consistency() {
    // Test search flags
    let search_flags = SzFlags::SEARCH_BY_ATTRIBUTES_ALL | SzFlags::SEARCH_BY_ATTRIBUTES_STRONG;
    assert!(search_flags.contains(SzFlags::SEARCH_BY_ATTRIBUTES_ALL));
    assert!(search_flags.contains(SzFlags::SEARCH_BY_ATTRIBUTES_STRONG));

    // Test entity flags
    let entity_flags = SzFlags::ENTITY_INCLUDE_ENTITY_NAME | SzFlags::ENTITY_INCLUDE_RECORD_DATA;
    assert!(entity_flags.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(entity_flags.contains(SzFlags::ENTITY_INCLUDE_RECORD_DATA));

    // Test export flags
    let export_flags =
        SzFlags::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES | SzFlags::EXPORT_INCLUDE_POSSIBLY_SAME;
    assert!(export_flags.contains(SzFlags::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES));
    assert!(export_flags.contains(SzFlags::EXPORT_INCLUDE_POSSIBLY_SAME));
}

/// Test flag containment
/// Mirrors C# TestGetFlags() and related containment tests
#[test]
#[serial]
fn test_flag_containment() {
    let combined = SzFlags::ENTITY_INCLUDE_ENTITY_NAME
        | SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY
        | SzFlags::ENTITY_INCLUDE_RECORD_DATA;

    // Test individual flag containment
    assert!(combined.contains(SzFlags::ENTITY_INCLUDE_ENTITY_NAME));
    assert!(combined.contains(SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY));
    assert!(combined.contains(SzFlags::ENTITY_INCLUDE_RECORD_DATA));

    // Test that it doesn't contain unset flags
    assert!(!combined.contains(SzFlags::ENTITY_INCLUDE_RECORD_MATCHING_INFO));
    assert!(!combined.contains(SzFlags::EXPORT_INCLUDE_MULTI_RECORD_ENTITIES));
}

/// Test default flag composition
/// Test that default flags contain expected individual flags
#[test]
#[serial]
fn test_default_flag_composition() {
    // ENTITY_DEFAULT_FLAGS should contain record summary, data, and matching info
    assert!(SzFlags::ENTITY_DEFAULT_FLAGS.contains(SzFlags::ENTITY_INCLUDE_RECORD_SUMMARY));
    assert!(SzFlags::ENTITY_DEFAULT_FLAGS.contains(SzFlags::ENTITY_INCLUDE_RECORD_DATA));
    assert!(SzFlags::ENTITY_DEFAULT_FLAGS.contains(SzFlags::ENTITY_INCLUDE_RECORD_MATCHING_INFO));

    // WHY_ENTITIES_DEFAULT_FLAGS should contain feature scores (per C# SDK)
    assert!(SzFlags::WHY_ENTITIES_DEFAULT_FLAGS.contains(SzFlags::INCLUDE_FEATURE_SCORES));

    // SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS should equal SEARCH_BY_ATTRIBUTES_ALL
    assert_eq!(
        SzFlags::SEARCH_BY_ATTRIBUTES_DEFAULT_FLAGS.bits(),
        SzFlags::SEARCH_BY_ATTRIBUTES_ALL.bits()
    );
}

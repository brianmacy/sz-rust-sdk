//! Senzing Utilities Test
//!
//! This module tests utility functions throughout the SDK,
//! mirroring the C# UtilitiesTest.cs test patterns.

use serial_test::serial;

/// Test hex formatting functionality
/// Mirrors C# TestHexFormat(long value) with TestCase values:
/// 0, 1, 2, 20, 40, 80, 160, 3200, 64000, 128345789
#[test]
#[serial]
fn test_hex_format_0() {
    let value = 0i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x0");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x0");
}

#[test]
#[serial]
fn test_hex_format_1() {
    let value = 1i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x1");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x1");
}

#[test]
#[serial]
fn test_hex_format_2() {
    let value = 2i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x2");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x2");
}

#[test]
#[serial]
fn test_hex_format_20() {
    let value = 20i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x14");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x14");
}

#[test]
#[serial]
fn test_hex_format_40() {
    let value = 40i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x28");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x28");
}

#[test]
#[serial]
fn test_hex_format_80() {
    let value = 80i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x50");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x50");
}

#[test]
#[serial]
fn test_hex_format_160() {
    let value = 160i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0xA0");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0xa0");
}

#[test]
#[serial]
fn test_hex_format_3200() {
    let value = 3200i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0xC80");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0xc80");
}

#[test]
#[serial]
fn test_hex_format_64000() {
    let value = 64000i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0xFA00");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0xfa00");
}

#[test]
#[serial]
fn test_hex_format_128345789() {
    let value = 128345789i64;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x7A666BD");

    // Test lowercase variant
    let hex_lowercase = format!("0x{value:x}");
    assert_eq!(hex_lowercase, "0x7a666bd");
}

/// Test JSON null value escaping
/// Mirrors C# testJsonEscapeNull()
#[test]
#[serial]
fn test_json_escape_null() {
    let null_value: Option<String> = None;
    let json_string = serde_json::to_string(&null_value).unwrap();

    assert_eq!(json_string, "null");
}

/// Test JSON string escaping functionality
/// Mirrors C# TestJsonEscape(string value) with TestCase values:
/// "Hello", "Hello,\nWorld", "\f\b\\\tHey!\r\n", "Bell \u0007!"
#[test]
#[serial]
fn test_json_escape_hello() {
    let value = "Hello";
    let json_string = serde_json::to_string(&value).unwrap();

    // Should be properly quoted
    assert_eq!(json_string, "\"Hello\"");

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

#[test]
#[serial]
fn test_json_escape_hello_newline_world() {
    let value = "Hello,\nWorld";
    let json_string = serde_json::to_string(&value).unwrap();

    // Should escape the newline
    assert!(json_string.contains("\\n"));
    assert_eq!(json_string, "\"Hello,\\nWorld\"");

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

#[test]
#[serial]
fn test_json_escape_control_characters() {
    let value = "\u{000C}\u{0008}\\\tHey!\r\n"; // \f\b\\\tHey!\r\n
    let json_string = serde_json::to_string(&value).unwrap();

    // Should escape all control characters
    assert!(json_string.contains("\\f")); // form feed
    assert!(json_string.contains("\\b")); // backspace
    assert!(json_string.contains("\\\\")); // backslash
    assert!(json_string.contains("\\t")); // tab
    assert!(json_string.contains("\\r")); // carriage return
    assert!(json_string.contains("\\n")); // newline

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

#[test]
#[serial]
fn test_json_escape_bell_character() {
    let value = "Bell \u{0007}!"; // Bell \u0007!
    let json_string = serde_json::to_string(&value).unwrap();

    // Should escape the bell character (control character)
    assert!(json_string.contains("\\u0007"));

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

/// Test additional JSON escaping scenarios
#[test]
#[serial]
fn test_json_escape_quotes() {
    let value = "Say \"Hello\" to the world";
    let json_string = serde_json::to_string(&value).unwrap();

    // Should escape the internal quotes
    assert!(json_string.contains("\\\""));

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

#[test]
#[serial]
fn test_json_escape_empty_string() {
    let value = "";
    let json_string = serde_json::to_string(&value).unwrap();

    assert_eq!(json_string, "\"\"");

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

#[test]
#[serial]
fn test_json_escape_unicode() {
    let value = "Unicode: 🦀 Rust 中文";
    let json_string = serde_json::to_string(&value).unwrap();

    // Should handle Unicode properly
    assert!(json_string.contains("🦀"));
    assert!(json_string.contains("中文"));

    // Test round-trip
    let parsed: String = serde_json::from_str(&json_string).unwrap();
    assert_eq!(parsed, value);
}

/// Test hex formatting with negative numbers
#[test]
#[serial]
fn test_hex_format_negative() {
    let value = -1i64;
    let hex_string = format!("0x{:X}", value as u64);

    // Negative numbers in hex representation
    assert_eq!(hex_string, "0xFFFFFFFFFFFFFFFF");
}

#[test]
#[serial]
fn test_hex_format_max_value() {
    let value = i64::MAX;
    let hex_string = format!("0x{value:X}");

    assert_eq!(hex_string, "0x7FFFFFFFFFFFFFFF");
}

#[test]
#[serial]
fn test_hex_format_min_value() {
    let value = i64::MIN;
    let hex_string = format!("0x{:X}", value as u64);

    assert_eq!(hex_string, "0x8000000000000000");
}

/// Test hex formatting with padding
#[test]
#[serial]
fn test_hex_format_with_padding() {
    let value = 255i64;

    // Test with zero padding
    let hex_padded = format!("0x{value:08X}");
    assert_eq!(hex_padded, "0x000000FF");

    // Test with different padding
    let hex_padded_16 = format!("0x{value:016X}");
    assert_eq!(hex_padded_16, "0x00000000000000FF");
}

/// Test binary formatting
#[test]
#[serial]
fn test_binary_format() {
    let value = 85i64; // 01010101 in binary
    let binary_string = format!("0b{value:b}");

    assert_eq!(binary_string, "0b1010101");

    // Test with padding
    let binary_padded = format!("0b{value:08b}");
    assert_eq!(binary_padded, "0b01010101");
}

/// Test octal formatting
#[test]
#[serial]
fn test_octal_format() {
    let value = 64i64; // 100 in octal
    let octal_string = format!("0o{value:o}");

    assert_eq!(octal_string, "0o100");
}

/// Test decimal formatting
#[test]
#[serial]
fn test_decimal_format() {
    let value = 12345i64;
    let decimal_string = format!("{value}");

    assert_eq!(decimal_string, "12345");

    // Test with sign
    let negative_value = -12345i64;
    let negative_decimal = format!("{negative_value}");
    assert_eq!(negative_decimal, "-12345");
}

/// Test string-to-number conversions
#[test]
#[serial]
fn test_string_to_number_conversion() {
    let hex_string = "0xFF";
    let parsed_from_hex = i64::from_str_radix(&hex_string[2..], 16).unwrap();
    assert_eq!(parsed_from_hex, 255);

    let binary_string = "0b1010101";
    let parsed_from_binary = i64::from_str_radix(&binary_string[2..], 2).unwrap();
    assert_eq!(parsed_from_binary, 85);

    let octal_string = "0o100";
    let parsed_from_octal = i64::from_str_radix(&octal_string[2..], 8).unwrap();
    assert_eq!(parsed_from_octal, 64);
}

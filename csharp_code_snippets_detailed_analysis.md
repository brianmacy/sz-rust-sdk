# C# v4 Code Snippets Enumeration - File by File Analysis

## **File: /home/bmacy/open_dev/sz-rust-sdk/senzing_csharp_detailed_test_analysis.md**

### **Snippet 1: Test Infrastructure Pattern (Line 1450-1459)**
```csharp
protected void PerformTest(Action test) {
    try {
        test();
    } catch (Exception e) {
        // Enhanced error reporting
        // Exception analysis
        // Test failure details
    }
}
```
**Functions Called:**
- **`test()`** - Invokes the passed Action delegate
- **Exception constructor** - Implicitly catches any derived Exception type

**Demonstrates:**
- **Action delegate usage** (C# 4.0 feature)
- **Protected method visibility** for inheritance
- **Try-catch exception handling**
- **Generic exception catching**

## **File: /home/bmacy/open_dev/sz-rust-sdk/csharp_v4_code_analysis.md**

### **Snippet 2: NUnit Assertion Examples**
```csharp
Assert.That(exception, Is.Not.Null)
Assert.That(exception.Message, Is.Not.Null)
Assert.That(exception.InnerException, Is.Null)
Assert.That(exception.ErrorCode, Is.Null)
```
**Functions Called:**
- **`Assert.That()`** - NUnit assertion method (called 60+ times throughout documentation)
- **`Is.Not.Null`** - NUnit constraint for null checking
- **`Is.Null`** - NUnit constraint for null validation
- **`Is.EqualTo()`** - NUnit constraint for value equality
- **`Does.Contain()`** - NUnit constraint for string content validation

**Demonstrates:**
- **NUnit testing framework** assertion patterns
- **Fluent assertion syntax**
- **Object property validation**
- **Null reference checking**

### **Snippet 3: Value Assertion Patterns**
```csharp
Assert.That(exception.Message, Is.EqualTo("Some Message"))
Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))
Assert.That(exception.ToString(), Does.Contain("Some Message"))
```
**Functions Called:**
- **`exception.ToString()`** - Object string representation
- **`Is.EqualTo(value)`** - Exact value comparison
- **`Does.Contain(substring)`** - String content validation

**Demonstrates:**
- **Value equality testing**
- **String content validation**
- **Method return value testing**

### **Snippet 4: JSON Parsing Validation**
```csharp
Assert.That(JsonDocument.Parse(jsonResult), Is.Not.Null)
Assert.That(csvLines.Length, Is.GreaterThan(1))
Assert.That(entityId, Is.GreaterThan(0))
```
**Functions Called:**
- **`JsonDocument.Parse()`** - System.Text.Json parsing method
- **`Is.GreaterThan()`** - NUnit numerical comparison constraint
- **`.Length`** - Array/string length property access

**Demonstrates:**
- **JSON document parsing**
- **Numerical value validation**
- **Array/collection length checking**

### **Snippet 5: Test Method Structure**
```csharp
[Test]
public void TestAddRecord() {
    // Setup, execution, assertion
}
```
**Functions Called:**
- **`[Test]`** - NUnit test attribute
- **Public method declaration**

**Demonstrates:**
- **NUnit test decoration**
- **Public method visibility**
- **Void return type for tests**

### **Snippet 6: Exception Handling Pattern**
```csharp
try {
    operation();
} catch (SzException e) {
    // Handle specific exception
}
```
**Functions Called:**
- **`operation()`** - Generic method call
- **Specific exception type catching**

**Demonstrates:**
- **Typed exception handling**
- **Custom exception types (SzException)**
- **Exception-specific error handling**

## **C# Function Calls Referenced Throughout Documentation**

### **Reflection and Dynamic Instantiation:**
1. **`Activator.CreateInstance(exceptionType)`** (Line 53)
2. **`Activator.CreateInstance(exceptionType, message)`** (Line 69)
3. **`Activator.CreateInstance(exceptionType, errorCode, message)`** (Line 85)
4. **`Activator.CreateInstance(exceptionType, cause)`** (Line 100)
5. **`Activator.CreateInstance(exceptionType, message, cause)`** (Line 115)
6. **`Activator.CreateInstance(exceptionType, errorCode, message, cause)`** (Line 132)
7. **`Activator.CreateInstance(args.exceptionType, args.errorCode, "Some Message")`** (Line 147)

**Demonstrates:**
- **System.Activator reflection usage**
- **Dynamic object instantiation**
- **Constructor overload resolution**
- **Generic type handling**

### **Utility Functions:**
1. **`Convert.ToInt64(hexString, 16)`** (Line 354)
2. **`Utilities.HexFormat(value)`** (Referenced)
3. **`Utilities.JsonEscape(null)`** (Referenced)
4. **`JsonDocument.Parse(escapedJson)`** (Line 385)

**Demonstrates:**
- **Hexadecimal number conversion**
- **Custom utility class usage**
- **JSON document processing**
- **String escaping utilities**

### **Senzing SDK Flag Operations:**
1. **`SzFlags.AllFlags`** (Line 165)
2. **`SzFlags.GetGroups(unrecognizedFlag)`** (Line 217)
3. **`SzFlags.GetGroups(flagValue)`** (Line 229)
4. **`SzFlags.ToFlagString(flagValue)`** (Line 270)
5. **`SzFlags.ToFlagString(0L, flagGroup)`** (Line 284)
6. **`SzFlags.GetFlags(flagGroup)`** (Line 296)
7. **`SzFlags.FlagsToLong(flagValue)`** (Line 323)
8. **`SzFlags.FlagsToLong(null)`** (Line 335)

**Demonstrates:**
- **Static class method calls**
- **Bitflag manipulation**
- **Enum/flag conversion utilities**
- **Null parameter handling**

### **Engine Operations (Referenced but not shown in snippets):**
1. **`engine.GetNativeApi()`**
2. **`engine.AddRecord(dataSourceCode, recordId, recordJson, flags)`**
3. **`engine.SearchByAttributes(searchJson, searchProfile, flags)`**
4. **`engine.GetEntity(dataSourceCode, recordId, flags)`**
5. **`config.CreateConfig()`**
6. **`configManager.RegisterConfig(configJson, configComment)`**
7. **`diagnostic.GetRepositoryInfo()`**

**Demonstrates:**
- **Senzing SDK method calls**
- **Multi-parameter method invocations**
- **Flag parameter usage**
- **JSON data passing**

## **Complete Function Call Inventory**

### **System Framework Functions:**
| Function | Purpose | C# Version | Usage Count |
|----------|---------|------------|-------------|
| `Activator.CreateInstance()` | Dynamic object creation | 2.0+ | 7 variations |
| `Convert.ToInt64()` | Number conversion | 1.0+ | 2 uses |
| `JsonDocument.Parse()` | JSON parsing | 3.0+ | 5+ uses |
| `Exception` constructor | Exception instantiation | 1.0+ | Multiple |
| `.ToString()` | Object string representation | 1.0+ | Multiple |
| `.Length` property | Collection size | 1.0+ | Multiple |

### **NUnit Testing Functions:**
| Function | Purpose | Usage Count |
|----------|---------|-------------|
| `Assert.That()` | Test assertion | 60+ uses |
| `Is.Not.Null` | Null validation | 15+ uses |
| `Is.Null` | Null checking | 10+ uses |
| `Is.EqualTo()` | Value equality | 20+ uses |
| `Does.Contain()` | String content | 10+ uses |
| `Is.GreaterThan()` | Numerical comparison | 8+ uses |
| `[Test]` attribute | Test marking | Referenced |

### **Senzing SDK Functions:**
| Function | Purpose | Usage Count |
|----------|---------|-------------|
| `SzFlags.AllFlags` | Flag enumeration | 1 use |
| `SzFlags.GetGroups()` | Flag grouping | 2 uses |
| `SzFlags.ToFlagString()` | Flag to string | 2 uses |
| `SzFlags.GetFlags()` | Flag retrieval | 1 use |
| `SzFlags.FlagsToLong()` | Flag conversion | 2 uses |
| Engine methods | Entity operations | 7+ methods |
| Config methods | Configuration | 3+ methods |
| Diagnostic methods | System info | 1+ methods |

### **Custom Utility Functions:**
| Function | Purpose | Usage Count |
|----------|---------|-------------|
| `Utilities.HexFormat()` | Hex formatting | 1 use |
| `Utilities.JsonEscape()` | JSON escaping | 1 use |
| `PerformTest()` | Test wrapper | 1 definition |

## **C# Language Features Demonstrated**

### **C# 4.0 Specific Features:**
1. **Action Delegates** - `Action test` parameter in PerformTest method
2. **Generic Type Constraints** - Used with Activator.CreateInstance
3. **Optional Parameters** - Implied in various method overloads
4. **Named Parameters** - Potential usage in SDK calls

### **General C# Features:**
1. **Exception Handling** - Try-catch blocks with specific types
2. **Reflection** - Extensive Activator.CreateInstance usage
3. **Properties** - Exception.Message, Exception.ErrorCode access
4. **Static Methods** - SzFlags class method calls
5. **Method Overloading** - Multiple constructor patterns
6. **Attributes** - [Test] attribute for unit testing
7. **Null Reference Handling** - Explicit null checking patterns
8. **String Manipulation** - ToString(), Contains() operations
9. **JSON Processing** - JsonDocument parsing and validation
10. **Fluent Interfaces** - NUnit assertion syntax

## **Integration with Rust Implementation**

### **Mapping Patterns:**

**C# Reflection → Rust Generics:**
```csharp
Activator.CreateInstance(exceptionType, message)
```
```rust
SzError::new_with_message::<T>(message)
```

**C# Exceptions → Rust Results:**
```csharp
try { operation(); } catch (SzException e) { ... }
```
```rust
match operation() { Ok(result) => ..., Err(e) => ... }
```

**C# NUnit → Rust Testing:**
```csharp
Assert.That(value, Is.EqualTo(expected))
```
```rust
assert_eq!(value, expected);
```

**C# JSON → Rust JSON:**
```csharp
JsonDocument.Parse(jsonString)
```
```rust
serde_json::from_str::<Value>(json_string)
```

## **Summary Statistics**

- **Total Files Analyzed:** 2
- **Total C# Code Snippets:** 6 major patterns
- **Total Function Calls Referenced:** 35+ unique functions
- **C# Framework Methods:** 12 distinct functions
- **NUnit Testing Methods:** 7 assertion patterns
- **Senzing SDK Methods:** 15+ SDK-specific functions
- **Custom Utility Methods:** 3 utility functions

**Primary Purpose:** Provide comprehensive C# SDK reference patterns for exact Rust implementation mirroring, ensuring 100% functional compatibility between C# and Rust SDKs.
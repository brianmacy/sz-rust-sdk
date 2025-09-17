# C# Code Snippets - Per File Enumeration

## File 1: `/home/bmacy/open_dev/sz-rust-sdk/senzing_csharp_detailed_test_analysis.md`

### **Snippet 1.1: PerformTest Method (Lines 1450-1459)**
**Location:** Line 1450
**Code:**
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
- `test()` - Action delegate invocation
- Exception constructor (implicit catch)

**Demonstrates:**
- C# 4.0 Action delegate parameter
- Protected method visibility
- Generic exception handling
- Try-catch error handling pattern

---

### **Function Call References in senzing_csharp_detailed_test_analysis.md:**

#### **Reflection Functions:**
**Line 53:**
- `Activator.CreateInstance(exceptionType)`

**Line 69:**
- `Activator.CreateInstance(exceptionType, message)`

**Line 85:**
- `Activator.CreateInstance(exceptionType, errorCode, message)`

**Line 100:**
- `Activator.CreateInstance(exceptionType, cause)`

**Line 115:**
- `Activator.CreateInstance(exceptionType, message, cause)`

**Line 132:**
- `Activator.CreateInstance(exceptionType, errorCode, message, cause)`

**Line 147:**
- `Activator.CreateInstance(args.exceptionType, args.errorCode, "Some Message")`
- `exception.ErrorCode` property access

#### **NUnit Assertion Functions:**
**Line 56:**
- `Assert.That(exception, Is.Not.Null)`

**Line 57:**
- `Assert.That(exception.Message, Is.Not.Null)`

**Line 58:**
- `Assert.That(exception.InnerException, Is.Null)`

**Line 59:**
- `Assert.That(exception.ErrorCode, Is.Null)`

**Line 60:**
- `Assert.That(exception.ToString(), Is.Not.Null)`

**Line 72:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`

**Line 73:**
- `Assert.That(exception.InnerException, Is.Null)`

**Line 74:**
- `Assert.That(exception.ErrorCode, Is.Null)`

**Line 75:**
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

**Line 88:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`

**Line 89:**
- `Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))`

**Line 90:**
- `Assert.That(exception.InnerException, Is.Null)`

**Line 91:**
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

**Line 103:**
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`

**Line 104:**
- `Assert.That(exception.ErrorCode, Is.Null)`

**Line 105:**
- `Assert.That(exception.ToString(), Is.Not.Null)`

**Line 118:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`

**Line 119:**
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`

**Line 120:**
- `Assert.That(exception.ErrorCode, Is.Null)`

**Line 121:**
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

**Line 135:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`

**Line 136:**
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`

**Line 137:**
- `Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))`

**Line 138:**
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

**Line 151:**
- `Assert.That(exception.ErrorCode, Is.EqualTo(args.errorCode))`

**Line 358:**
- `Assert.That(Convert.ToInt64(hexResult, 16), Is.EqualTo(value))`

**Line 372:**
- `Assert.That(result, Is.EqualTo("null"))`

**Line 413:**
- `Assert.That(nativeApi, Is.Not.Null)`

**Line 589:**
- `Assert.That(csvLines.Length, Is.GreaterThan(1))`

**Line 590:**
- `Assert.That(csvLines[0], Does.Contain("RESOLVED_ENTITY_ID"))`

**Line 606:**
- `Assert.That(JsonDocument.Parse(jsonResult), Is.Not.Null)`

**Line 626:**
- `Assert.That(JsonDocument.Parse(entityJson), Is.Not.Null)`

**Line 627:**
- `Assert.That(entityId, Is.GreaterThan(0))`

**Line 662:**
- `Assert.That(JsonDocument.Parse(searchResults), Is.Not.Null)`

**Line 721:**
- `Assert.That(response, Is.Not.Null)`

**Line 785:**
- `Assert.That(redoRecord, Is.Not.Null)`

**Line 797:**
- `Assert.That(count, Is.GreaterThanOrEqualTo(0))`

**Line 827:**
- `Assert.That(JsonDocument.Parse(pathResult), Is.Not.Null)`

**Line 847:**
- `Assert.That(networkResult.Entities.Count, Is.LessThanOrEqualTo(maxEntities))`

**Line 876:**
- `Assert.That(JsonDocument.Parse(howResult), Is.Not.Null)`

**Line 906:**
- `Assert.That(JsonDocument.Parse(whyResult), Is.Not.Null)`

**Line 980:**
- `Assert.That(configHandle, Is.GreaterThan(0))`

**Line 1006:**
- `Assert.That(JsonDocument.Parse(exportJson), Is.Not.Null)`

**Line 1049:**
- `Assert.That(registryJson, Does.Contain(dataSourceCode))`

**Line 1068:**
- `Assert.That(configId, Is.GreaterThan(0))`

**Line 1096:**
- `Assert.That(JsonDocument.Parse(registryJson), Is.Not.Null)`

**Line 1135:**
- `Assert.That(defaultId, Is.GreaterThan(0))`

**Line 1161:**
- `Assert.That(JsonDocument.Parse(repoInfo), Is.Not.Null)`

**Line 1177:**
- `Assert.That(perfResult, Does.Contain("insertTime"))`

**Line 1191:**
- `Assert.That(JsonDocument.Parse(featureInfo), Is.Not.Null)`

**Line 1237:**
- `Assert.That(environment, Is.Not.Null)`

**Line 1263:**
- `Assert.That(engine, Is.Not.Null)`

**Line 1276:**
- `Assert.That(config, Is.Not.Null)`

**Line 1289:**
- `Assert.That(configManager, Is.Not.Null)`

**Line 1302:**
- `Assert.That(diagnostic, Is.Not.Null)`

**Line 1315:**
- `Assert.That(product, Is.Not.Null)`

**Line 1355:**
- `Assert.That(JsonDocument.Parse(licenseJson), Is.Not.Null)`

**Line 1356:**
- `Assert.That(licenseJson, Does.Contain("customer"))`

**Line 1357:**
- `Assert.That(licenseJson, Does.Contain("contract"))`

**Line 1358:**
- `Assert.That(licenseJson, Does.Contain("issueDate"))`

**Line 1373:**
- `Assert.That(JsonDocument.Parse(versionJson), Is.Not.Null)`

**Line 1374:**
- `Assert.That(versionJson, Does.Contain("VERSION"))`

**Line 1375:**
- `Assert.That(versionJson, Does.Contain("BUILD_NUMBER"))`

#### **Utility Functions:**
**Line 354:**
- `Convert.ToInt64(hexString, 16)`

**Line 385:**
- `JsonDocument.Parse(escapedJson)`

#### **SzFlags Functions:**
**Line 165:**
- `SzFlags.AllFlags`

**Line 217:**
- `SzFlags.GetGroups(unrecognizedFlag)`

**Line 229:**
- `SzFlags.GetGroups(flagValue)`

**Line 270:**
- `SzFlags.ToFlagString(flagValue)`

**Line 284:**
- `SzFlags.ToFlagString(0L, flagGroup)`

**Line 296:**
- `SzFlags.GetFlags(flagGroup)`

**Line 323:**
- `SzFlags.FlagsToLong(flagValue)`

**Line 335:**
- `SzFlags.FlagsToLong(null)`

#### **Exception Constructor Calls:**
**Line 97:**
- `new Exception("Root Cause")`

**Line 112:**
- `new Exception("Root Cause")`

**Line 129:**
- `new Exception("Root Cause")`

---

## File 2: `/home/bmacy/open_dev/sz-rust-sdk/csharp_v4_code_analysis.md`

### **Snippet 2.1: NUnit Assertion Pattern (Lines 149-154)**
**Location:** Line 149
**Code:**
```csharp
Assert.That(exception, Is.Not.Null)
Assert.That(exception.Message, Is.Not.Null)
Assert.That(exception.InnerException, Is.Null)
Assert.That(exception.ErrorCode, Is.Null)
```

**Functions Called:**
- `Assert.That()` - NUnit assertion framework
- `Is.Not.Null` - NUnit constraint
- `Is.Null` - NUnit constraint

**Demonstrates:**
- NUnit fluent assertion syntax
- Object property validation
- Null reference checking

---

### **Snippet 2.2: Value Verification Assertions (Lines 157-161)**
**Location:** Line 157
**Code:**
```csharp
Assert.That(exception.Message, Is.EqualTo("Some Message"))
Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))
Assert.That(exception.ToString(), Does.Contain("Some Message"))
```

**Functions Called:**
- `Assert.That()` - NUnit assertion
- `Is.EqualTo()` - NUnit equality constraint
- `Does.Contain()` - NUnit content constraint
- `exception.ToString()` - Object string representation

**Demonstrates:**
- Value equality testing
- String content validation
- Method return value testing

---

### **Snippet 2.3: JSON Validation Assertions (Lines 164-168)**
**Location:** Line 164
**Code:**
```csharp
Assert.That(JsonDocument.Parse(jsonResult), Is.Not.Null)
Assert.That(csvLines.Length, Is.GreaterThan(1))
Assert.That(entityId, Is.GreaterThan(0))
```

**Functions Called:**
- `JsonDocument.Parse()` - System.Text.Json parsing
- `Assert.That()` - NUnit assertion
- `Is.Not.Null` - NUnit constraint
- `Is.GreaterThan()` - NUnit numerical constraint
- `.Length` - Array/string property

**Demonstrates:**
- JSON document parsing
- Numerical value validation
- Array/collection length checking

---

### **Snippet 2.4: Test Method Structure (Lines 184-189)**
**Location:** Line 184
**Code:**
```csharp
[Test]
public void TestAddRecord() {
    // Setup, execution, assertion
}
```

**Functions Called:**
- `[Test]` - NUnit test attribute

**Demonstrates:**
- NUnit test decoration
- Public method visibility
- Void return type for tests

---

### **Snippet 2.5: Exception Handling Pattern (Lines 201-207)**
**Location:** Line 201
**Code:**
```csharp
try {
    operation();
} catch (SzException e) {
    // Handle specific exception
}
```

**Functions Called:**
- `operation()` - Generic method call

**Demonstrates:**
- Typed exception handling
- Custom exception types (SzException)
- Exception-specific error handling

---

## Summary Per File:

### **File 1 (`senzing_csharp_detailed_test_analysis.md`):**
- **Total Snippets:** 1 major code block
- **Function Calls:** 60+ Assert.That statements, 7 Activator.CreateInstance variations, 8 SzFlags methods
- **Unique Functions:** 35+ distinct function calls
- **Primary Purpose:** Test infrastructure and assertion patterns

### **File 2 (`csharp_v4_code_analysis.md`):**
- **Total Snippets:** 5 code examples
- **Function Calls:** NUnit assertions, JSON parsing, exception handling
- **Unique Functions:** 8 distinct function calls
- **Primary Purpose:** Pattern examples and Rust mapping templates

### **Grand Total:**
- **Files:** 2
- **Code Snippets:** 6 distinct patterns
- **Function Calls:** 43+ unique functions
- **Lines of C# Code:** ~50 lines across all snippets
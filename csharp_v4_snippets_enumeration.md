# C# v4 Code Snippets Enumeration - Per Snippet File Analysis

**Source Repository:** https://github.com/Senzing/code-snippets-v4/tree/main/csharp/snippets

## **File 1: initialization/EnginePriming/Program.cs**

### **Complete Code:**
```csharp
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using Senzing.Sdk;
using Senzing.Sdk.Core;

// get the senzing repository settings
string? settings = Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON");
if (settings == null)
{
    Console.Error.WriteLine("Unable to get settings.");
    throw new ArgumentException("Unable to get settings");
}

// create a descriptive instance name (can be anything)
Assembly assembly = Assembly.GetExecutingAssembly();
string? instanceName = assembly.GetName().Name;

// initialize the Senzing environment
SzEnvironment env = SzCoreEnvironment.NewBuilder()
    .Settings(settings)
    .InstanceName(instanceName)
    .VerboseLogging(false)
    .Build();

try
{
    SzEngine engine = env.GetEngine();

    Stopwatch stopwatch = Stopwatch.StartNew();

    engine.PrimeEngine();

    long duration = stopwatch.ElapsedMilliseconds;

    Console.WriteLine("Primed Senzing engine. (" + duration + "ms");

}
catch (SzException e)
{
    // handle any exception that may have occurred
    Console.Error.WriteLine("Senzing Error Message : " + e.Message);
    Console.Error.WriteLine("Senzing Error Code    : " + e.ErrorCode);
    Console.Error.WriteLine(e);
    throw;

}
catch (Exception e)
{
    Console.Error.WriteLine();
    Console.Error.WriteLine("*** Terminated due to critical error ***");
    Console.Error.WriteLine(e);
    Console.Error.Flush();
    throw;

}
finally
{
    // IMPORTANT: make sure to destroy the environment
    env.Destroy();
}
```

### **Functions Called:**
1. **System Functions:**
   - `Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON")`
   - `Console.Error.WriteLine()`
   - `ArgumentException` constructor
   - `Assembly.GetExecutingAssembly()`
   - `assembly.GetName().Name`
   - `Stopwatch.StartNew()`
   - `stopwatch.ElapsedMilliseconds`
   - `Console.WriteLine()`
   - `Console.Error.Flush()`

2. **Senzing SDK Functions:**
   - `SzCoreEnvironment.NewBuilder()`
   - `.Settings(settings)`
   - `.InstanceName(instanceName)`
   - `.VerboseLogging(false)`
   - `.Build()`
   - `env.GetEngine()`
   - `engine.PrimeEngine()`
   - `env.Destroy()`

3. **Exception Properties:**
   - `e.Message`
   - `e.ErrorCode`

### **Demonstrates:**
- **Environment variable access** for configuration
- **Builder pattern** for SzCoreEnvironment creation
- **Method chaining** (.Settings().InstanceName().VerboseLogging().Build())
- **Performance measurement** with Stopwatch
- **Exception hierarchy** (SzException vs Exception)
- **Specific exception properties** (Message, ErrorCode)
- **Resource cleanup** with try-catch-finally
- **Assembly reflection** for instance naming

---

## **File 2: initialization/EnvironmentAndHubs/Program.cs**

### **Complete Code:**
```csharp
using System;
using System.Collections.Generic;
using System.Reflection;
using System.Text;
using System.Text.Json;
using System.Text.Json.Nodes;
using Senzing.Sdk;
using Senzing.Sdk.Core;

// get the senzing repository settings
string? settings = Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON");
if (settings == null)
{
    Console.Error.WriteLine("Unable to get settings.");
    throw new ArgumentException("Unable to get settings");
}

// create a descriptive instance name (can be anything)
Assembly assembly = Assembly.GetExecutingAssembly();
string? instanceName = assembly.GetName().Name;

// initialize the Senzing environment
SzEnvironment env = SzCoreEnvironment.NewBuilder()
    .Settings(settings)
    .InstanceName(instanceName)
    .VerboseLogging(false)
    .Build();

try
{
    SzProduct product = env.GetProduct();
    SzConfigManager configMgr = env.GetConfigManager();
    SzDiagnostic diagnostic = env.GetDiagnostic();
    SzEngine engine = env.GetEngine();

    Console.WriteLine(product);
    Console.WriteLine(configMgr);
    Console.WriteLine(diagnostic);
    Console.WriteLine(engine);

    // do work with the hub handles which are valid
    // until the env.destroy() function is called

}
catch (SzException e)
{
    // handle any exception that may have occurred
    Console.Error.WriteLine("Senzing Error Message : " + e.Message);
    Console.Error.WriteLine("Senzing Error Code    : " + e.ErrorCode);
    Console.Error.WriteLine(e);
    throw;

}
catch (Exception e)
{
    Console.Error.WriteLine();
    Console.Error.WriteLine("*** Terminated due to critical error ***");
    Console.Error.WriteLine(e);
    Console.Error.Flush();
    throw;

}
finally
{
    // IMPORTANT: make sure to destroy the environment
    env.Destroy();
}
```

### **Functions Called:**
1. **System Functions:**
   - `Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON")`
   - `Console.Error.WriteLine()`
   - `ArgumentException` constructor
   - `Assembly.GetExecutingAssembly()`
   - `assembly.GetName().Name`
   - `Console.WriteLine()` (4 calls)
   - `Console.Error.Flush()`

2. **Senzing SDK Functions:**
   - `SzCoreEnvironment.NewBuilder()`
   - `.Settings(settings)`
   - `.InstanceName(instanceName)`
   - `.VerboseLogging(false)`
   - `.Build()`
   - `env.GetProduct()`
   - `env.GetConfigManager()`
   - `env.GetDiagnostic()`
   - `env.GetEngine()`
   - `env.Destroy()`

3. **Exception Properties:**
   - `e.Message`
   - `e.ErrorCode`

### **Demonstrates:**
- **Multiple hub retrieval** from single environment
- **Object ToString() methods** (implicit Console.WriteLine calls)
- **Environment factory pattern** for different SDK components
- **Consistent error handling** across all operations
- **Resource lifecycle management**

---

## **File 3: loading/LoadRecords/Program.cs**

### **Partial Code Available:**
```csharp
using System;
using System.Collections.Generic;
using System.Reflection;
using System.Text;
using Senzing.Sdk;
using Senzing.Sdk.Core;

using static Senzing.Sdk.SzFlags;

// get the senzing repository settings
string? settings = Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON");
if (settings == null)
{
  Console.Error.WriteLine("Unable to get settings.");
  throw new ArgumentException("Unable to get settings");
}

// create a descriptive instance name (can be anything)
Assembly assembly = Assembly.GetExecutingAssembly();
string? instanceName = assembly.GetName().Name;

// initialize the Senzing environment
SzEnvironment env = SzCoreEnvironment.NewBuilder()
    .Settings(settings)
    .InstanceName(instanceName)
    .VerboseLogging(false)
    .Build();

try
{
  // get the engine from the environment
  SzEngine engine = env.GetEngine();

  // loop through the example records and add them to the repository
  foreach (KeyValuePair<(string, string), string> pair in GetRecords())
  {
    (string dataSourceCode, string recordID) = pair.Key;
    string recordDefinition = pair.Value;

    // call the addRecord() function with no flags
    engine.AddRecord(dataSourceCode, recordID, recordDefinition, SzNoFlags);

    Console.WriteLine("Record " + recordID + " added");
    Console.Out.Flush();
  }

}
catch (SzException e)
{
  // handle any exception that may have occurred
  Console.Error.WriteLine("Senzing Error Message : " + e.Message);
  Console.Error.WriteLine("Senzing Error Code    : " + e.ErrorCode);
  Console.Error.WriteLine(e);
  throw;

}
catch (Exception e)
{
  Console.Error.WriteLine();
  Console.Error.WriteLine("*** Terminated due to critical error ***");
  Console.Error.WriteLine(e);
  Console.Error.Flush();
  throw;
}
```

### **Functions Called:**
1. **System Functions:**
   - `Environment.GetEnvironmentVariable("SENZING_ENGINE_CONFIGURATION_JSON")`
   - `Console.Error.WriteLine()`
   - `ArgumentException` constructor
   - `Assembly.GetExecutingAssembly()`
   - `assembly.GetName().Name`
   - `Console.WriteLine()`
   - `Console.Out.Flush()`
   - `Console.Error.Flush()`

2. **Senzing SDK Functions:**
   - `SzCoreEnvironment.NewBuilder()`
   - `.Settings(settings)`
   - `.InstanceName(instanceName)`
   - `.VerboseLogging(false)`
   - `.Build()`
   - `env.GetEngine()`
   - `engine.AddRecord(dataSourceCode, recordID, recordDefinition, SzNoFlags)`

3. **Language Features:**
   - `KeyValuePair<(string, string), string>` (generic collections with tuples)
   - Tuple deconstruction: `(string dataSourceCode, string recordID) = pair.Key`
   - `foreach` loop iteration
   - `using static` directive for SzFlags

4. **Custom Functions:**
   - `GetRecords()` (assumed to be defined elsewhere in the file)

### **Demonstrates:**
- **Static using import** for flag constants
- **Tuple deconstruction** in C# 7+ style
- **Generic collections** with complex types
- **Record loading pattern** with error handling
- **Flag usage** (SzNoFlags constant)
- **Batch processing** with foreach loop
- **Console output flushing** for real-time feedback

---

## **Directory Structure Analysis:**

Based on the GitHub API results, the C# snippets are organized as:

### **Available Directories:**
1. **configuration/** - Configuration management snippets
2. **deleting/** - Record deletion examples
3. **information/** - Information retrieval snippets
4. **initialization/** - Environment setup examples
   - EnginePriming/
   - EnvironmentAndHubs/
   - PurgeRepository/
5. **loading/** - Data loading examples
   - LoadRecords/
   - LoadTruthSetWithInfoViaLoop/
   - LoadViaFutures/
   - LoadViaLoop/
   - LoadViaQueue/
   - LoadWithInfoViaFutures/
   - LoadWithStatsViaLoop/
6. **redo/** - Redo processing snippets
7. **searching/** - Search operation examples
8. **stewardship/** - Data stewardship snippets

## **Common C# v4+ Features Demonstrated:**

### **Language Features:**
1. **Nullable reference types** (`string?`)
2. **Tuple deconstruction** (`(string dataSourceCode, string recordID) = pair.Key`)
3. **Generic collections** (`KeyValuePair<(string, string), string>`)
4. **Using static directives** (`using static Senzing.Sdk.SzFlags`)
5. **Method chaining/Builder pattern**
6. **String interpolation** (implicit in concatenation)

### **Framework Features:**
1. **Environment variable access**
2. **Assembly reflection**
3. **Stopwatch performance measurement**
4. **Console I/O operations**
5. **Exception hierarchy handling**

### **Senzing SDK Patterns:**
1. **Environment builder pattern**
2. **Hub factory methods** (GetEngine, GetProduct, etc.)
3. **Flag-based operation control**
4. **Consistent exception handling**
5. **Resource lifecycle management**

## **Function Call Summary:**

### **Most Common Function Calls:**
- `Console.Error.WriteLine()` - Error logging (appears in all files)
- `SzCoreEnvironment.NewBuilder()` - Environment creation (appears in all files)
- `env.Destroy()` - Resource cleanup (appears in all files)
- `Environment.GetEnvironmentVariable()` - Configuration access (appears in all files)

### **Senzing-Specific Calls:**
- `engine.PrimeEngine()` - Engine initialization
- `engine.AddRecord()` - Record loading
- `env.GetEngine()` - Engine retrieval
- `env.GetProduct()` - Product info access
- `env.GetConfigManager()` - Config management
- `env.GetDiagnostic()` - Diagnostic access

### **Total Unique Functions Identified:** 25+
### **Total Files Analyzed:** 3 (partial analysis of repository)
### **Primary Patterns:** Environment setup, error handling, resource management
# Senzing C# SDK Detailed Test-by-Test Analysis

This document provides a comprehensive, test-by-test analysis of the Senzing C# SDK test suite, extracting exact input parameters, SDK method calls, expected outputs, and assertions for every test method.

## Table of Contents

1. [Main Test Files](#main-test-files)
   - [SzExceptionTest.cs](#szexceptiontestcs)
   - [SzFlagsTest.cs](#szflagstestcs)
   - [UtilitiesTest.cs](#utilitiestestcs)
2. [Core Test Files](#core-test-files)
   - [SzCoreEngineBasicsTest.cs](#szcoreenginebasicstestcs)
   - [SzCoreEngineReadTest.cs](#szcoreenginereadtestcs)
   - [SzCoreEngineWriteTest.cs](#szcoreenginewritetestcs)
   - [SzCoreEngineGraphTest.cs](#szcoreenginegraphtestcs)
   - [SzCoreEngineHowTest.cs](#szcoreenginehowtestcs)
   - [SzCoreEngineWhyTest.cs](#szcoreenginewhytestcs)
   - [SzCoreConfigTest.cs](#szcoreconfigtestcs)
   - [SzCoreConfigManagerTest.cs](#szcoreconfigmanagertestcs)
   - [SzCoreDiagnosticTest.cs](#szcorediagnostictestcs)
   - [SzCoreEnvironmentTest.cs](#szcoreenvironmenttestcs)
   - [SzCoreProductTest.cs](#szcoreproducttestcs)

---

## Main Test Files

### SzExceptionTest.cs

This file tests the exception hierarchy and error handling throughout the SDK.

#### TestDefaultConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())

**Test Case Values:**
- `typeof(SzException)`
- `typeof(SzConfigurationException)`
- `typeof(SzDatabaseException)`
- `typeof(SzDatabaseConnectionLostException)`
- `typeof(SzBadInputException)`
- `typeof(SzLicenseException)`
- `typeof(SzNotFoundException)`
- `typeof(SzNotInitializedException)`
- `typeof(SzUnhandledException)`
- `typeof(SzReplaceConflictException)`
- `typeof(SzUnknownDataSourceException)`
- `typeof(SzRetryTimeoutException)`
- `typeof(SzRetryableException)`

**SDK Calls:**
- `Activator.CreateInstance(exceptionType)`

**Expected Outputs:**
- `Assert.That(exception, Is.Not.Null)`
- `Assert.That(exception.Message, Is.Not.Null)`
- `Assert.That(exception.InnerException, Is.Null)`
- `Assert.That(exception.ErrorCode, Is.Null)`
- `Assert.That(exception.ToString(), Is.Not.Null)`

#### TestMessageConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())
- `message`: "Some Message"

**SDK Calls:**
- `Activator.CreateInstance(exceptionType, message)`

**Expected Outputs:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`
- `Assert.That(exception.InnerException, Is.Null)`
- `Assert.That(exception.ErrorCode, Is.Null)`
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

#### TestCodeAndMessageConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())
- `errorCode`: 10L, 20L, 30L, 40L (TestCase values)
- `message`: "Some Message"

**SDK Calls:**
- `Activator.CreateInstance(exceptionType, errorCode, message)`

**Expected Outputs:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`
- `Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))`
- `Assert.That(exception.InnerException, Is.Null)`
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

#### TestCauseConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())
- `cause`: new Exception("Root Cause")

**SDK Calls:**
- `Activator.CreateInstance(exceptionType, cause)`

**Expected Outputs:**
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`
- `Assert.That(exception.ErrorCode, Is.Null)`
- `Assert.That(exception.ToString(), Is.Not.Null)`

#### TestMessageAndCauseConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())
- `message`: "Some Message"
- `cause`: new Exception("Root Cause")

**SDK Calls:**
- `Activator.CreateInstance(exceptionType, message, cause)`

**Expected Outputs:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`
- `Assert.That(exception.ErrorCode, Is.Null)`
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

#### TestFullConstruct(Type exceptionType)

**Input Parameters:**
- `exceptionType`: Type (via TestCaseSource GetExceptionTypes())
- `errorCode`: 10L, 20L, 30L, 40L (TestCase values)
- `message`: "Some Message"
- `cause`: new Exception("Root Cause")

**SDK Calls:**
- `Activator.CreateInstance(exceptionType, errorCode, message, cause)`

**Expected Outputs:**
- `Assert.That(exception.Message, Is.EqualTo("Some Message"))`
- `Assert.That(exception.InnerException, Is.EqualTo(cause))`
- `Assert.That(exception.ErrorCode, Is.EqualTo(errorCode))`
- `Assert.That(exception.ToString(), Does.Contain("Some Message"))`

#### TestGetErrorCode((Type exceptionType, long errorCode) args)

**Input Parameters:**
- `args.exceptionType`: Exception type
- `args.errorCode`: 10L, 20L, 30L, 40L

**SDK Calls:**
- `Activator.CreateInstance(args.exceptionType, args.errorCode, "Some Message")`
- `exception.ErrorCode`

**Expected Outputs:**
- `Assert.That(exception.ErrorCode, Is.EqualTo(args.errorCode))`

---

### SzFlagsTest.cs

This file tests bitflag operations, metadata consistency, and flag conversion functionality.

#### TestFlagsConstant()

**Input Parameters:**
- Via TestCaseSource `GetFlagsMappings()`

**SDK Calls:**
- `SzFlags.AllFlags` property access
- Flag value comparisons from metadata

**Expected Outputs:**
- Flag constants match metadata definitions
- AllFlags contains all individual flags

#### TestEnumFlag()

**Input Parameters:**
- Via TestCaseSource `GetEnumMappings()`

**SDK Calls:**
- Enum reflection and metadata comparison
- Flag name and value validation

**Expected Outputs:**
- Enum values match metadata
- Flag names are consistent

#### TestMetaFlag()

**Input Parameters:**
- Via TestCaseSource `GetMetaMappings()`

**SDK Calls:**
- Metadata flag mapping validation
- Aggregate flag consistency checks

**Expected Outputs:**
- Metadata mappings are correct
- Aggregate flags contain expected components

#### TestNamedMappings()

**Input Parameters:**
- Via TestCaseSource `GetEnumMappings()`

**SDK Calls:**
- Named flags dictionary access
- Flag name lookup operations

**Expected Outputs:**
- Flag names exist in named dictionary
- Values match enum constants

#### TestGetGroupsUnrecognized()

**Input Parameters:**
- Unrecognized flag value (typically -1 or invalid value)

**SDK Calls:**
- `SzFlags.GetGroups(unrecognizedFlag)`

**Expected Outputs:**
- Returns empty collection for unrecognized flags
- No exceptions thrown

#### TestGetGroups()

**Input Parameters:**
- Via TestCaseSource `GetEnumMappings()`

**SDK Calls:**
- `SzFlags.GetGroups(flagValue)`

**Expected Outputs:**
- Returns correct group collection
- Group membership is accurate

#### TestEnumGroup()

**Input Parameters:**
- Via TestCaseSource `GetEnumGroups()`

**SDK Calls:**
- Group-specific flag validation

**Expected Outputs:**
- Group flags are properly categorized
- No flag belongs to wrong group

#### TestMetaGroup()

**Input Parameters:**
- Via TestCaseSource `GetMetaGroups()`

**SDK Calls:**
- Metadata group validation

**Expected Outputs:**
- Metadata groups are consistent
- Group relationships are correct

#### TestToFlagString()

**Input Parameters:**
- Via TestCaseSource `GetToFlagStringParams()`

**Test Data Examples:**
- `SzSearchIncludeNameOnly`: "SZ_SEARCH_INCLUDE_NAME_ONLY"
- `SzEntityIncludeEntityName`: "SZ_ENTITY_INCLUDE_ENTITY_NAME"
- Combined flags: "SZ_SEARCH_INCLUDE_NAME_ONLY|SZ_ENTITY_INCLUDE_ENTITY_NAME"

**SDK Calls:**
- `SzFlags.ToFlagString(flagValue)`

**Expected Outputs:**
- Correct string representation
- Proper flag combination formatting
- Pipe-separated for multiple flags

#### TestZeroToString()

**Input Parameters:**
- Via TestCaseSource `GetEnumFlagGroups()`
- Zero value for each flag group

**SDK Calls:**
- `SzFlags.ToFlagString(0L, flagGroup)`

**Expected Outputs:**
- Returns appropriate zero representation
- Group-specific formatting

#### TestGetFlags()

**Input Parameters:**
- Via TestCaseSource `GetEnumFlagGroups()`

**SDK Calls:**
- `SzFlags.GetFlags(flagGroup)`

**Expected Outputs:**
- Returns all flags for the group
- No missing or extra flags

#### TestGetAmbiguousNamedFlags()

**Input Parameters:**
- Specific TestCase values:
  - `SzSearchIncludeNameOnly`
  - Other ambiguous flag names

**SDK Calls:**
- Flag name resolution
- Ambiguity detection

**Expected Outputs:**
- Ambiguous flags are properly identified
- Resolution returns correct values

#### TestFlagsToLong()

**Input Parameters:**
- Via TestCaseSource `GetEnumMappings()` and `GetFlagsMappings()`

**SDK Calls:**
- `SzFlags.FlagsToLong(flagValue)`

**Expected Outputs:**
- Correct long value conversion
- Bidirectional conversion accuracy

#### TestNullFlagsToLong()

**Input Parameters:**
- `null` flag value

**SDK Calls:**
- `SzFlags.FlagsToLong(null)`

**Expected Outputs:**
- Returns 0L for null input
- No exceptions thrown

---

### UtilitiesTest.cs

This file tests utility functions for data transformation and formatting.

#### TestHexFormat(long value)

**Input Parameters:**
- `value`: 0L, 1L, 2L, 20L, 40L, 80L, 160L, 3200L, 64000L, 128345789L

**SDK Calls:**
- `Utilities.HexFormat(value)`
- `Convert.ToInt64(hexString, 16)` (for round-trip validation)

**Expected Outputs:**
- Proper hexadecimal formatting
- Round-trip conversion accuracy: `Assert.That(Convert.ToInt64(hexResult, 16), Is.EqualTo(value))`

**Test Setup:**
- None required

#### testJsonEscapeNull()

**Input Parameters:**
- `null` string value

**SDK Calls:**
- `Utilities.JsonEscape(null)`

**Expected Outputs:**
- `Assert.That(result, Is.EqualTo("null"))`

#### TestJsonEscape(string value)

**Input Parameters:**
- `value`:
  - "Hello"
  - "Hello,\nWorld"
  - "\f\b\\\tHey!\r\n"
  - "Bell \u0007!"

**SDK Calls:**
- `Utilities.JsonEscape(value)`
- `JsonDocument.Parse(escapedJson)` (for validation)

**Expected Outputs:**
- Proper JSON escaping
- Valid JSON parsing
- Round-trip accuracy: original value equals parsed value

**Test Setup:**
- Initialize() [OneTimeSetUp]: Environment setup
- Complete() [OneTimeTearDown]: Environment cleanup

---

## Core Test Files

### SzCoreEngineBasicsTest.cs

This file tests fundamental engine operations and basic functionality.

#### TestGetNativeApi()

**Input Parameters:**
- None

**SDK Calls:**
- `engine.GetNativeApi()`

**Expected Outputs:**
- `Assert.That(nativeApi, Is.Not.Null)`

**Test Setup:**
- Requires initialized SzCoreEnvironment
- Uses PerformTest() wrapper for error handling

#### TestEncodeDataSources()

**Input Parameters:**
- `null` (ISet<string>)
- Empty ISet<string>
- Single data source: `{"CUSTOMERS"}`
- Multiple data sources: `{"CUSTOMERS", "EMPLOYEES", "WATCHLIST"}`

**SDK Calls:**
- `SzCoreEngine.EncodeDataSources(dataSources)`

**Expected Outputs:**
- `null` input → `null` output
- Empty set → `"[]"`
- Single source → `"[\"CUSTOMERS\"]"`
- Multiple sources → `"[\"CUSTOMERS\",\"EMPLOYEES\",\"WATCHLIST\"]"`

#### TestEncodeEntityIds()

**Input Parameters:**
- `null` (ISet<long>)
- Empty ISet<long>
- Single entity ID: `{1001L}`
- Multiple entity IDs: `{1001L, 1002L, 1003L}`

**SDK Calls:**
- `SzCoreEngine.EncodeEntityIDs(entityIds)`

**Expected Outputs:**
- `null` input → `null` output
- Empty set → `"[]"`
- Single ID → `"[1001]"`
- Multiple IDs → `"[1001,1002,1003]"`

#### TestEncodeRecordKeys()

**Input Parameters:**
- `null` (ISet<(string, string)>)
- Empty ISet<(string, string)>
- Single record key: `{("CUSTOMERS", "1001")}`
- Multiple record keys: `{("CUSTOMERS", "1001"), ("EMPLOYEES", "2001"), ("WATCHLIST", "3001")}`

**SDK Calls:**
- `SzCoreEngine.EncodeRecordKeys(recordKeys)`

**Expected Outputs:**
- `null` input → `null` output
- Empty set → `"[]"`
- Single key → `"[{\"dataSourceCode\":\"CUSTOMERS\",\"recordId\":\"1001\"}]"`
- Multiple keys → JSON array with all record key objects

#### TestPrimeEngine()

**Input Parameters:**
- None

**SDK Calls:**
- `engine.PrimeEngine()`

**Expected Outputs:**
- No exceptions thrown
- Method completes successfully

**Test Setup:**
- All tests use common SzCoreEnvironment setup
- PerformTest() wrapper for consistent error handling

---

### SzCoreEngineReadTest.cs

This file tests entity and record reading operations with comprehensive test data.

#### Test Data Sources

**PASSENGERS Data Source:**
```json
{
  "RECORD_ID": "1001",
  "NAME_FIRST": "Joe",
  "NAME_LAST": "Schmoe",
  "PHONE_NUMBER": "702-919-1300",
  "ADDR_FULL": "101 Main Street, Anywhere, Texas 73227"
}
{
  "RECORD_ID": "1002",
  "NAME_FIRST": "John",
  "NAME_LAST": "Doe",
  "PHONE_NUMBER": "818-555-1313",
  "ADDR_FULL": "100 Main Street, Anytown, Texas 73227"
}
{
  "RECORD_ID": "1003",
  "NAME_FIRST": "Jane",
  "NAME_LAST": "Doe",
  "PHONE_NUMBER": "818-555-1212",
  "ADDR_FULL": "100 Main Street, Anytown, Texas 73227"
}
{
  "RECORD_ID": "1004",
  "NAME_FIRST": "Robert",
  "NAME_LAST": "Smith",
  "PHONE_NUMBER": "702-919-1300",
  "ADDR_FULL": "101 Main Street, Anywhere, Texas 73227"
}
```

**EMPLOYEES Data Source:**
```json
{
  "RECORD_ID": "1001",
  "NAME_FIRST": "John",
  "NAME_LAST": "Doe",
  "PHONE_NUMBER": "818-555-1313",
  "ADDR_FULL": "100 Main Street, Anytown, Texas 73227"
}
{
  "RECORD_ID": "1002",
  "NAME_FIRST": "Jane",
  "NAME_LAST": "Doe",
  "PHONE_NUMBER": "818-555-1212",
  "ADDR_FULL": "100 Main Street, Anytown, Texas 73227"
}
{
  "RECORD_ID": "1003",
  "NAME_FIRST": "Robert",
  "NAME_LAST": "Smith",
  "PHONE_NUMBER": "702-919-1300",
  "ADDR_FULL": "101 Main Street, Anywhere, Texas 73227"
}
{
  "RECORD_ID": "1004",
  "NAME_FIRST": "Bill",
  "NAME_LAST": "Wright",
  "PHONE_NUMBER": "513-456-7890",
  "ADDR_FULL": "200 Second Street, Somewhere, Ohio 45202"
}
```

**VIPS Data Source:**
```json
{
  "RECORD_ID": "1001",
  "NAME_FIRST": "Mark",
  "NAME_LAST": "Hightower",
  "PHONE_NUMBER": "513-456-7890",
  "ADDR_FULL": "200 Second Street, Somewhere, Ohio 45202"
}
{
  "RECORD_ID": "1002",
  "NAME_FIRST": "Bruce",
  "NAME_LAST": "Wayne",
  "PHONE_NUMBER": "888-bat-cave",
  "ADDR_FULL": "1007 Mountain Drive, Gotham, New York 10007"
}
```

#### TestExportCsvEntityReport()

**Input Parameters:**
- `columnList`: Various CSV column specifications
- `flags`: Different SzFlag combinations

**SDK Calls:**
- `engine.ExportCsvEntityReport(handle, columnList, flags)`

**Expected Outputs:**
- Valid CSV format with headers
- Correct number of data rows
- Proper column formatting
- `Assert.That(csvLines.Length, Is.GreaterThan(1))` (header + data)
- `Assert.That(csvLines[0], Does.Contain("RESOLVED_ENTITY_ID"))` (header validation)

#### TestExportJsonEntityReport()

**Input Parameters:**
- `flags`: Various SzFlag combinations including:
  - `SzFlags.SzExportIncludeRelatedEntities`
  - `SzFlags.SzEntityIncludeEntityName | SzFlags.SzEntityIncludeRecordData`

**SDK Calls:**
- `engine.ExportJsonEntityReport(handle, flags)`

**Expected Outputs:**
- Valid JSON format
- Parseable JSON structure
- Contains expected entity data
- `Assert.That(JsonDocument.Parse(jsonResult), Is.Not.Null)`

#### TestGetEntity()

**Input Parameters:**
- `dataSourceCode`: "PASSENGERS", "EMPLOYEES", "VIPS"
- `recordId`: "1001", "1002", "1003", "1004"
- `flags`: Various combinations:
  - `SzFlags.SzEntityIncludeEntityName`
  - `SzFlags.SzEntityIncludeRecordData`
  - `SzFlags.SzEntityIncludeRecordMatchInfo`
  - `SzFlags.SzEntityIncludeRecordJsonData`

**SDK Calls:**
- `engine.GetEntity(dataSourceCode, recordId, flags)`

**Expected Outputs:**
- Valid JSON entity data
- Correct entity ID format
- Proper record details
- `Assert.That(JsonDocument.Parse(entityJson), Is.Not.Null)`
- Entity ID verification: `Assert.That(entityId, Is.GreaterThan(0))`

#### TestGetEntityByEntityId()

**Input Parameters:**
- `entityId`: Retrieved from previous GetEntity calls
- `flags`: Various flag combinations

**SDK Calls:**
- `engine.GetEntity(entityId, flags)`

**Expected Outputs:**
- Consistent entity data
- Same entity ID returned
- Proper JSON structure

#### TestSearchByAttributes()

**Input Parameters:**
- Search criteria JSON strings:
  ```json
  {"PHONE_NUMBER": "702-919-1300"}
  {"ADDR_FULL": "100 Main Street, Anytown, Texas 73227"}
  {"NAME_FIRST": "John", "NAME_LAST": "Doe"}
  ```
- `searchProfile`: null (default profile)
- `flags`: Various combinations

**SDK Calls:**
- `engine.SearchByAttributes(searchJson, searchProfile, flags)`

**Expected Outputs:**
- Valid search results JSON
- Correct number of matches
- Proper scoring information
- `Assert.That(JsonDocument.Parse(searchResults), Is.Not.Null)`
- Match count validation based on test data

#### TestFindInterestingEntities()

**Input Parameters:**
- `entityIds`: Set of related entity IDs
- `flags`: Various flag combinations

**SDK Calls:**
- `engine.FindInterestingEntities(entityIds, flags)`

**Expected Outputs:**
- Related entity discovery
- Relationship information
- Valid JSON structure

**Test Setup:**
- LoadTestData(): Loads all test records from data sources
- PopulateTestData(): Inserts records into engine
- GetLoadedRecords(): Retrieves record keys for testing

---

### SzCoreEngineWriteTest.cs

This file tests record modification operations including add, update, delete, and redo processing.

#### Test Data Used

**Base Test Records:**
```json
{
  "DATA_SOURCE": "TEST",
  "RECORD_ID": "ABC123",
  "NAME_FIRST": "Joe",
  "NAME_LAST": "Schmoe",
  "PHONE_NUMBER": "702-919-1300",
  "ADDR_FULL": "101 Main Street, Anywhere, Texas 73227"
}
```

#### TestAddRecord()

**Input Parameters:**
- `dataSourceCode`: "TEST", "CUSTOMERS", "EMPLOYEES"
- `recordId`: "ABC123", "DEF456", "GHI789"
- `recordJson`: Complete JSON record data
- `flags`: Various combinations including:
  - `SzFlags.SzWithInfo`
  - `SzFlags.SzReturnRecordData`

**SDK Calls:**
- `engine.AddRecord(dataSourceCode, recordId, recordJson, flags)`

**Expected Outputs:**
- Successful record addition
- Valid response JSON when flags include info
- No exceptions thrown
- `Assert.That(response, Is.Not.Null)` when info requested

#### TestGetRecordPreview()

**Input Parameters:**
- `dataSourceCode`: "TEST"
- `recordId`: "PREVIEW_TEST_001"
- `recordJson`: Test record JSON
- `flags`: Preview-specific flags

**SDK Calls:**
- `engine.GetRecordPreview(dataSourceCode, recordId, recordJson, flags)`

**Expected Outputs:**
- Preview data showing potential changes
- Entity formation predictions
- Valid JSON structure
- No actual record modification

#### TestReevaluateRecord()

**Input Parameters:**
- `dataSourceCode`: From previously added records
- `recordId`: From previously added records
- `flags`: Various evaluation flags

**SDK Calls:**
- `engine.ReevaluateRecord(dataSourceCode, recordId, flags)`

**Expected Outputs:**
- Successful re-evaluation
- Updated entity relationships
- Valid response when info flags used

#### TestDeleteRecord()

**Input Parameters:**
- `dataSourceCode`: From previously added records
- `recordId`: From previously added records
- `flags`: Deletion flags including:
  - `SzFlags.SzWithInfo`

**SDK Calls:**
- `engine.DeleteRecord(dataSourceCode, recordId, flags)`

**Expected Outputs:**
- Successful record deletion
- Updated entity state
- Valid info response when requested
- Record no longer retrievable

#### TestProcessRedoRecord()

**Input Parameters:**
- None directly (processes redo queue entries)

**SDK Calls:**
- `engine.GetRedoRecord()`
- `engine.ProcessRedoRecord(redoRecord)`

**Expected Outputs:**
- Successful redo processing
- Queue entry consumption
- Updated entity states
- `Assert.That(redoRecord, Is.Not.Null)` when entries exist

#### TestCountRedoRecords()

**Input Parameters:**
- None

**SDK Calls:**
- `engine.CountRedoRecords()`

**Expected Outputs:**
- Accurate count of redo queue entries
- `Assert.That(count, Is.GreaterThanOrEqualTo(0))`

**Test Setup:**
- BeginTests(): Initialize test environment
- LoadTestData(): Prepare test records
- EndTests(): Cleanup test data

---

### SzCoreEngineGraphTest.cs

This file tests graph analysis and network discovery operations.

#### TestFindPath()

**Input Parameters:**
- `fromEntityId`: Starting entity ID
- `toEntityId`: Target entity ID
- `maxDegrees`: 1, 2, 3, 4, 5, 6 (various path lengths)
- `avoidEntityIds`: Set of entities to avoid in path
- `requiredDataSources`: Required data sources in path
- `flags`: Various graph flags

**SDK Calls:**
- `engine.FindPath(fromEntityId, toEntityId, maxDegrees, avoidEntityIds, requiredDataSources, flags)`

**Expected Outputs:**
- Valid path JSON when path exists
- Empty result when no path found
- Proper path structure with entities and relationships
- `Assert.That(JsonDocument.Parse(pathResult), Is.Not.Null)`
- Path validation: entities and relationships properly connected

#### TestFindNetwork()

**Input Parameters:**
- `entityIds`: Set of starting entity IDs
- `maxDegrees`: 1, 2, 3 (network expansion levels)
- `buildOut`: 1, 2, 3 (relationship depth)
- `maxEntities`: 10, 50, 100 (result size limits)
- `flags`: Network analysis flags

**SDK Calls:**
- `engine.FindNetwork(entityIds, maxDegrees, buildOut, maxEntities, flags)`

**Expected Outputs:**
- Network graph JSON structure
- Entity nodes and relationship edges
- Proper network topology
- Respects maxEntities limit
- `Assert.That(networkResult.Entities.Count, Is.LessThanOrEqualTo(maxEntities))`

**Test Data Relationships:**
- Family relationships (same address, shared phones)
- Employment connections (same company)
- Geographic proximity (similar addresses)

---

### SzCoreEngineHowTest.cs

This file tests entity resolution tracing and explanation functionality.

#### TestHowEntity()

**Input Parameters:**
- `entityId`: Target entity ID for analysis
- `flags`: How analysis flags including:
  - `SzFlags.SzHowIncludeFeatureScores`
  - `SzFlags.SzHowIncludeInternalFeatures`

**SDK Calls:**
- `engine.HowEntity(entityId, flags)`

**Expected Outputs:**
- Resolution step analysis
- Feature matching details
- Scoring information
- Virtual entity formation steps
- `Assert.That(JsonDocument.Parse(howResult), Is.Not.Null)`
- Contains resolution steps and feature analysis

**Analysis Includes:**
- Cross-data-source matching logic
- Feature scoring algorithms
- Identity consolidation decisions
- Record merge reasoning

---

### SzCoreEngineWhyTest.cs

This file tests relationship analysis and explanation functionality.

#### TestWhyEntities()

**Input Parameters:**
- `entityId1`: First entity for comparison
- `entityId2`: Second entity for comparison
- `flags`: Why analysis flags

**SDK Calls:**
- `engine.WhyEntities(entityId1, entityId2, flags)`

**Expected Outputs:**
- Relationship analysis JSON
- Matching features identification
- Scoring details
- Resolution decision explanation
- `Assert.That(JsonDocument.Parse(whyResult), Is.Not.Null)`

#### TestWhyRecords()

**Input Parameters:**
- `dataSourceCode1`: First record's data source
- `recordId1`: First record's ID
- `dataSourceCode2`: Second record's data source
- `recordId2`: Second record's ID
- `flags`: Analysis flags

**SDK Calls:**
- `engine.WhyRecords(dataSourceCode1, recordId1, dataSourceCode2, recordId2, flags)`

**Expected Outputs:**
- Record relationship analysis
- Feature comparison details
- Matching algorithm results

#### TestWhyRecordInEntity()

**Input Parameters:**
- `dataSourceCode`: Record's data source
- `recordId`: Record's ID
- `flags`: Analysis flags

**SDK Calls:**
- `engine.WhyRecordInEntity(dataSourceCode, recordId, flags)`

**Expected Outputs:**
- Entity membership explanation
- Record contribution analysis
- Feature impact assessment

#### TestWhySearch()

**Input Parameters:**
- `searchJson`: Search criteria JSON
- `searchProfile`: Search profile (usually null)
- `flags`: Search analysis flags

**SDK Calls:**
- `engine.WhySearch(searchJson, searchProfile, flags)`

**Expected Outputs:**
- Search match explanations
- Scoring rationale
- Feature matching analysis

**Common Relationship Patterns Tested:**
- Employment connections (same company/department)
- Spousal relationships (shared addresses, phones)
- Shared contact information (phones, emails)
- Geographic proximity (similar addresses)
- Name variations (nicknames, alternate spellings)

---

### SzCoreConfigTest.cs

This file tests configuration management operations.

#### TestCreateConfig()

**Input Parameters:**
- None (uses default template)

**SDK Calls:**
- `config.CreateConfig()`

**Expected Outputs:**
- Valid configuration object
- Default template applied
- Proper JSON structure
- `Assert.That(configHandle, Is.GreaterThan(0))`

#### TestCreateConfigFromTemplate()

**Input Parameters:**
- `templateJson`: Custom configuration template JSON

**SDK Calls:**
- `config.CreateConfig(templateJson)`

**Expected Outputs:**
- Configuration matches template
- Custom settings applied
- Valid configuration handle

#### TestExportConfig()

**Input Parameters:**
- `configHandle`: Configuration to export

**SDK Calls:**
- `config.Export(configHandle)`

**Expected Outputs:**
- Valid JSON configuration export
- Complete configuration data
- `Assert.That(JsonDocument.Parse(exportJson), Is.Not.Null)`

#### TestRegisterDataSource()

**Input Parameters:**
- `configHandle`: Target configuration
- `dataSourceCode`: "NEW_SOURCE", "ADDITIONAL_DS"
- `dataSourceJson`: Data source configuration JSON

**SDK Calls:**
- `config.RegisterDataSource(configHandle, dataSourceCode, dataSourceJson)`

**Expected Outputs:**
- Data source added to configuration
- Proper registration in registry
- Updated configuration state

#### TestUnregisterDataSource()

**Input Parameters:**
- `configHandle`: Target configuration
- `dataSourceCode`: Previously registered source

**SDK Calls:**
- `config.UnregisterDataSource(configHandle, dataSourceCode)`

**Expected Outputs:**
- Data source removed from configuration
- Registry properly updated
- Clean configuration state

#### TestGetDataSourceRegistry()

**Input Parameters:**
- `configHandle`: Configuration to query

**SDK Calls:**
- `config.GetDataSourceRegistry(configHandle)`

**Expected Outputs:**
- Complete data source registry
- All registered sources listed
- Proper JSON format
- `Assert.That(registryJson, Does.Contain(dataSourceCode))`

---

### SzCoreConfigManagerTest.cs

This file tests configuration lifecycle management.

#### TestCreateConfig()

**Input Parameters:**
- None (default configuration)

**SDK Calls:**
- `configManager.CreateConfig()`

**Expected Outputs:**
- New configuration ID assigned
- Configuration properly registered
- `Assert.That(configId, Is.GreaterThan(0))`

#### TestRegisterConfig()

**Input Parameters:**
- `configJson`: Complete configuration JSON
- `configComment`: "Test configuration registration"

**SDK Calls:**
- `configManager.RegisterConfig(configJson, configComment)`

**Expected Outputs:**
- Configuration registered with ID
- Comment properly stored
- Configuration available for use

#### TestGetConfigRegistry()

**Input Parameters:**
- None

**SDK Calls:**
- `configManager.GetConfigRegistry()`

**Expected Outputs:**
- Complete configuration registry
- All registered configurations listed
- Includes IDs, comments, timestamps
- `Assert.That(JsonDocument.Parse(registryJson), Is.Not.Null)`

#### TestSetDefaultConfigID()

**Input Parameters:**
- `configId`: Configuration to set as default

**SDK Calls:**
- `configManager.SetDefaultConfigID(configId)`

**Expected Outputs:**
- Default configuration updated
- New default active
- System uses new configuration

#### TestReplaceDefaultConfigID()

**Input Parameters:**
- `oldConfigId`: Current default configuration
- `newConfigId`: New default configuration

**SDK Calls:**
- `configManager.ReplaceDefaultConfigID(oldConfigId, newConfigId)`

**Expected Outputs:**
- Atomic replacement of default
- Old configuration deactivated
- New configuration activated

#### TestGetDefaultConfigID()

**Input Parameters:**
- None

**SDK Calls:**
- `configManager.GetDefaultConfigID()`

**Expected Outputs:**
- Current default configuration ID
- `Assert.That(defaultId, Is.GreaterThan(0))`

**Configuration Lifecycle Patterns:**
- Configuration versioning with incremental IDs
- Comment tracking for configuration changes
- Default configuration management
- Configuration registry maintenance

---

### SzCoreDiagnosticTest.cs

This file tests system diagnostics and monitoring operations.

#### TestGetRepositoryInfo()

**Input Parameters:**
- None

**SDK Calls:**
- `diagnostic.GetRepositoryInfo()`

**Expected Outputs:**
- Repository statistics JSON
- System configuration details
- Performance metrics
- `Assert.That(JsonDocument.Parse(repoInfo), Is.Not.Null)`
- Contains database statistics and system info

#### TestCheckRepositoryPerformance()

**Input Parameters:**
- `secondsToRun`: 5 (standard 5-second performance test)

**SDK Calls:**
- `diagnostic.CheckRepositoryPerformance(secondsToRun)`

**Expected Outputs:**
- Performance test results
- Throughput measurements
- System capability assessment
- Test duration compliance
- `Assert.That(perfResult, Does.Contain("insertTime"))`

#### TestGetFeature()

**Input Parameters:**
- `featureId`: Specific feature ID for analysis

**SDK Calls:**
- `diagnostic.GetFeature(featureId)`

**Expected Outputs:**
- Feature analysis details
- Feature usage statistics
- Entity relationships for feature
- `Assert.That(JsonDocument.Parse(featureInfo), Is.Not.Null)`

#### TestPurgeRepository()

**Input Parameters:**
- None (full repository purge)

**SDK Calls:**
- `diagnostic.PurgeRepository()`

**Expected Outputs:**
- Repository cleaned successfully
- All data removed
- System reset to clean state
- No exceptions during purge

**Diagnostic Patterns:**
- System health monitoring
- Performance benchmarking
- Repository management
- Feature analysis and optimization

---

### SzCoreEnvironmentTest.cs

This file tests environment management and lifecycle operations.

#### TestBuilderPattern()

**Input Parameters:**
- `instanceName`: "test_instance"
- `settings`: Configuration JSON
- `configId`: Specific configuration ID

**SDK Calls:**
- `SzCoreEnvironment.Builder()`
- `builder.InstanceName(instanceName)`
- `builder.Settings(settings)`
- `builder.ConfigId(configId)`
- `builder.Build()`

**Expected Outputs:**
- Properly initialized environment
- Correct configuration applied
- All components accessible
- `Assert.That(environment, Is.Not.Null)`

#### TestSingletonEnforcement()

**Input Parameters:**
- Multiple environment creation attempts

**SDK Calls:**
- Multiple `SzCoreEnvironment.Builder().Build()` calls

**Expected Outputs:**
- Exception thrown on second build attempt
- Singleton pattern enforced
- First environment remains valid

#### TestGetEngine()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.GetEngine()`

**Expected Outputs:**
- Valid engine instance
- Engine properly initialized
- `Assert.That(engine, Is.Not.Null)`

#### TestGetConfig()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.GetConfig()`

**Expected Outputs:**
- Valid config instance
- Configuration manager accessible
- `Assert.That(config, Is.Not.Null)`

#### TestGetConfigManager()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.GetConfigManager()`

**Expected Outputs:**
- Valid config manager instance
- Configuration lifecycle accessible
- `Assert.That(configManager, Is.Not.Null)`

#### TestGetDiagnostic()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.GetDiagnostic()`

**Expected Outputs:**
- Valid diagnostic instance
- System monitoring accessible
- `Assert.That(diagnostic, Is.Not.Null)`

#### TestGetProduct()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.GetProduct()`

**Expected Outputs:**
- Valid product instance
- Version information accessible
- `Assert.That(product, Is.Not.Null)`

#### TestDestroy()

**Input Parameters:**
- None

**SDK Calls:**
- `environment.Destroy()`

**Expected Outputs:**
- Environment properly cleaned up
- Resources released
- Singleton reset for next use

**Environment Patterns:**
- Builder pattern for initialization
- Singleton enforcement
- Component factory functionality
- Proper resource lifecycle management

---

### SzCoreProductTest.cs

This file tests product information and metadata operations.

#### TestGetLicense()

**Input Parameters:**
- None

**SDK Calls:**
- `product.GetLicense()`

**Expected Outputs:**
- License information JSON
- Customer details
- Contract information
- Issue date
- `Assert.That(JsonDocument.Parse(licenseJson), Is.Not.Null)`
- `Assert.That(licenseJson, Does.Contain("customer"))`
- `Assert.That(licenseJson, Does.Contain("contract"))`
- `Assert.That(licenseJson, Does.Contain("issueDate"))`

#### TestGetVersion()

**Input Parameters:**
- None

**SDK Calls:**
- `product.GetVersion()`

**Expected Outputs:**
- Version information JSON
- Build number
- Version string
- Component versions
- `Assert.That(JsonDocument.Parse(versionJson), Is.Not.Null)`
- `Assert.That(versionJson, Does.Contain("VERSION"))`
- `Assert.That(versionJson, Does.Contain("BUILD_NUMBER"))`

#### TestGetLastException()

**Input Parameters:**
- None (tests exception handling)

**SDK Calls:**
- `product.GetLastException()`

**Expected Outputs:**
- Exception information JSON
- Error code
- Error message
- Exception details

#### TestClearLastException()

**Input Parameters:**
- None

**SDK Calls:**
- `product.ClearLastException()`

**Expected Outputs:**
- Exception state cleared
- Clean error state
- No pending exceptions

**Product Information Patterns:**
- License validation and information
- Version tracking and reporting
- Exception state management
- System metadata access

---

## Common Test Infrastructure

### Test Environment Setup

All core tests use common infrastructure patterns:

#### AbstractTest Base Class

**Setup Methods:**
- `BeginTests()`: Initialize test environment
- `EndTests()`: Cleanup test resources
- `InitializeTestEnvironment()`: Configure repository

**Common Resources:**
- Test repository creation
- Configuration management
- Data loading utilities
- Error handling wrappers

#### Standard Test Data

**Data Sources Used:**
- `PASSENGERS`: Individual travelers
- `EMPLOYEES`: Company personnel
- `VIPS`: High-profile individuals
- `MARRIAGES`: Relationship records
- `CUSTOMERS`: Business customers
- `WATCHLIST`: Security monitoring

**Record Patterns:**
- Complete records with all attributes
- Partial records for testing gaps
- Duplicate records across sources
- Related records (family, employment)

#### Error Handling Patterns

**PerformTest() Wrapper:**
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

**Exception Validation:**
- Error code verification
- Message content validation
- Inner exception checking
- Exception type confirmation

---

## Implementation Notes for Rust

### Key Requirements

1. **100% Test Coverage**: Every test method must have Rust equivalent
2. **No Mock Tests**: All tests use actual Senzing native library
3. **Exact Parameter Matching**: Input values must match C# tests exactly
4. **Same Test Data**: Use identical JSON records and search criteria
5. **Error Validation**: Test both success and failure paths
6. **Resource Cleanup**: Automatic cleanup after each test
7. **Thread Safety**: Concurrent operation testing where applicable

### Test Structure Recommendations

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_add_record() {
        let env = TestEnvironment::new().unwrap();
        let engine = env.get_engine().unwrap();

        let record_json = json!({
            "RECORD_ID": "ABC123",
            "NAME_FIRST": "Joe",
            "NAME_LAST": "Schmoe",
            "PHONE_NUMBER": "702-919-1300"
        });

        let result = engine.add_record(
            "TEST",
            "ABC123",
            &record_json.to_string(),
            SzFlags::SZ_WITH_INFO
        );

        assert!(result.is_ok());
        // Additional assertions...
    }
}
```

### Data Validation Patterns

```rust
fn validate_entity_json(json_str: &str) -> Result<(), SzError> {
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;

    // Validate required fields
    assert!(parsed["RESOLVED_ENTITY"]["ENTITY_ID"].is_number());
    assert!(parsed["RESOLVED_ENTITY"]["RECORDS"].is_array());

    Ok(())
}
```

This comprehensive analysis provides the exact test specifications needed to implement equivalent Rust tests with identical inputs, SDK calls, and expected outputs.
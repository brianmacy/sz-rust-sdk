## Senzing Installation Paths

### macOS (Homebrew)

Installed via: `brew install senzingsdk-runtime-unofficial`

```
Base:       /opt/homebrew/opt/senzing/runtime
Library:    /opt/homebrew/opt/senzing/runtime/er/lib/libSz.dylib
Config:     /opt/homebrew/opt/senzing/runtime/er/resources/templates
Resources:  /opt/homebrew/opt/senzing/runtime/er/resources
Support:    /opt/homebrew/opt/senzing/runtime/data
```

Required environment variable:

```bash
export DYLD_LIBRARY_PATH=/opt/homebrew/opt/senzing/runtime/er/lib
```

### Linux (Standard Installation)

```
Base:       /opt/senzing/er
Library:    /opt/senzing/er/lib/libSz.so
Config:     /opt/senzing/er/resources/templates
Resources:  /opt/senzing/er/resources
Support:    /opt/senzing/data
```

Required environment variable:

```bash
export LD_LIBRARY_PATH=/opt/senzing/er/lib
```

### Engine Configuration JSON

The SDK's `ExampleEnvironment` helper auto-detects paths. For manual configuration, pass JSON to `get_instance()`:

```json
{
  "PIPELINE": {
    "CONFIGPATH": "/opt/homebrew/opt/senzing/runtime/er/resources/templates",
    "RESOURCEPATH": "/opt/homebrew/opt/senzing/runtime/er/resources",
    "SUPPORTPATH": "/opt/homebrew/opt/senzing/runtime/data"
  },
  "SQL": {
    "CONNECTION": "sqlite3://na:na@/tmp/senzing.db"
  }
}
```

Or set `SENZING_ENGINE_CONFIGURATION_JSON` environment variable.

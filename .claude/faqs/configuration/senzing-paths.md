## Senzing Installation Paths

### macOS (Homebrew — Official Cask)

Installed via:

```bash
brew tap senzing/senzingsdk https://github.com/Senzing/homebrew-senzingsdk
brew install --cask senzingsdk
```

```
Base:       /opt/homebrew/opt/senzing
Library:    /opt/homebrew/opt/senzing/er/lib/libSz.dylib
Config:     /opt/homebrew/opt/senzing/er/resources/templates
Resources:  /opt/homebrew/opt/senzing/er/resources
Support:    /opt/homebrew/opt/senzing/data
```

Required environment variable:

```bash
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/senzing/er/lib:/opt/homebrew/opt/sqlite/lib:/opt/homebrew/opt/openssl@3/lib"
```

Note: The 4.3 cask is missing rpath for openssl and sqlite3 — include their Homebrew lib paths.
Or source the setup script: `source "$(brew --prefix)/opt/senzing/er/setupEnv"`

### Windows (Scoop — Official Bucket)

```pwsh
scoop bucket add senzingsdk https://github.com/Senzing/scoop-senzingsdk
scoop install senzingsdk/senzingsdk
```

Scoop sets `SENZING_DIR` and adds `er\lib` to `PATH` automatically.

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
    "CONFIGPATH": "/opt/homebrew/opt/senzing/er/resources/templates",
    "RESOURCEPATH": "/opt/homebrew/opt/senzing/er/resources",
    "SUPPORTPATH": "/opt/homebrew/opt/senzing/data"
  },
  "SQL": {
    "CONNECTION": "internal://"
  }
}
```

Or set `SENZING_ENGINE_CONFIGURATION_JSON` environment variable.

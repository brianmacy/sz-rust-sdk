## Upgrading from Unofficial to Official Homebrew

### Old (unofficial tap)

```bash
brew install senzing/tap/senzing
```

Paths: `/opt/homebrew/opt/senzing/runtime/er/...` (note `runtime/` subdirectory)

### New (official cask, v4.3+)

```bash
brew tap senzing/senzingsdk https://github.com/Senzing/homebrew-senzingsdk
brew install --cask senzingsdk
```

Paths: `/opt/homebrew/opt/senzing/er/...` (no `runtime/` — symlink points directly to `Caskroom/senzingsdk/<version>/senzing`)

### Upgrade steps

```bash
brew uninstall senzing/tap/senzing
brew untap senzing/tap
brew tap senzing/senzingsdk https://github.com/Senzing/homebrew-senzingsdk
HOMEBREW_SENZING_ACCEPT_EULA=i_accept_the_senzing_eula brew install --cask senzingsdk
```

### DYLD_LIBRARY_PATH change

The 4.3 cask is missing rpath entries for openssl and sqlite3. Use:

```bash
export DYLD_LIBRARY_PATH="/opt/homebrew/opt/senzing/er/lib:/opt/homebrew/opt/sqlite/lib:/opt/homebrew/opt/openssl@3/lib"
```

Or source the setup script: `source "$(brew --prefix)/opt/senzing/er/setupEnv"`

### SDK auto-detection

`build.rs` and `ExampleEnvironment` check official paths first, then fall back to legacy. If both are installed, official wins. No code changes needed when upgrading — just reinstall and rebuild.

### Windows (Scoop)

```pwsh
scoop bucket add senzingsdk https://github.com/Senzing/scoop-senzingsdk
scoop install senzingsdk/senzingsdk
```

Scoop sets `SENZING_DIR` and adds the lib directory to `PATH` automatically.

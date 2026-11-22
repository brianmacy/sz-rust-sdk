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

### Staging cask installed next to the released one

Installing or upgrading the `senzingsdk-staging` cask re-points `/opt/homebrew/opt/senzing`
to `Caskroom/senzingsdk-staging/<version>/senzing`. Auto-detection in `build.rs` and
`ExampleEnvironment` then builds and tests against the pre-release. The released cask stays
under `Caskroom/senzingsdk/<version>/senzing`. To test against it, set `SENZING_DIR` to its
`er` directory. Both `build.rs` (link path) and `ExampleEnvironment` (resources, templates,
`../data`) use it. Put its `lib` first in `DYLD_LIBRARY_PATH`:

```bash
export SENZING_DIR=/opt/homebrew/Caskroom/senzingsdk/4.5.0.26320/senzing/er
export DYLD_LIBRARY_PATH="$SENZING_DIR/lib:/opt/homebrew/opt/openssl@3/lib:/opt/homebrew/opt/sqlite/lib"
cargo test --all-features
```

`build.rs` declares `rerun-if-env-changed=SENZING_DIR`, so switching versions triggers a rebuild.
To confirm which library loaded, run a test binary with `DYLD_PRINT_LIBRARIES=1` and grep for `libSz`.

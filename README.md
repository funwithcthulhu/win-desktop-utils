# win-desktop-utils

Windows-first desktop utility helpers for Rust apps.

`win-desktop-utils` provides small, focused helpers for common Windows desktop-app tasks without forcing consumers to work directly with raw Win32 shell and mutex APIs.

## Scope

This crate currently provides helpers to:

- open an existing file or directory with the default Windows handler
- open a URL with the default browser or registered handler
- reveal an existing path in Explorer
- move an existing file or directory to the Recycle Bin
- enforce single-instance behavior with a named mutex
- resolve per-user roaming and local app-data paths
- create per-user roaming and local app-data paths if needed
- check whether the current process is elevated
- relaunch the current executable as administrator

This crate supports Windows only.

## Installation

```toml
[dependencies]
win-desktop-utils = "0.2"
```

## Quick Start

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let _guard = match win_desktop_utils::single_instance("demo-app")? {
        Some(guard) => guard,
        None => {
            println!("already running");
            return Ok(());
        }
    };

    let local = win_desktop_utils::ensure_local_app_data("demo-app")?;
    println!("local app dir: {}", local.display());

    Ok(())
}
```

## Current API

- `open_with_default(path)`
- `open_with_verb(verb, path)`
- `open_url(url)`
- `reveal_in_explorer(path)`
- `move_to_recycle_bin(path)`
- `move_paths_to_recycle_bin(paths)`
- `single_instance(app_id)`
- `single_instance_with_scope(app_id, scope)`
- `roaming_app_data(app_name)`
- `local_app_data(app_name)`
- `ensure_roaming_app_data(app_name)`
- `ensure_local_app_data(app_name)`
- `is_elevated()`
- `restart_as_admin(args)`
- `InstanceScope::{CurrentSession, Global}`

## Examples

The [`examples/`](https://github.com/funwithcthulhu/win-desktop-utils/tree/main/examples) directory includes runnable samples for:

- app-data path lookup and creation
- URL opening and Explorer reveal helpers
- shell verb execution for files and directories
- Recycle Bin integration
- elevation checks and relaunch
- single-instance enforcement
- single-instance enforcement across all sessions

Run any example with:

```powershell
cargo run --example single_instance
```

## Error behavior

The crate exposes a small public error type with explicit path-related cases.

Notable error distinctions include:

- `Error::InvalidInput(...)` for empty or malformed input
- `Error::PathNotAbsolute` when an operation requires an absolute path
- `Error::PathDoesNotExist` when an operation requires an existing path
- `Error::WindowsApi { .. }` when a Win32 or shell operation reports failure
- `Error::Io(...)` for underlying I/O failures

## Behavior notes

- `open_with_default` requires a non-empty existing path.
- `open_with_verb` requires a non-empty existing path and a non-empty shell verb such as `open` or `properties`.
- `open_url` trims surrounding whitespace before delegating to the Windows shell.
- `reveal_in_explorer` requires an existing path and launches `explorer.exe`.
- `move_to_recycle_bin` requires an absolute existing path and uses `IFileOperation` on a dedicated STA thread for recycle-bin behavior.
- `move_paths_to_recycle_bin` validates every path before starting one recycle-bin shell operation.
- `roaming_app_data` and `local_app_data` resolve the base directory via `SHGetKnownFolderPath`.
- `single_instance` uses a `Local\...` named mutex, so the lock is scoped to the current Windows session.
- `single_instance_with_scope` can opt into either the current-session (`Local\...`) or global (`Global\...`) namespace.
- `single_instance` rejects backslashes in `app_id` because Windows reserves them for kernel-object namespaces such as `Local\` and `Global\`.
- Keep the returned `InstanceGuard` alive for as long as the process should own the single-instance lock.
- `restart_as_admin` starts a new elevated instance of the current executable and does not terminate the current process.
- `restart_as_admin` rejects arguments containing NUL bytes.

## Quality

The crate includes:

- automated tests for validation and single-instance behavior
- unit tests covering argument quoting and input normalization edge cases
- doctest examples in the public modules
- Windows CI via GitHub Actions for formatting, tests, clippy, examples, docs, and packaging
- docs published on docs.rs

## Links

- Crates.io: https://crates.io/crates/win-desktop-utils
- Docs: https://docs.rs/win-desktop-utils
- Repository: https://github.com/funwithcthulhu/win-desktop-utils
- Changelog: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/CHANGELOG.md

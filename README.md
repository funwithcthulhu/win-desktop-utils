# win-desktop-utils

[![Crates.io](https://img.shields.io/crates/v/win-desktop-utils.svg)](https://crates.io/crates/win-desktop-utils)
[![Docs.rs](https://docs.rs/win-desktop-utils/badge.svg)](https://docs.rs/win-desktop-utils)
[![CI](https://github.com/funwithcthulhu/win-desktop-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/funwithcthulhu/win-desktop-utils/actions/workflows/ci.yml)

Windows desktop helpers for Rust apps.

`win-desktop-utils` wraps the small but fiddly Windows desktop tasks that show up in real apps: shell opening, Explorer reveal, Recycle Bin moves, shortcuts, app-data paths, single-instance locks, and elevation prompts.

Use the low-level helpers directly, or start with `DesktopApp` for the common “app identity, app-data directory, and single-instance guard” workflow.

## Scope

This crate currently provides helpers to:

- open an existing file or directory with the default Windows handler
- open an existing file or directory with an explicit Windows shell verb
- open a URL with the default browser or registered handler
- reveal an existing path in Explorer
- move an existing file or directory to the Recycle Bin
- empty the Recycle Bin silently
- create Windows `.lnk` shortcuts
- create Internet Shortcut `.url` files
- enforce single-instance behavior with a named mutex
- configure single-instance behavior with builder-style options
- resolve per-user roaming and local app-data paths
- create per-user roaming and local app-data paths if needed
- check whether the current process is elevated
- relaunch the current executable as administrator
- launch arbitrary commands through shell verbs such as `open` or `runas`

This crate supports Windows only.

## Installation

```toml
[dependencies]
win-desktop-utils = "0.3"
```

Default features enable the full API. To keep a dependency focused, disable defaults and opt into only the modules you need:

```toml
[dependencies]
win-desktop-utils = { version = "0.3", default-features = false, features = ["paths", "instance"] }
```

## Quick Start

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::new("demo-app")?;

    let _guard = match app.single_instance()? {
        Some(guard) => guard,
        None => {
            println!("already running");
            return Ok(());
        }
    };

    let local = app.ensure_local_data_dir()?;
    println!("local app dir: {}", local.display());

    Ok(())
}
```

## Feature Flags

- `app`: `DesktopApp` facade for app-data and single-instance startup.
- `paths`: per-user local and roaming app-data helpers.
- `instance`: named-mutex single-instance helpers.
- `shell`: shell opening, URL, Explorer, and shell-verb helpers.
- `recycle-bin`: Recycle Bin move and empty helpers.
- `shortcuts`: `.lnk` and `.url` shortcut helpers.
- `elevation`: elevation detection and shell-based relaunch helpers.

## Current API

- `DesktopApp`
- `open_with_default(path)`
- `open_with_verb(verb, path)`
- `show_properties(path)`
- `print_with_default(path)`
- `open_url(url)`
- `reveal_in_explorer(path)`
- `open_containing_folder(path)`
- `move_to_recycle_bin(path)`
- `move_paths_to_recycle_bin(paths)`
- `empty_recycle_bin()`
- `empty_recycle_bin_for_root(root_path)`
- `create_shortcut(shortcut_path, target_path, options)`
- `create_url_shortcut(shortcut_path, url)`
- `single_instance(app_id)`
- `single_instance_with_scope(app_id, scope)`
- `single_instance_with_options(options)`
- `roaming_app_data(app_name)`
- `local_app_data(app_name)`
- `ensure_roaming_app_data(app_name)`
- `ensure_local_app_data(app_name)`
- `is_elevated()`
- `restart_as_admin(args)`
- `run_as_admin(executable, args)`
- `run_with_verb(verb, executable, args)`
- `InstanceScope::{CurrentSession, Global}`
- `SingleInstanceOptions`
- `ShortcutOptions`
- `ShortcutIcon`

## Cookbook

The [`docs/cookbook.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/cookbook.md) file has copy-paste recipes for:

- starting a single-instance app
- creating local and roaming app-data folders
- opening files, URLs, folders, and Properties
- creating `.lnk` and `.url` shortcuts
- relaunching as administrator
- moving files to the Recycle Bin

## Examples

The [`examples/`](https://github.com/funwithcthulhu/win-desktop-utils/tree/main/examples) directory includes runnable samples for:

- app-data path lookup and creation
- URL opening and Explorer reveal helpers
- shell verb execution for files and directories
- Recycle Bin integration
- shortcut creation
- elevation checks and relaunch
- launching arbitrary commands through shell verbs
- single-instance enforcement
- single-instance enforcement across all sessions
- builder-style single-instance options

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
- `show_properties` and `print_with_default` are convenience wrappers over shell verbs.
- `open_url` trims surrounding whitespace before delegating to the Windows shell.
- `reveal_in_explorer` requires an existing path and launches `explorer.exe`.
- `open_containing_folder` requires an existing path and opens its parent directory.
- `move_to_recycle_bin` requires an absolute existing path and uses `IFileOperation` on a dedicated STA thread for recycle-bin behavior.
- `move_paths_to_recycle_bin` validates every path before starting one recycle-bin shell operation.
- `empty_recycle_bin` and `empty_recycle_bin_for_root` permanently empty Recycle Bin contents without showing shell UI.
- `create_shortcut` requires an absolute `.lnk` path, an existing output parent directory, and an existing absolute target path.
- `create_url_shortcut` requires an absolute `.url` path and rejects line breaks in URLs to avoid malformed shortcut files.
- `roaming_app_data` and `local_app_data` resolve the base directory via `SHGetKnownFolderPath`.
- `single_instance` uses a `Local\...` named mutex, so the lock is scoped to the current Windows session.
- `single_instance_with_scope` can opt into either the current-session (`Local\...`) or global (`Global\...`) namespace.
- `SingleInstanceOptions` provides a small builder around single-instance app ID and scope selection.
- `single_instance` rejects backslashes in `app_id` because Windows reserves them for kernel-object namespaces such as `Local\` and `Global\`.
- Keep the returned `InstanceGuard` alive for as long as the process should own the single-instance lock.
- `restart_as_admin` starts a new elevated instance of the current executable and does not terminate the current process.
- `run_as_admin` starts an arbitrary command with the `runas` shell verb and may trigger UAC.
- `restart_as_admin`, `run_as_admin`, and `run_with_verb` reject arguments containing NUL bytes.

## Quality

The crate includes:

- automated tests for validation and single-instance behavior
- unit tests covering argument quoting and input normalization edge cases
- doctest examples in the public modules
- Windows CI via GitHub Actions for MSRV, formatting, tests, clippy, examples, docs, packaging, dependency policy, and semver checks
- docs published on docs.rs

The minimum supported Rust version is `1.82`, matching the current `windows` crate dependency floor.

## Support Policy

- Supported OS family: Windows 10 and Windows 11.
- Supported Rust: 1.82 and newer.
- Public API compatibility is checked with `cargo-semver-checks`.
- Dependency advisories, licenses, duplicate versions, and sources are checked with `cargo-deny`.

## Links

- Crates.io: https://crates.io/crates/win-desktop-utils
- Docs: https://docs.rs/win-desktop-utils
- Repository: https://github.com/funwithcthulhu/win-desktop-utils
- Changelog: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/CHANGELOG.md
- Cookbook: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/cookbook.md

# win-desktop-utils

[![Crates.io](https://img.shields.io/crates/v/win-desktop-utils.svg)](https://crates.io/crates/win-desktop-utils)
[![Docs.rs](https://docs.rs/win-desktop-utils/badge.svg)](https://docs.rs/win-desktop-utils)
[![CI](https://github.com/funwithcthulhu/win-desktop-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/funwithcthulhu/win-desktop-utils/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/win-desktop-utils.svg)](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/LICENSE-MIT)
[![Latest Release](https://img.shields.io/github/v/release/funwithcthulhu/win-desktop-utils)](https://github.com/funwithcthulhu/win-desktop-utils/releases)
[![MSRV](https://img.shields.io/badge/MSRV-1.82-blue)](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/RELEASE.md)

Windows desktop helpers for Rust apps that need the Windows shell without
owning raw Win32 glue.

`win-desktop-utils` wraps the small but fiddly tasks that show up in real
desktop apps: shell opening, Explorer reveal, Recycle Bin moves, shortcuts,
app-data paths, single-instance locks, and elevation prompts. Use the low-level
helpers directly, or start with `DesktopApp` for the common "app identity,
app-data directory, and single-instance guard" workflow.

Good fit:

- Rust GUI, tray, launcher, installer-adjacent, and local utility apps on Windows
- apps that want focused feature flags instead of a large desktop abstraction
- cross-platform apps that want Windows helpers to type-check behind shared code

Not a fit:

- window creation, widgets, rendering, menus, installers, services, or updates
- broad Win32 coverage; use the `windows` crate directly for that

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

This crate is Windows-first. On non-Windows targets, the public API still
compiles and operational helpers return `Error::Unsupported`.

## Installation

```toml
[dependencies]
win-desktop-utils = "0.5"
```

Default features enable the full API. To keep a dependency focused, disable defaults and opt into only the modules you need:

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["paths", "instance"] }
```

For crates that only need these helpers in Windows-specific code, use a
target-specific dependency:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

For cross-platform crates that want the same public symbols available everywhere,
use a normal dependency and handle `Error::Unsupported` on non-Windows targets.

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

## Why Try It

- Small, focused API for common Windows desktop chores.
- Validates inputs before crossing into shell, COM, known-folder, mutex, and
  elevation APIs where practical.
- No background runtime, async executor, telemetry, or global worker.
- Feature flags let apps opt into only the helper groups they use.
- Windows CI checks tests, clippy, examples, docs, package creation, dependency
  policy, public API semver, and feature combinations.
- Non-Windows CI checks that public stubs compile and return
  `Error::Unsupported`.

## Pick The Right Helper

| Need | Start with | Feature | Notes |
| --- | --- | --- | --- |
| App identity, app-data paths, and one-instance startup | `DesktopApp` | `app` | Best first stop for normal apps |
| Per-user config, cache, logs, or state folders | `ensure_local_app_data` / `ensure_roaming_app_data` | `paths` | Creates folders only when using `ensure_*` |
| Current-session or global single-instance behavior | `single_instance` / `SingleInstanceOptions` | `instance` | Keep the guard alive |
| Open files, URLs, folders, Properties, or print verbs | `open_with_default`, `open_url`, `open_with_verb` | `shell` | Uses user shell associations |
| Show a file or folder in Explorer | `reveal_in_explorer` | `shell` | Starts `explorer.exe` |
| Move files or folders to the Recycle Bin | `move_to_recycle_bin` / `move_paths_to_recycle_bin` | `recycle-bin` | Requires absolute existing paths |
| Create `.lnk` or `.url` shortcuts | `create_shortcut` / `create_url_shortcut` | `shortcuts` | Output parent must already exist |
| Check or request administrator elevation | `is_elevated`, `restart_as_admin`, `run_as_admin` | `elevation` | May show UAC and starts another process |

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

Additional guides:

- [`docs/adoption.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/adoption.md): integration notes for common app shapes
- [`docs/feature-flags.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/feature-flags.md): minimal dependency snippets by feature
- [`docs/integrations.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/integrations.md): framework and packaging integration sketches
- [`docs/overhead.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/overhead.md): runtime model, side costs, and dependency surface
- [`docs/which-api.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/which-api.md): pick the right helper for a task
- [`docs/side-effects.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/side-effects.md): user-visible behavior and safety notes
- [`docs/compatibility.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/compatibility.md): OS, Rust, feature, and non-Windows build policy
- [`docs/design.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/design.md): project scope and API acceptance rules
- [`docs/testing.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/testing.md): test structure and expectations
- [`docs/trust.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/trust.md): maintenance, compatibility, and release guarantees

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
- cohesive `DesktopApp` startup flow

See [`examples/README.md`](https://github.com/funwithcthulhu/win-desktop-utils/blob/main/examples/README.md) for expected behavior, side effects, and feature flags for each example.

A tiny companion app repo is available at
[`funwithcthulhu/win-desktop-utils-demo`](https://github.com/funwithcthulhu/win-desktop-utils-demo).
It shows `DesktopApp`, app-data setup, single-instance startup, shell opening,
Explorer reveal, elevation checks, and shortcut creation in one small binary.

Run any example with:

```powershell
cargo run --example single_instance
```

## Why Not Use `windows` Directly?

Use `windows` directly when you need broad Win32 coverage, custom flags, direct
COM object ownership, or APIs outside this crate's scope.

Use `win-desktop-utils` when you want a small, documented layer for common
desktop app chores with validation, Rust-friendly builders, examples, and a
stable public API.

## Alternatives And Scope

This crate is not a GUI framework, installer, updater, service framework, or
cross-platform desktop abstraction. It works well alongside GUI frameworks and
installer tools by handling a small set of Windows desktop tasks that application
code often needs after startup or in response to user actions.

## Error behavior

The crate exposes a small public error type with explicit path-related cases.

Notable error distinctions include:

- `Error::InvalidInput(...)` for empty or malformed input
- `Error::PathNotAbsolute` when an operation requires an absolute path
- `Error::PathDoesNotExist` when an operation requires an existing path
- `Error::WindowsApi { .. }` when a Win32 or shell operation reports failure
- `Error::Io(...)` for underlying I/O failures
- `Error::Unsupported(...)` for Windows operations called on non-Windows targets

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
- On non-Windows targets, public APIs compile and Windows operations return `Error::Unsupported`.

## Quality

The crate includes:

- automated tests for validation and single-instance behavior
- unit tests covering argument quoting and input normalization edge cases
- table-driven validation tests for quoting, identity, URL, verb, and path inputs
- doctest examples in the public modules
- rustdoc lint checks for public documentation quality
- `cargo xtask` automation for docs, feature, package, and release checks
- `cargo xtask release-audit` checks for release metadata and package contents
- ignored manual Windows smoke tests available through `cargo xtask smoke`
- Windows CI via GitHub Actions for MSRV, formatting, tests, clippy, examples, doctests, docs, feature combinations, packaging, dependency policy, and semver checks
- non-Windows CI checks that the public API stubs compile and return unsupported errors
- scheduled CI to catch dependency, runner, and toolchain drift
- documentation link checks for local Markdown links
- docs published on docs.rs

These checks are meant to keep the crate boring in the good way: small API,
documented behavior, predictable side effects, and no surprise runtime.

The minimum supported Rust version is `1.82`, matching the current `windows` crate dependency floor.

## Support Policy

- Supported OS family: Windows 10 and Windows 11.
- Non-Windows targets compile public API stubs that return `Error::Unsupported`.
- Supported Rust: 1.82 and newer.
- Public API compatibility is checked with `cargo-semver-checks`.
- Dependency advisories, licenses, duplicate versions, and sources are checked with `cargo-deny`.

## Links

- Crates.io: https://crates.io/crates/win-desktop-utils
- Docs: https://docs.rs/win-desktop-utils
- Repository: https://github.com/funwithcthulhu/win-desktop-utils
- Changelog: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/CHANGELOG.md
- Cookbook: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/cookbook.md
- API guide: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/which-api.md
- Adoption notes: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/adoption.md
- Compatibility: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/compatibility.md
- Design contract: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/design.md
- Feature flags: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/feature-flags.md
- Integrations: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/integrations.md
- Demo app: https://github.com/funwithcthulhu/win-desktop-utils-demo
- Runtime overhead: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/overhead.md
- Testing guide: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/testing.md
- Trust and maintenance: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/docs/trust.md
- Roadmap: https://github.com/funwithcthulhu/win-desktop-utils/blob/main/ROADMAP.md

# Compatibility

`win-desktop-utils` is Windows-first. It is intended for interactive Windows
desktop apps, tray apps, GUI utilities, installer-adjacent tools, and local
developer utilities.

## Operating Systems

Primary support targets:

- Windows 10
- Windows 11

The crate uses ordinary Win32 shell, COM, known-folder, mutex, and elevation APIs
that are expected to exist on supported desktop Windows versions.

## Rust Version

The minimum supported Rust version is `1.82`.

CI checks the MSRV with:

```powershell
cargo +1.82.0-x86_64-pc-windows-msvc check --all-targets
```

## Non-Windows Builds

Non-Windows targets compile the public API as stubs. Operational helpers return:

```rust
Err(win_desktop_utils::Error::Unsupported(_))
```

This helps libraries and cross-platform applications type-check code paths that
are only exercised on Windows.

If your code only needs the crate on Windows, prefer a target-specific
dependency:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.4"
```

If your code wants the same public symbols available on every target, use a
normal dependency:

```toml
[dependencies]
win-desktop-utils = "0.4"
```

## Feature Flags

Default features enable the full API. For smaller dependency surfaces, disable
defaults and opt into the groups you need:

```toml
[dependencies]
win-desktop-utils = { version = "0.4", default-features = false, features = ["paths", "instance"] }
```

Available features:

- `app`: `DesktopApp` facade for app-data and single-instance startup.
- `paths`: per-user local and roaming app-data helpers.
- `instance`: named-mutex single-instance helpers.
- `shell`: shell opening, URL, Explorer, and shell-verb helpers.
- `recycle-bin`: Recycle Bin move and empty helpers.
- `shortcuts`: `.lnk` and `.url` shortcut helpers.
- `elevation`: elevation detection and shell-based relaunch helpers.

The `app` feature depends on `paths` and `instance`.

## docs.rs

docs.rs builds target the Windows MSVC target so Windows APIs are visible in the
published Rust documentation.

## Interactive Desktop Assumptions

Some APIs depend on interactive Windows desktop state:

- Shell verbs and URL opening depend on user file associations and registered
  handlers.
- Explorer reveal helpers require Explorer to be available.
- Elevation helpers may show UAC prompts.
- Known-folder helpers resolve paths for the current user.
- Global mutexes can be affected by policy or permissions.

These workflows can behave differently in services, scheduled tasks, CI,
containers, remote sessions, or Session 0.

## Package Formats

The crate does not currently provide package-format-specific helpers for MSIX,
MSI, NSIS, WiX, Squirrel, or portable app layouts. The APIs can still be useful
inside apps shipped by those systems, but installation, registration, and update
policy remain the responsibility of the application or installer.

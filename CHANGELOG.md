# Changelog

All notable changes to `win-desktop-utils` are documented here.

## 0.4.0

- Added crate feature flags so consumers can opt into focused API groups.
- Added `DesktopApp` as a convenience facade for app-data and single-instance startup workflows.
- Expanded crate-level docs into a docs.rs-oriented manual with feature flags, workflows, and side-effect notes.
- Added a cookbook with copy-paste recipes for common Windows desktop app tasks.
- Improved crate discoverability metadata with shortcut and Explorer-oriented keywords.
- Added contribution guidelines, security policy, and GitHub issue templates.

## 0.3.0

- Lowered the crate MSRV from Rust 1.94 to Rust 1.82.
- Added `.lnk` shortcut creation with `create_shortcut`, `ShortcutOptions`, and `ShortcutIcon`.
- Added Internet Shortcut creation with `create_url_shortcut`.
- Added arbitrary shell-verb command launching with `run_with_verb` and `run_as_admin`.
- Added shell convenience helpers: `show_properties`, `print_with_default`, and `open_containing_folder`.
- Added silent Recycle Bin emptying helpers with `empty_recycle_bin` and `empty_recycle_bin_for_root`.
- Added `SingleInstanceOptions` and `single_instance_with_options` for builder-style single-instance configuration.
- Added README badges, release notes, shortcut examples, MSRV CI, cargo-deny configuration, and semver-checks CI.

## 0.2.2

- Added `InstanceScope` and `single_instance_with_scope` for choosing current-session or global single-instance mutexes.
- Added `open_with_verb` for launching files and directories with explicit Windows shell verbs.
- Added `move_paths_to_recycle_bin` for validating and recycling multiple paths in one shell operation.
- Updated README, crate docs, examples, and validation coverage for the new APIs.

## 0.2.1

- Modernized Recycle Bin support from `SHFileOperationW` to `IFileOperation`.
- Switched app-data directory resolution to `SHGetKnownFolderPath`.
- Hardened validation for URLs, app IDs, recycle-bin paths, and elevated relaunch arguments.
- Fixed administrator relaunch argument quoting.
- Refreshed README, crate docs, examples, and tests.

## 0.2.0

- Added Windows desktop helpers for shell opening, Explorer reveal, Recycle Bin moves, app-data paths, single-instance locking, and elevation workflows.

# Changelog

All notable changes to `win-desktop-utils` are documented here.

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

# Changelog

All notable changes to `win-desktop-utils` are documented here.

## Unreleased

- Expanded feature checking to cover every pairwise public feature combination.
- Added more table-driven validation and argument quoting tests.
- Added an examples index with expected behavior, side effects, and feature flags.
- Added feature-flag recipe documentation and runtime-overhead notes.
- Polished the crate-level docs for faster API selection.
- Expanded security policy and post-release docs.rs verification guidance.

## 0.5.2 - 2026-05-02

- Centralized shared private Windows helper code for COM apartments, shell execution, argument quoting, string normalization, and wide-string conversion.
- Added ignored manual Windows desktop smoke tests and `cargo xtask smoke`.
- Added `cargo xtask release-audit` for version metadata and package-content checks.
- Added framework integration notes and trust/maintenance documentation.
- Added a README helper decision matrix.
- Added scheduled CI coverage for drift detection.

## 0.5.1 - 2026-05-02

- Added a design contract that documents project scope, API acceptance rules, side-effect rules, and release expectations.
- Added adoption notes for common app shapes without expanding the public API.
- Added a testing guide and test layout documentation.
- Added `cargo xtask` automation for docs, feature, package, and release checks.
- Enabled rustdoc documentation lints in the crate and CI.
- Expanded contribution guidance with explicit criteria for new APIs.

## 0.5.0 - 2026-05-02

- Removed the hard non-Windows compile error and added non-Windows public API stubs that return `Error::Unsupported` for Windows operations.
- Added cross-platform dependency guidance for both target-specific Windows use and cross-platform type-checking.
- Added `docs/which-api.md`, `docs/side-effects.md`, and `docs/compatibility.md`.
- Added a cohesive `examples/desktop_app.rs` startup example.
- Added a project roadmap and code of conduct.
- Expanded README trust and adoption notes with license, release, and MSRV badges, scope guidance, direct `windows` crate comparison, and documentation links.
- Expanded CI coverage with explicit doctests, non-Windows stub checks, and Markdown link checking.

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

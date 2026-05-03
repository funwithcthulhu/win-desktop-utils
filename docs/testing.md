# Testing Guide

The test suite is organized around risk rather than around source file names.

## Test Categories

- Unit tests live beside implementation modules in `src/`.
- Public API validation tests live in `tests/api_validation.rs`.
- Path integration tests live in `tests/basic.rs`.
- Error formatting tests live in `tests/error_display.rs`.
- Non-Windows stub tests live in `src/unsupported.rs` and run in Linux CI.
- Doctests live in rustdoc examples.

## What To Test For New APIs

New public APIs should include tests for:

- empty input
- malformed input
- NUL bytes when text crosses an OS boundary
- relative paths when absolute paths are required
- missing paths when existing paths are required
- non-Windows `Error::Unsupported` behavior
- one side-effect-free success path, when practical

If an API starts another process or shows shell UI, prefer `no_run` docs and
validation tests over CI tests that open windows.

## Local Commands

Use focused commands while developing:

```powershell
cargo test
cargo test --doc --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo xtask docs-check
cargo xtask feature-check
```

Before release, run:

```powershell
cargo xtask release-check
```

For manual Windows desktop smoke coverage, run:

```powershell
cargo xtask smoke
```

This runs ignored tests in `tests/smoke.rs` for contained desktop behavior such
as shortcut creation, Recycle Bin moves, and elevation-state checks. Tests that
open Explorer, open a browser, or trigger UAC require explicit opt-in:

```powershell
$env:WIN_DESKTOP_UTILS_SMOKE_UI='1'
cargo xtask smoke

$env:WIN_DESKTOP_UTILS_SMOKE_ELEVATION='1'
cargo xtask smoke
```

## CI Coverage

Windows CI runs formatting, tests, clippy, examples, docs, no-default feature
checks, single-feature checks, pairwise feature checks, packaging, dependency
policy, semver checks, and release metadata audits. Linux CI checks that the
non-Windows stubs compile, test, and lint with all features. Scheduled CI runs
weekly to catch toolchain, runner, and dependency drift.

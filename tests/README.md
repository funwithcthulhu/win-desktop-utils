# Test Layout

This directory contains public integration tests. Module-private unit tests live
beside implementation code in `src/`.

## Files

- `api_validation.rs`: public API validation and side-effect-light integration
  checks for shell, shortcut, Recycle Bin, elevation, and single-instance helpers.
- `basic.rs`: app-data path integration checks.
- `error_display.rs`: public error display and source behavior.
- `smoke.rs`: ignored manual Windows desktop smoke tests for contained
  side-effect checks.

Non-Windows stub tests live in `src/unsupported.rs` because that module is only
compiled on non-Windows targets.

## Adding Tests

Prefer validation tests for APIs that can launch UI or affect the desktop. Add
side-effecting success tests only when the operation is contained, cleanup is
reliable, and the test can run unattended in CI.

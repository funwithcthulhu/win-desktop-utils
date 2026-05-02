# Contributing

Thanks for improving `win-desktop-utils`.

## Scope

This crate is intentionally small. Contributions should fit common Windows desktop app workflows and keep the public API safe, focused, and easy to explain.

Good fits include:

- shell and Explorer helpers
- shortcut helpers
- app-data and known-folder helpers
- single-instance helpers
- elevation helpers
- documentation, examples, tests, CI, and release automation

Please avoid broad framework features or unrelated Windows APIs unless there is a clear desktop app workflow behind them.

## Local Checks

Run these before opening a PR:

```powershell
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo check --examples
cargo doc --no-deps
cargo package
cargo deny check
```

If you change public API, also run:

```powershell
cargo semver-checks check-release
```

## MSRV

The minimum supported Rust version is `1.82`. Changes should keep:

```powershell
cargo +1.82.0-x86_64-pc-windows-msvc check --all-targets
```

working on Windows.

## Release Notes

User-visible changes should be added to `CHANGELOG.md`. Release steps live in `RELEASE.md`.

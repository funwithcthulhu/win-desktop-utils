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
cargo test --doc --all-features
cargo doc --no-deps
lychee --offline --no-progress README.md CHANGELOG.md CONTRIBUTING.md SECURITY.md RELEASE.md ROADMAP.md CODE_OF_CONDUCT.md docs/cookbook.md docs/which-api.md docs/side-effects.md docs/compatibility.md
cargo package
cargo deny check
```

If you change cross-platform behavior, also run:

```powershell
rustup target add x86_64-unknown-linux-gnu
cargo check --target x86_64-unknown-linux-gnu --all-targets --all-features
cargo check --target x86_64-unknown-linux-gnu --no-default-features
```

The Linux CI job runs the non-Windows unit tests natively.

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

## Conduct

By participating, you agree to follow the project [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

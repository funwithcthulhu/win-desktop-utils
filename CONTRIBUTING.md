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
cargo xtask docs-check
cargo xtask feature-check
cargo xtask release-audit
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo check --examples
cargo package
cargo deny check
```

Before release, run the full local gate:

```powershell
cargo xtask release-check
```

For manual Windows desktop behavior, run:

```powershell
cargo xtask smoke
```

The smoke command is intentionally ignored by default test runs because some
checks can touch the desktop shell. See [`docs/testing.md`](docs/testing.md) for
UI and elevation opt-in variables.

`cargo xtask feature-check` covers the empty feature set, every individual public
feature, and every pairwise public feature combination.

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

## New API Criteria

Before proposing a new public API, check it against [`docs/design.md`](docs/design.md).

A new API should have:

- a common interactive Windows desktop workflow behind it
- a clear feature flag home
- explicit input validation before side effects start
- rustdoc explaining purpose, errors, and side effects
- Markdown docs when the behavior has user-visible consequences
- focused tests, including validation failures
- non-Windows stub behavior returning `Error::Unsupported`
- an example when users are likely to copy the workflow

APIs that only expose another Win32 call without a common workflow are usually
outside this crate's scope.

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

# Trust And Maintenance

This crate is small on purpose. Its main promise is not broad Windows coverage;
it is predictable behavior for common interactive desktop app tasks.

## Compatibility Contract

- Supported OS family: Windows 10 and Windows 11.
- Supported Rust: 1.82 and newer.
- Public API compatibility is checked with `cargo-semver-checks`.
- Non-Windows targets compile public stubs that return `Error::Unsupported`.
- Default features expose the full API; focused feature flags are available for
  smaller dependency surfaces.

## Runtime Behavior Contract

The crate should validate inputs before starting side effects whenever practical.

User-visible behavior is documented instead of hidden:

- shell helpers may open files, folders, browsers, or registered handlers
- Explorer helpers start `explorer.exe`
- Recycle Bin helpers move or permanently delete Recycle Bin contents
- shortcut helpers create or overwrite `.lnk` and `.url` files
- elevation helpers may show UAC and start another process
- single-instance helpers hold a named mutex until the guard is dropped

See [`side-effects.md`](side-effects.md) for the longer behavior guide.

## CI Gates

GitHub Actions checks:

- MSRV compilation on Windows
- formatting
- unit tests, integration tests, and doctests
- clippy with warnings denied
- examples
- rustdoc warnings
- local Markdown links
- feature combinations
- package creation
- dependency advisories, licenses, duplicate versions, and source policy
- public API semver compatibility
- non-Windows stub compilation, tests, and linting

CI also runs on a weekly schedule so dependency, runner, and toolchain drift is
caught even when no code changes are active.

## Local Release Gates

`cargo xtask release-check` runs the full release verification suite. It includes
`cargo xtask release-audit`, which checks:

- `Cargo.toml` and `Cargo.lock` version agreement
- a dated changelog section for the package version
- README install snippets for the current minor line
- package contents for files that should not ship to crates.io

Manual desktop behavior can be checked with:

```powershell
cargo xtask smoke
```

The smoke command runs ignored Windows desktop tests for shortcuts, Recycle Bin
behavior, and elevation state. UI-opening checks are opt-in with environment
variables so automated runs do not unexpectedly open windows or UAC prompts.

## What Is Not Guaranteed

This crate does not guarantee that a user's installed shell handlers support a
given verb, that Explorer is available in every account/session shape, or that
service and scheduled-task environments behave like an interactive desktop. It
also does not own installer, updater, GUI framework, or package-manager policy.

When an application needs lower-level control, use the `windows` crate directly.

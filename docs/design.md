# Design Contract

`win-desktop-utils` should stay small, explicit, and auditable.

This document is the bar for future changes. It exists so the crate can improve
without becoming a grab bag of Windows APIs.

## What This Crate Is

- A focused helper crate for common interactive Windows desktop app workflows.
- A validation layer around shell, shortcut, app-data, single-instance, Recycle
  Bin, and elevation tasks.
- A crate that favors predictable Rust APIs over exposing raw Win32 types.
- A Windows-first crate with non-Windows stubs for cross-platform type-checking.

## What This Crate Is Not

- Not a GUI framework.
- Not an installer, updater, or package manager.
- Not a service framework.
- Not a general Win32 wrapper.
- Not a cross-platform desktop abstraction.
- Not a place for every useful Windows API.

## API Design Rules

New public APIs should satisfy all of these:

- They solve a common desktop app workflow, not just expose an interesting API.
- They have a clear owner feature flag or justify a new one.
- Inputs are validated before side effects start.
- Side effects are documented in rustdoc and Markdown docs when meaningful.
- Errors map to the crate's small public `Error` type.
- The non-Windows behavior is explicit and returns `Error::Unsupported`.
- The API can be explained in a short README bullet.
- The API has focused tests and an example when users are likely to copy it.

## Side-Effect Rules

Side-effecting helpers should be explicit about what they do:

- Launching another process is a side effect.
- Showing UAC is a side effect.
- Opening Explorer or a browser is a side effect.
- Creating or overwriting a shortcut is a side effect.
- Moving files to the Recycle Bin is a side effect.
- Emptying the Recycle Bin is destructive and must stay clearly labeled.

The crate should not hide these behaviors behind vague helper names.

## Dependency Rules

- Keep runtime dependencies minimal.
- Keep Windows bindings behind feature flags.
- Keep Windows bindings target-specific so non-Windows builds do not compile
  Win32 crates unnecessarily.
- Prefer standard library helpers for project tooling unless a dependency earns
  its weight.

## Documentation Rules

Every public API should have:

- a one-sentence purpose
- error behavior
- side-effect notes when relevant
- a doctest or `no_run` example when practical

Broader behavior should live in focused Markdown docs rather than in long README
sections. The README should route users to the right guide.

## Release Rules

Release commits should be exact and auditable:

- version bump in `Cargo.toml` and `Cargo.lock`
- README and docs install snippets updated
- changelog entry dated
- `cargo xtask release-check` passed
- clean working tree before `cargo publish`
- tag created on the exact published commit
- GitHub release created after the tag
- crates.io, docs.rs, GitHub release, and CI confirmed

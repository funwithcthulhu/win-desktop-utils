# Roadmap

This project is intentionally small: practical Windows desktop helpers with
clear behavior, validation, examples, and docs.

## Current Focus

- Keep the public API easy to understand from the README and docs.rs.
- Keep examples copy-pasteable for normal Windows desktop apps.
- Preserve a low dependency surface through feature flags.
- Keep MSRV and public API compatibility checked in CI.
- Keep release metadata, package contents, and manual smoke coverage easy to audit.
- Make cross-platform consumers able to type-check the public API, while keeping
  Windows behavior explicit.

## Candidate Improvements

These are good future directions when they have clear demand and can stay small:

- more shortcut options that map cleanly to stable Shell Link properties
- helper examples for common installers and portable app layouts
- integration examples with common Rust GUI frameworks
- optional helpers for focusing or signaling an existing app instance
- additional known-folder helpers used by desktop apps
- more docs around service and scheduled-task limitations

## Non-Goals

- becoming a GUI framework
- wrapping arbitrary Win32 APIs
- hiding UAC, shell UI, or Recycle Bin side effects
- providing a cross-platform desktop abstraction
- owning installer, updater, or package-manager behavior

## How To Propose Work

Open a GitHub issue with:

- the user workflow
- the Windows API or behavior involved, if known
- expected inputs and side effects
- what should happen on failure
- whether it needs a new feature flag or fits an existing one

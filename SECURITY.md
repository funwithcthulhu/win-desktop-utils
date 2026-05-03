# Security Policy

## Supported Versions

Security fixes are expected to land on the latest published version.

## Reporting A Vulnerability

Please report security issues privately by opening a GitHub security advisory for the repository when available. If that is not available, open a minimal issue asking for a private contact path without including exploit details.

Please include:

- affected version
- affected API or workflow
- Windows version, when relevant
- a minimal reproduction or description
- expected impact

## Dependency Policy

CI runs `cargo deny check` for RustSec advisories, license policy, duplicate dependency warnings, and source policy.

## Desktop Boundary Policy

This crate is not a sandbox, privilege boundary, malware defense, or installer
policy layer. It validates inputs for the workflows it owns and then asks
Windows shell, COM, known-folder, mutex, or elevation APIs to perform the
operation.

Security-sensitive reports are appropriate when behavior in this crate can cause
unexpected command execution, malformed shortcut contents, path validation
bypass, incorrect privilege-boundary documentation, or surprising destructive
behavior beyond the documented Recycle Bin APIs.

Behavior inherited from user file associations, installed shell handlers,
Explorer policy, UAC policy, or service/session configuration may still be
important, but it is usually environment-specific unless the crate is passing
incorrect data to Windows or documenting the behavior incorrectly.

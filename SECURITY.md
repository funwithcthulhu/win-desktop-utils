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

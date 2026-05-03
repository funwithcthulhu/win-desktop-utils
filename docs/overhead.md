# Runtime Overhead

`win-desktop-utils` is mostly a validation and FFI boundary around Windows shell
APIs. Most operations are dominated by the operating system work they ask
Windows to perform.

## Runtime Model

- No background runtime is started by the crate.
- No async executor is created or required.
- No telemetry, network calls, or analytics are performed.
- No global mutable state is owned by the crate.
- Single-instance guards own one Windows mutex handle until dropped.
- Shortcut and Recycle Bin helpers use short-lived STA worker threads for COM
  operations that need apartment-threaded shell APIs.
- Shell and elevation helpers delegate to `ShellExecuteW`.
- App-data helpers call `SHGetKnownFolderPath` and optionally `create_dir_all`.

## Cost Shape

The expensive part is usually outside this crate:

- opening a file or URL depends on the registered handler
- Explorer reveal starts or reuses Explorer
- UAC prompts depend on Windows elevation policy
- shortcut creation depends on COM Shell Link APIs
- Recycle Bin operations depend on the shell and filesystem

The crate performs input validation before these calls where practical, then
returns ordinary Rust `Result` values.

## Dependency Surface

The only runtime dependency is the target-specific optional `windows` crate.
Feature flags control which public modules are exposed, while enabled Windows
features share one Windows binding dependency.

Non-Windows builds do not compile the Windows bindings. They compile public API
stubs that return `Error::Unsupported`.

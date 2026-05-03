# Examples

These examples are intended to be small, copy-pasteable starting points. Some
examples open Windows UI or touch user-visible desktop state; check the behavior
column before running them.

Run an example with:

```powershell
cargo run --example single_instance
```

## Index

| Example | Feature | Behavior |
| --- | --- | --- |
| `desktop_app.rs` | `app` | Creates app-data directories and acquires a single-instance guard. |
| `ensure_paths.rs` | `paths` | Creates local and roaming app-data directories. |
| `paths.rs` | `paths` | Resolves local and roaming app-data paths without opening UI. |
| `single_instance.rs` | `instance` | Acquires a current-session single-instance mutex. |
| `single_instance_global.rs` | `instance` | Acquires a global single-instance mutex. |
| `single_instance_options.rs` | `instance` | Shows builder-style single-instance configuration. |
| `open.rs` | `shell` | Opens URLs, files, or folders through the Windows shell. |
| `reveal.rs` | `shell` | Starts Explorer and selects a path. |
| `run_with_verb.rs` | `elevation` | Starts a command through a shell verb. |
| `is_elevated.rs` | `elevation` | Checks whether the current process is elevated. |
| `restart_as_admin.rs` | `elevation` | May show UAC and start a new elevated process. |
| `shortcuts.rs` | `shortcuts` | Creates `.lnk` and `.url` shortcuts in a temp location. |
| `recycle_bin.rs` | `recycle-bin` | Moves a temp file to the Recycle Bin. |

## Feature-Focused Runs

Most examples can be checked with the full default feature set:

```powershell
cargo check --examples
```

When testing a focused dependency surface, use the feature listed above:

```powershell
cargo run --no-default-features --features paths --example paths
cargo run --no-default-features --features instance --example single_instance
cargo run --no-default-features --features shortcuts --example shortcuts
```

## UI And Side Effects

- Shell and Explorer examples may open windows or registered handlers.
- Elevation examples may show UAC or start another process.
- Shortcut examples write files to a temp location and clean up after themselves.
- Recycle Bin examples create a temp file and move it to the Recycle Bin.
- Path and single-instance examples avoid visible shell UI.

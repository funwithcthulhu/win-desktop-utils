# Which API Should I Use?

Use this guide when you know the desktop task you want, but not the helper name
yet.

## App Startup

| Task | Use | Feature |
| --- | --- | --- |
| Keep app identity, app-data paths, and single-instance setup together | `DesktopApp` | `app` |
| Allow only one instance in the current signed-in session | `single_instance` | `instance` |
| Allow only one instance across all sessions | `single_instance_with_scope` with `InstanceScope::Global` | `instance` |
| Keep single-instance configuration in a builder | `SingleInstanceOptions` | `instance` |
| Store logs, caches, downloads, or machine-local settings | `local_app_data` or `ensure_local_app_data` | `paths` |
| Store small settings that may roam with a domain profile | `roaming_app_data` or `ensure_roaming_app_data` | `paths` |

For most desktop apps, start with `DesktopApp`:

```rust
let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
let _guard = app.single_instance()?.expect("already running");
let local_dir = app.ensure_local_data_dir()?;
# Ok::<(), win_desktop_utils::Error>(())
```

## Shell And Explorer

| Task | Use | Feature |
| --- | --- | --- |
| Open a file or folder with the default app | `open_with_default` | `shell` |
| Open a file or folder with a shell verb such as `properties` or `print` | `open_with_verb` | `shell` |
| Show a file or folder Properties dialog | `show_properties` | `shell` |
| Print a file through its registered handler | `print_with_default` | `shell` |
| Open a web URL or registered URI scheme | `open_url` | `shell` |
| Select an existing path in Explorer | `reveal_in_explorer` | `shell` |
| Open the folder containing an existing path | `open_containing_folder` | `shell` |

Use `open_with_default` for normal user-facing launch behavior. Use
`open_with_verb` when you intentionally want a shell verb and are prepared for
that verb to depend on file associations and the user's installed software.

## Recycle Bin

| Task | Use | Feature |
| --- | --- | --- |
| Send one absolute existing path to the Recycle Bin | `move_to_recycle_bin` | `recycle-bin` |
| Send several absolute existing paths in one shell operation | `move_paths_to_recycle_bin` | `recycle-bin` |
| Empty the Recycle Bin for all drives | `empty_recycle_bin` | `recycle-bin` |
| Empty the Recycle Bin for one drive root | `empty_recycle_bin_for_root` | `recycle-bin` |

Prefer `move_paths_to_recycle_bin` when deleting a batch. It validates every path
before starting the shell operation, which avoids partially validating input in
your application code.

## Shortcuts

| Task | Use | Feature |
| --- | --- | --- |
| Create or overwrite a Windows `.lnk` shortcut | `create_shortcut` | `shortcuts` |
| Configure `.lnk` arguments, working directory, icon, or description | `ShortcutOptions` | `shortcuts` |
| Configure a shortcut icon resource | `ShortcutIcon` | `shortcuts` |
| Create or overwrite an Internet Shortcut `.url` file | `create_url_shortcut` | `shortcuts` |

Use `.lnk` shortcuts for local applications, tools, and launch targets. Use
`.url` shortcuts for web links or registered URI schemes.

## Elevation

| Task | Use | Feature |
| --- | --- | --- |
| Check whether the current process is elevated | `is_elevated` | `elevation` |
| Relaunch the current executable as administrator | `restart_as_admin` | `elevation` |
| Launch another executable as administrator | `run_as_admin` | `elevation` |
| Launch another executable with a specific shell verb | `run_with_verb` | `elevation` |

Use elevation helpers only at clear user-driven boundaries. They may show UAC and
they start another process; they do not silently transform the current process
into an elevated one.

## Feature Set Examples

Small app startup helper:

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["app"] }
```

Only shell opening helpers:

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["shell"] }
```

Cross-platform app that calls Windows helpers only from Windows-specific code:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

Cross-platform app that wants the public API to type-check everywhere:

```toml
[dependencies]
win-desktop-utils = "0.5"
```

On non-Windows targets, Windows operations return `Error::Unsupported`.

## When This Crate Is Not The Right Layer

Use the `windows` crate directly when you need Win32 APIs outside this crate's
small desktop-app scope, need custom flags not exposed here, or need direct COM
object ownership.

Use a GUI framework when you need windows, controls, rendering, menus, event
loops, or native widgets. This crate is deliberately not a GUI toolkit.

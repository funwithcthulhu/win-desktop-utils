# Side Effects And Safety Notes

Most helpers in this crate are thin, validated wrappers around Windows desktop
behavior. They intentionally do real desktop work, so callers should treat them
as user-visible operations.

## Shell Launching

The shell helpers may start another process, focus another window, or show UI.
The exact handler depends on Windows file associations, installed applications,
registered shell verbs, default browser settings, and user policy.

- `open_with_default` uses the target path's default handler.
- `open_with_verb` uses a requested shell verb such as `open`, `edit`,
  `print`, or `properties`.
- `show_properties` and `print_with_default` are shell-verb convenience wrappers.
- `open_url` delegates the URL or URI scheme to the Windows shell.
- `reveal_in_explorer` launches `explorer.exe`.
- `open_containing_folder` opens a target's parent directory.

Do not call shell launch helpers from background loops or unattended cleanup
jobs. They are best used in direct response to user action.

## Elevation

Elevation helpers may show a UAC prompt and start a separate process.

- `is_elevated` checks the current process state.
- `restart_as_admin` starts a new elevated copy of the current executable.
- `run_as_admin` starts another executable with the `runas` shell verb.
- `run_with_verb` starts another executable with the verb you provide.

`restart_as_admin` does not terminate the current process. If your app should
exit after a successful relaunch request, do that explicitly in your own code.

## Recycle Bin

Recycle Bin helpers use the Windows shell to request recycle-bin behavior.

- `move_to_recycle_bin` and `move_paths_to_recycle_bin` require absolute,
  existing paths without NUL bytes.
- `move_paths_to_recycle_bin` validates all paths before starting the shell
  operation.
- `empty_recycle_bin` and `empty_recycle_bin_for_root` permanently empty Recycle
  Bin contents without shell UI.

Emptying the Recycle Bin cannot be undone through this crate. Prefer an explicit
user confirmation in application code before calling an emptying helper.

## Shortcuts

`create_shortcut` and `create_url_shortcut` create or overwrite files at the
requested output path.

- `.lnk` shortcut creation requires an absolute `.lnk` path, an existing parent
  directory, and an existing absolute target path without NUL bytes.
- `.url` shortcut creation requires an absolute `.url` path whose parent is an
  existing directory and rejects line breaks in the URL to avoid malformed
  shortcut content.
- Shortcut arguments are stored in the shortcut file; avoid putting secrets in
  them.

## App Data

App-data helpers resolve Windows known folders for the current user.

- Local app data is a good fit for logs, caches, downloaded assets, and
  machine-local settings.
- Roaming app data is a good fit for small user preferences that may roam with a
  domain profile.
- `ensure_*` helpers create directories with `std::fs::create_dir_all`.

Choose stable app names and company names. Changing them changes the app-data
directory path.

## Single Instance

Single-instance helpers use named Windows mutexes.

- `InstanceScope::CurrentSession` uses the `Local\` namespace.
- `InstanceScope::Global` uses the `Global\` namespace and can require policy or
  permissions in restricted environments.
- Backslashes are rejected in `app_id` because Windows uses them as namespace
  separators for named kernel objects.
- The returned `InstanceGuard` must stay alive for as long as the process should
  hold the lock.

These helpers do not activate, message, or focus an already-running instance.
They only report whether the lock was acquired.

## Services, Scheduled Tasks, And Session 0

This crate targets interactive desktop applications. Shell UI, Explorer, default
handlers, UAC, and per-user known folders can behave differently from Windows
services, scheduled tasks, non-interactive sessions, and Session 0.

For service code, prefer explicit service-safe APIs and test in the same account
and session model used in production.

## Non-Windows Targets

On non-Windows targets, public APIs compile but Windows operations return
`Error::Unsupported`. This is meant to make cross-platform crates easier to
type-check; it is not a cross-platform desktop abstraction.

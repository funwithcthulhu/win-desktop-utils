# win-desktop-utils

Windows-first desktop utility helpers for Rust apps.

Published on crates.io: https://crates.io/crates/win-desktop-utils
API docs: https://docs.rs/win-desktop-utils

## What this crate does

`win-desktop-utils` provides small, focused helpers for common Windows desktop-app tasks:

- open files and directories with the default shell handler
- open URLs
- reveal files in Explorer
- send files or directories to the Recycle Bin
- enforce single-instance behavior with a named mutex
- resolve per-user roaming and local app-data paths
- check elevation status and relaunch as admin

This crate is intended for Windows desktop applications and utilities. Some functions launch external shell behavior, and elevation helpers may trigger UAC prompts.

## Current API

- `open_with_default(path)`
- `open_url(url)`
- `reveal_in_explorer(path)`
- `move_to_recycle_bin(path)`
- `single_instance(app_id)`
- `roaming_app_data(app_name)`
- `local_app_data(app_name)`
- `ensure_roaming_app_data(app_name)`
- `ensure_local_app_data(app_name)`
- `is_elevated()`
- `restart_as_admin(args)`

## Example

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    win_desktop_utils::open_url("https://www.rust-lang.org")?;

    let local = win_desktop_utils::ensure_local_app_data("demo-app")?;
    println!("local app dir: {}", local.display());

    match win_desktop_utils::single_instance("demo-app")? {
        Some(_guard) => println!("first instance"),
        None => println!("already running"),
    }

    Ok(())
}
```

## Behavior notes

- `open_with_default` and `open_url` delegate to the Windows shell.
- `reveal_in_explorer` starts `explorer.exe` and asks it to select the provided path.
- `move_to_recycle_bin` requires an absolute path and returns an error if the path does not exist.
- `single_instance` uses a `Local\...` named mutex, so it is scoped to the current Windows session.
- `restart_as_admin` starts a new elevated instance of the current executable and does not terminate the current process.

## Status

Published crate. Early-stage but usable.

# win-desktop-utils

Windows-first desktop utility helpers for Rust apps.

Published on crates.io: https://crates.io/crates/win-desktop-utils  
API docs: https://docs.rs/win-desktop-utils

## Current features

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

## Status

Published crate. Windows-focused.

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

## Notes

- This crate is intended for Windows desktop tools.
- Some functions launch external Windows shell behavior.
- Elevation helpers may trigger UAC prompts.
- `move_to_recycle_bin` requires an absolute path.

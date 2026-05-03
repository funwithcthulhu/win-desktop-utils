# Feature Flags

Default features enable the full public API. For smaller dependency surfaces,
disable defaults and opt into only the workflow groups your app uses.

## Full API

```toml
[dependencies]
win-desktop-utils = "0.5"
```

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::new("Demo App")?;
    let _guard = app.single_instance()?;

    Ok(())
}
```

## `app`

Use `app` when startup code wants app identity, app-data paths, and
single-instance configuration in one place.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["app"] }
```

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
    let local = app.ensure_local_data_dir()?;

    assert!(local.ends_with(r"Acme\Editor"));
    Ok(())
}
```

`app` depends on `paths` and `instance`.

## `paths`

Use `paths` for per-user local and roaming app-data directories.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["paths"] }
```

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let local = win_desktop_utils::ensure_local_app_data("Demo App")?;

    assert!(local.ends_with("Demo App"));
    Ok(())
}
```

## `instance`

Use `instance` for named mutex single-instance behavior.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["instance"] }
```

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let guard = win_desktop_utils::single_instance("demo-app")?;
    if guard.is_none() {
        return Ok(());
    }

    Ok(())
}
```

## `shell`

Use `shell` for opening files, folders, URLs, Explorer selections, and shell
verbs.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["shell"] }
```

```rust,no_run
fn main() -> Result<(), win_desktop_utils::Error> {
    win_desktop_utils::open_url("https://www.rust-lang.org")?;
    win_desktop_utils::show_properties(r"C:\Windows\notepad.exe")?;

    Ok(())
}
```

## `recycle-bin`

Use `recycle-bin` for Recycle Bin moves and silent Recycle Bin emptying.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["recycle-bin"] }
```

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir()?.join("temporary-file.txt");

    std::fs::write(&path, "temporary")?;
    win_desktop_utils::move_to_recycle_bin(path)?;

    Ok(())
}
```

## `shortcuts`

Use `shortcuts` for Windows `.lnk` and Internet Shortcut `.url` files.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["shortcuts"] }
```

```rust,no_run
fn main() -> Result<(), win_desktop_utils::Error> {
    let shortcut = std::env::temp_dir().join("notepad.lnk");
    let options = win_desktop_utils::ShortcutOptions::new()
        .description("Open Notepad");

    win_desktop_utils::create_shortcut(shortcut, r"C:\Windows\notepad.exe", &options)?;

    Ok(())
}
```

## `elevation`

Use `elevation` for elevation checks and shell-based elevated process launches.

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["elevation"] }
```

```rust,no_run
use std::ffi::OsString;

fn main() -> Result<(), win_desktop_utils::Error> {
    if !win_desktop_utils::is_elevated()? {
        win_desktop_utils::restart_as_admin(&[OsString::from("--elevated")])?;
    }

    Ok(())
}
```

## Cross-Platform Layouts

Use a target-specific dependency when only Windows code calls the helpers:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

Use a normal dependency when shared code wants the public API to type-check on
every target:

```toml
[dependencies]
win-desktop-utils = "0.5"
```

On non-Windows targets, operational helpers return `Error::Unsupported`.

# Cookbook

Copy-paste recipes for common Windows desktop app tasks.

## Start A Single-Instance App

```rust
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::new("my-app")?;

    let _guard = match app.single_instance()? {
        Some(guard) => guard,
        None => {
            println!("already running");
            return Ok(());
        }
    };

    Ok(())
}
```

## Create Local App Data

Use local app data for machine-local cache, logs, downloaded assets, and settings that should not roam.

```rust
let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
let dir = app.ensure_local_data_dir()?;
let config = dir.join("settings.json");

std::fs::write(config, "{}")?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Create Roaming App Data

Use roaming app data for small user preferences that may roam with a domain profile.

```rust
let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
let dir = app.ensure_roaming_data_dir()?;

println!("roaming settings live in {}", dir.display());
# Ok::<(), win_desktop_utils::Error>(())
```

## Open A File Or Folder

```rust,no_run
win_desktop_utils::open_with_default(r"C:\Windows\notepad.exe")?;
win_desktop_utils::open_containing_folder(r"C:\Windows\notepad.exe")?;
win_desktop_utils::show_properties(r"C:\Windows\notepad.exe")?;
# Ok::<(), win_desktop_utils::Error>(())
```

## Open A URL

```rust,no_run
win_desktop_utils::open_url("https://www.rust-lang.org")?;
# Ok::<(), win_desktop_utils::Error>(())
```

## Create A Windows Shortcut

```rust,no_run
let shortcut = std::env::current_dir()?.join("notepad.lnk");
let options = win_desktop_utils::ShortcutOptions::new()
    .description("Open Notepad")
    .working_directory(r"C:\Windows")
    .icon(r"C:\Windows\notepad.exe", 0);

win_desktop_utils::create_shortcut(&shortcut, r"C:\Windows\notepad.exe", &options)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Create A URL Shortcut

```rust
let shortcut = std::env::temp_dir().join(format!(
    "win-desktop-utils-rust-docs-{}.url",
    std::process::id()
));

win_desktop_utils::create_url_shortcut(&shortcut, "https://doc.rust-lang.org/std/")?;
std::fs::remove_file(shortcut)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Relaunch As Administrator

```rust,no_run
use std::ffi::OsString;

if !win_desktop_utils::is_elevated()? {
    let args = [OsString::from("--elevated")];
    win_desktop_utils::restart_as_admin(&args)?;
}
# Ok::<(), win_desktop_utils::Error>(())
```

## Run Another Command As Administrator

```rust,no_run
use std::ffi::OsString;

let args = [OsString::from("/c"), OsString::from("echo elevated")];
win_desktop_utils::run_as_admin("cmd.exe", &args)?;
# Ok::<(), win_desktop_utils::Error>(())
```

## Move Files To The Recycle Bin

```rust,no_run
let first = std::env::current_dir()?.join("temporary-a.txt");
let second = std::env::current_dir()?.join("temporary-b.txt");
std::fs::write(&first, "temporary")?;
std::fs::write(&second, "temporary")?;

win_desktop_utils::move_paths_to_recycle_bin([&first, &second])?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Use A Small Feature Set

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["paths", "instance"] }
```

```rust
let local = win_desktop_utils::ensure_local_app_data("my-app")?;
let _guard = win_desktop_utils::single_instance("my-app")?;
# Ok::<(), win_desktop_utils::Error>(())
```

## Use The Crate From A Cross-Platform App

If only your Windows-specific module needs these helpers, make the dependency
target-specific:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

If you want the public symbols to type-check on every target, keep a normal
dependency and handle `Error::Unsupported` when running outside Windows:

```rust
match win_desktop_utils::open_url("https://www.rust-lang.org") {
    Ok(()) => {}
    Err(win_desktop_utils::Error::Unsupported(_)) => {
        eprintln!("open_url is only available on Windows");
    }
    Err(err) => return Err(err),
}
# Ok::<(), win_desktop_utils::Error>(())
```

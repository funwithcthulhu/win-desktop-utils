# Adoption Notes

These examples show where `win-desktop-utils` usually fits in real application
shapes. They are intentionally integration notes, not new feature proposals.

## Cross-Platform Application Layout

Keep Windows desktop behavior in a Windows-only module when the rest of the app
is cross-platform:

```rust
#[cfg(windows)]
fn prepare_desktop_app() -> win_desktop_utils::Result<()> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
    let _guard = app.single_instance()?.expect("already running");
    let _local_dir = app.ensure_local_data_dir()?;
    Ok(())
}

#[cfg(not(windows))]
fn prepare_desktop_app() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
```

Use a target-specific dependency for this shape:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

## Tauri-Style Startup

Use `DesktopApp` before or during the app setup hook. Keep the returned
`InstanceGuard` in application state for the lifetime of the process.

```rust,no_run
fn prepare_startup() -> Result<Option<win_desktop_utils::InstanceGuard>, win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Tauri App")?;
    app.ensure_local_data_dir()?;
    app.single_instance()
}
```

Handle `Ok(None)` with your app's own policy: show a message, exit quietly, or
signal the existing instance.

## eframe Or egui Startup

Acquire the single-instance guard before starting the GUI event loop, then keep
it alive beside your app state:

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "egui Tool")?;
    let _guard = match app.single_instance()? {
        Some(guard) => guard,
        None => return Ok(()),
    };

    let _local_dir = app.ensure_local_data_dir()?;
    // Start eframe here.
    Ok(())
}
```

## Tray App Startup

Tray apps often need stable app-data paths and one running instance:

```rust,no_run
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Tray Monitor")?;
    let _guard = match app.single_instance()? {
        Some(guard) => guard,
        None => return Ok(()),
    };

    let log_dir = app.ensure_local_data_dir()?.join("logs");
    std::fs::create_dir_all(log_dir)?;

    // Start tray icon and message loop here.
    Ok(())
}
```

## Installer Shortcut Or App-Created Shortcut?

Prefer installer-created shortcuts when:

- the shortcut belongs in Start Menu or Desktop during installation
- the shortcut should be removed by uninstall
- installation policy controls location and permissions

Use `create_shortcut` when:

- the user chooses a shortcut location inside the running app
- a portable app creates a shortcut in a user-owned folder
- the shortcut is part of app state rather than install state

## Service Or Scheduled Task Code

This crate targets interactive desktop behavior. For services and scheduled
tasks, avoid shell UI and Explorer helpers unless you have tested the exact
session and account model. See [`side-effects.md`](side-effects.md) for details.

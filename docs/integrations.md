# Integration Notes

These notes show where `win-desktop-utils` usually sits inside application
startup code. They are glue examples, not proposals to add framework-specific
APIs to this crate.

## General Pattern

A typical desktop app can do this before the main UI event loop starts:

1. Build a `DesktopApp` identity.
2. Acquire and store the single-instance guard.
3. Create or resolve app-data directories.
4. Start the framework runtime.

Keep the returned `InstanceGuard` alive for as long as the app should own the
single-instance lock.

```rust,no_run
struct DesktopState {
    guard: win_desktop_utils::InstanceGuard,
    local_data_dir: std::path::PathBuf,
}

fn prepare_desktop() -> Result<Option<DesktopState>, win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
    let Some(guard) = app.single_instance()? else {
        return Ok(None);
    };

    Ok(Some(DesktopState {
        guard,
        local_data_dir: app.ensure_local_data_dir()?,
    }))
}
```

## Dependency Shape

Use the default feature set when application startup, shell actions, shortcuts,
and elevation all live in one app crate:

```toml
[dependencies]
win-desktop-utils = "0.5"
```

Use focused features when a small helper crate owns only one workflow:

```toml
[dependencies]
win-desktop-utils = { version = "0.5", default-features = false, features = ["app", "shell"] }
```

## Tauri

Prepare desktop state before `run`. Keep the guard in a local binding that
lives until `run` returns, and manage only the paths or lightweight state Tauri
commands need.

```rust,no_run
struct AppPaths {
    local_data_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(desktop) = prepare_desktop()? else {
        return Ok(());
    };

    let _guard = desktop.guard;
    let paths = AppPaths {
        local_data_dir: desktop.local_data_dir,
    };

    tauri::Builder::default()
        .setup(move |app| {
            app.manage(paths);
            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}
```

Commands can then use shell helpers for explicit user actions:

```rust,no_run
#[tauri::command]
fn open_docs() -> Result<(), String> {
    win_desktop_utils::open_url("https://docs.rs/win-desktop-utils")
        .map_err(|err| err.to_string())
}
```

## eframe Or egui

Acquire the guard before `run_native`, then move it into the app struct or hold
it beside the app state.

```rust,no_run
struct AppState {
    _guard: win_desktop_utils::InstanceGuard,
    local_data_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(state) = prepare_desktop()? else {
        return Ok(());
    };

    // Move `state` into your eframe app constructor:
    // eframe::run_native("Editor", options, Box::new(|_| Ok(Box::new(App::new(state)))))?;
    Ok(())
}
```

## iced

Prepare desktop state before launching the application, then keep the guard in
your application model.

```rust,no_run
struct Model {
    _guard: win_desktop_utils::InstanceGuard,
    local_data_dir: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(model) = prepare_desktop()? else {
        return Ok(());
    };

    // Pass `model` to your iced application initialization.
    Ok(())
}
```

## Slint

Resolve desktop state before creating the main window. Keep the guard in Rust
state that lives at least as long as the UI.

```rust,no_run
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(desktop) = prepare_desktop()? else {
        return Ok(());
    };

    // Create the Slint UI and keep `desktop` alive beside it.
    let _desktop = desktop;
    Ok(())
}
```

## Tray Apps

Tray apps often have no obvious main window, so single-instance behavior and a
known local data directory are usually startup concerns.

```rust,no_run
fn main() -> Result<(), win_desktop_utils::Error> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Tray Monitor")?;
    let Some(_guard) = app.single_instance()? else {
        return Ok(());
    };

    let log_dir = app.ensure_local_data_dir()?.join("logs");
    std::fs::create_dir_all(log_dir)?;

    // Start the tray icon and message loop here.
    Ok(())
}
```

Use `open_containing_folder`, `open_url`, or `show_properties` from tray menu
callbacks only after the user explicitly asks for those shell actions.

## Portable Apps

Portable apps can still use this crate for user-driven shell behavior such as
opening folders, creating shortcuts in user-selected locations, or enforcing a
single running instance. Be careful with app-data helpers if your app promises
not to write outside its portable directory.

## Installer-Adjacent Code

Prefer installer-owned shortcuts for Start Menu and Desktop entries created at
install time. Use `create_shortcut` from the app when the user chooses a
shortcut location or when the shortcut belongs to user data rather than install
state.

For small installer-adjacent command-line helpers, keep shell side effects behind
clear subcommands:

```rust,no_run
fn create_docs_shortcut(shortcut: std::path::PathBuf) -> win_desktop_utils::Result<()> {
    win_desktop_utils::create_url_shortcut(
        shortcut,
        "https://docs.rs/win-desktop-utils",
    )
}
```

## Cross-Platform Crates

Use a Windows-only dependency when only Windows code calls the helpers:

```toml
[target.'cfg(windows)'.dependencies]
win-desktop-utils = "0.5"
```

Use a normal dependency when shared code wants the public symbols to type-check
on every target:

```toml
[dependencies]
win-desktop-utils = "0.5"
```

On non-Windows targets, operational helpers return `Error::Unsupported`.

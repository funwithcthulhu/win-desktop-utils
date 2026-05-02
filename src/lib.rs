//! Windows desktop helpers for Rust apps.
//!
//! `win-desktop-utils` wraps a focused set of Windows desktop chores that many
//! GUI, tray, installer-adjacent, and local utility apps need, while keeping raw
//! Win32 shell, shortcut, mutex, known-folder, and elevation APIs out of the
//! application code that uses them.
//!
//! The low-level helpers stay available as small functions, and [`DesktopApp`]
//! provides a friendlier facade for common app startup and app-data workflows.
//!
//! # When To Use This
//!
//! Use this crate when a Windows desktop app needs to:
//!
//! - [`open_with_default`]
//! - [`open_with_verb`]
//! - [`show_properties`]
//! - [`print_with_default`]
//! - [`open_url`]
//! - [`reveal_in_explorer`]
//! - [`open_containing_folder`]
//! - [`move_to_recycle_bin`]
//! - [`move_paths_to_recycle_bin`]
//! - [`empty_recycle_bin`]
//! - [`empty_recycle_bin_for_root`]
//! - [`create_shortcut`]
//! - [`create_url_shortcut`]
//! - [`single_instance`]
//! - [`single_instance_with_scope`]
//! - [`single_instance_with_options`]
//! - [`roaming_app_data`]
//! - [`local_app_data`]
//! - [`ensure_roaming_app_data`]
//! - [`ensure_local_app_data`]
//! - [`is_elevated`]
//! - [`restart_as_admin`]
//! - [`run_as_admin`]
//! - [`run_with_verb`]
//! - [`InstanceScope`]
//! - [`SingleInstanceOptions`]
//! - [`DesktopApp`]
//! - [`ShortcutOptions`]
//! - [`ShortcutIcon`]
//!
//! This crate supports Windows only.
//!
//! # Quick Start
//!
//! ```
//! fn main() -> Result<(), win_desktop_utils::Error> {
//!     let app = win_desktop_utils::DesktopApp::new(format!(
//!         "demo-app-{}",
//!         std::process::id()
//!     ))?;
//!
//!     let _guard = match app.single_instance()? {
//!         Some(guard) => guard,
//!         None => {
//!             println!("already running");
//!             return Ok(());
//!         }
//!     };
//!
//!     let local = app.ensure_local_data_dir()?;
//!     assert!(local.exists());
//!
//!     Ok(())
//! }
//! ```
//!
//! # Feature Flags
//!
//! Default features enable the full API. Consumers can opt out and select only
//! the modules they need:
//!
//! ```toml
//! [dependencies]
//! win-desktop-utils = { version = "0.3", default-features = false, features = ["paths", "instance"] }
//! ```
//!
//! Available features:
//!
//! - `app`: [`DesktopApp`] facade for app-data and single-instance startup.
//! - `paths`: per-user local and roaming app-data helpers.
//! - `instance`: named-mutex single-instance helpers.
//! - `shell`: shell opening, URL, Explorer, and shell-verb helpers.
//! - `recycle-bin`: Recycle Bin move and empty helpers.
//! - `shortcuts`: `.lnk` and `.url` shortcut helpers.
//! - `elevation`: elevation detection and shell-based relaunch helpers.
//!
//! The default feature set is intentionally broad for convenience. Feature flags
//! control this crate's public modules; the underlying `windows` dependency uses
//! one shared set of Win32 bindings when any Windows API feature is enabled.
//!
//! # Common Workflows
//!
//! Startup guard plus app-data directory:
//!
//! ```
//! let app = win_desktop_utils::DesktopApp::new(format!(
//!     "workflow-demo-{}",
//!     std::process::id()
//! ))?;
//! let _guard = app.single_instance()?.expect("first instance");
//! let config_dir = app.ensure_local_data_dir()?;
//! assert!(config_dir.exists());
//! # Ok::<(), win_desktop_utils::Error>(())
//! ```
//!
//! Create a shortcut:
//!
//! ```no_run
//! let shortcut = std::env::current_dir()?.join("notepad.lnk");
//! let options = win_desktop_utils::ShortcutOptions::new()
//!     .description("Open Notepad");
//! win_desktop_utils::create_shortcut(&shortcut, r"C:\Windows\notepad.exe", &options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Relaunch the current executable as administrator:
//!
//! ```no_run
//! use std::ffi::OsString;
//!
//! if !win_desktop_utils::is_elevated()? {
//!     win_desktop_utils::restart_as_admin(&[OsString::from("--elevated")])?;
//! }
//! # Ok::<(), win_desktop_utils::Error>(())
//! ```
//!
//! # Behavior And Side Effects
//!
//! - [`open_with_default`] requires a non-empty existing path.
//! - [`open_with_verb`] uses `ShellExecuteW` with the requested shell verb.
//! - [`show_properties`] and [`print_with_default`] are small shell-verb wrappers.
//! - [`open_url`] trims surrounding whitespace before delegating to the Windows shell.
//! - [`reveal_in_explorer`] requires an existing path and launches `explorer.exe`.
//! - [`open_containing_folder`] opens the existing path's parent directory.
//! - [`move_to_recycle_bin`] requires an absolute existing path and uses `IFileOperation`
//!   on a dedicated STA thread for recycle-bin behavior.
//! - [`move_paths_to_recycle_bin`] validates all paths before starting one shell
//!   recycle-bin operation.
//! - [`empty_recycle_bin`] permanently empties the Recycle Bin without showing shell UI.
//! - [`create_shortcut`] uses `IShellLinkW` on a dedicated STA thread.
//! - [`roaming_app_data`] and [`local_app_data`] resolve their base directories via
//!   `SHGetKnownFolderPath`.
//! - [`single_instance`] uses a `Local\...` named mutex, so the lock is scoped to the
//!   current Windows session, and `app_id` cannot contain backslashes.
//! - [`single_instance_with_scope`] can opt into either the current-session or global
//!   mutex namespace.
//! - [`restart_as_admin`] starts a new elevated instance of the current executable,
//!   does not terminate the current process, and rejects arguments containing NUL bytes.
//! - [`run_as_admin`] and [`run_with_verb`] launch arbitrary commands through
//!   `ShellExecuteW`.
//!
//! # Stability
//!
//! The minimum supported Rust version is 1.82. Public API compatibility is checked
//! in CI with `cargo-semver-checks`, and dependency policy is checked with
//! `cargo-deny`.

#[cfg(not(windows))]
compile_error!("win-desktop-utils supports Windows only.");

#[cfg(all(windows, feature = "app"))]
pub mod app;
#[cfg(all(windows, feature = "elevation"))]
pub mod elevation;
#[cfg(windows)]
pub mod error;
#[cfg(all(windows, feature = "instance"))]
pub mod instance;
#[cfg(all(windows, feature = "paths"))]
pub mod paths;
#[cfg(all(windows, any(feature = "shell", feature = "recycle-bin")))]
pub mod shell;
#[cfg(all(windows, feature = "shortcuts"))]
pub mod shortcuts;

#[cfg(windows)]
pub use error::{Error, Result};

#[cfg(all(windows, feature = "app"))]
pub use app::DesktopApp;
#[cfg(all(windows, feature = "elevation"))]
pub use elevation::{is_elevated, restart_as_admin, run_as_admin, run_with_verb};
#[cfg(all(windows, feature = "instance"))]
pub use instance::{
    single_instance, single_instance_with_options, single_instance_with_scope, InstanceGuard,
    InstanceScope, SingleInstanceOptions,
};
#[cfg(all(windows, feature = "paths"))]
pub use paths::{ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data};
#[cfg(all(windows, feature = "recycle-bin"))]
pub use shell::{
    empty_recycle_bin, empty_recycle_bin_for_root, move_paths_to_recycle_bin, move_to_recycle_bin,
};
#[cfg(all(windows, feature = "shell"))]
pub use shell::{
    open_containing_folder, open_url, open_with_default, open_with_verb, print_with_default,
    reveal_in_explorer, show_properties,
};
#[cfg(all(windows, feature = "shortcuts"))]
pub use shortcuts::{create_shortcut, create_url_shortcut, ShortcutIcon, ShortcutOptions};

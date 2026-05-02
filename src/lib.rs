//! Windows-first desktop utility helpers for Rust apps.
//!
//! `win-desktop-utils` provides small, focused helpers for common Windows desktop-app
//! tasks without forcing consumers to work directly with raw Win32 shell, mutex, and
//! known-folder APIs.
//!
//! # Scope
//!
//! This crate currently provides helpers for:
//!
//! - opening files and directories with the default shell handler
//! - opening files and directories with explicit shell verbs
//! - opening URLs
//! - revealing items in Explorer
//! - sending files or directories to the Recycle Bin
//! - creating Windows `.lnk` and `.url` shortcuts
//! - enforcing single-instance behavior
//! - resolving per-user app-data directories
//! - creating per-user app-data directories if needed
//! - checking elevation and relaunching as admin
//!
//! This crate is intended for Windows desktop applications and utilities.
//! Some functions launch external shell behavior or may trigger a UAC prompt.
//! This crate supports Windows only.
//!
//! # Current API
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
//! - [`ShortcutOptions`]
//! - [`ShortcutIcon`]
//!
//! # Example
//!
//! ```
//! fn main() -> Result<(), win_desktop_utils::Error> {
//!     let _guard = match win_desktop_utils::single_instance("demo-app")? {
//!         Some(guard) => guard,
//!         None => {
//!             println!("already running");
//!             return Ok(());
//!         }
//!     };
//!
//!     let local = win_desktop_utils::ensure_local_app_data("demo-app")?;
//!     assert!(local.ends_with("demo-app"));
//!
//!     Ok(())
//! }
//! ```
//!
//! # Behavior Notes
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

#[cfg(not(windows))]
compile_error!("win-desktop-utils supports Windows only.");

#[cfg(windows)]
pub mod elevation;
#[cfg(windows)]
pub mod error;
#[cfg(windows)]
pub mod instance;
#[cfg(windows)]
pub mod paths;
#[cfg(windows)]
pub mod shell;
#[cfg(windows)]
pub mod shortcuts;

#[cfg(windows)]
pub use error::{Error, Result};

#[cfg(windows)]
pub use elevation::{is_elevated, restart_as_admin, run_as_admin, run_with_verb};
#[cfg(windows)]
pub use instance::{
    single_instance, single_instance_with_options, single_instance_with_scope, InstanceGuard,
    InstanceScope, SingleInstanceOptions,
};
#[cfg(windows)]
pub use paths::{ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data};
#[cfg(windows)]
pub use shell::{
    empty_recycle_bin, empty_recycle_bin_for_root, move_paths_to_recycle_bin, move_to_recycle_bin,
    open_containing_folder, open_url, open_with_default, open_with_verb, print_with_default,
    reveal_in_explorer, show_properties,
};
#[cfg(windows)]
pub use shortcuts::{create_shortcut, create_url_shortcut, ShortcutIcon, ShortcutOptions};

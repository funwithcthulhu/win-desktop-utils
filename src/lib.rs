//! Windows-first desktop utility helpers for Rust apps.
//!
//! This crate provides small helpers for common Windows desktop-app tasks:
//!
//! - opening files and directories with the default shell handler
//! - opening URLs
//! - revealing items in Explorer
//! - sending files or directories to the Recycle Bin
//! - enforcing single-instance behavior
//! - resolving per-user app-data directories
//! - checking elevation and relaunching as admin
//!
//! This crate is intended for Windows desktop applications and utilities.
//! Some functions launch external shell behavior or may trigger a UAC prompt.
//! This crate supports Windows only.
//!
//! # Example
//!
//! ```
//! fn main() -> Result<(), win_desktop_utils::Error> {
//!     let local = win_desktop_utils::local_app_data("demo-app")?;
//!     assert!(local.ends_with("demo-app"));
//!
//!     match win_desktop_utils::single_instance("demo-app")? {
//!         Some(_guard) => {}
//!         None => {}
//!     }
//!
//!     Ok(())
//! }
//! ```

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
pub use error::{Error, Result};

#[cfg(windows)]
pub use elevation::{is_elevated, restart_as_admin};
#[cfg(windows)]
pub use instance::{single_instance, InstanceGuard};
#[cfg(windows)]
pub use paths::{ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data};
#[cfg(windows)]
pub use shell::{move_to_recycle_bin, open_url, open_with_default, reveal_in_explorer};

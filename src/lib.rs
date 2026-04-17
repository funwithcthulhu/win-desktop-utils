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

pub mod elevation;
pub mod error;
pub mod instance;
pub mod paths;
pub mod shell;

pub use error::{Error, Result};

pub use elevation::{is_elevated, restart_as_admin};
pub use instance::{single_instance, InstanceGuard};
pub use paths::{ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data};
pub use shell::{move_to_recycle_bin, open_url, open_with_default, reveal_in_explorer};

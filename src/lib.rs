//! Windows-first desktop utility helpers for Rust apps.
//!
//! This crate provides small, focused helpers for common Windows desktop-app tasks:
//! opening files and URLs, revealing items in Explorer, sending files to the
//! Recycle Bin, enforcing single-instance behavior, resolving app-data
//! directories, and dealing with elevation.

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

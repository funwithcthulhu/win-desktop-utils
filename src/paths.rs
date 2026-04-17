use std::fs;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Returns the per-user roaming app-data directory for the given app name.
///
/// This function reads the `APPDATA` environment variable and appends `app_name`.
/// It does not create the directory.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_name` is empty or whitespace only.
/// Returns [`Error::Unsupported`] if `APPDATA` is not available.
///
/// # Examples
///
/// ```
/// let path = win_desktop_utils::roaming_app_data("demo-app")?;
/// assert!(path.ends_with("demo-app"));
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn roaming_app_data(app_name: &str) -> Result<PathBuf> {
    if app_name.trim().is_empty() {
        return Err(Error::InvalidInput("app_name cannot be empty"));
    }

    let base = std::env::var_os("APPDATA").ok_or(Error::Unsupported("APPDATA is not set"))?;

    Ok(PathBuf::from(base).join(app_name))
}

/// Returns the per-user local app-data directory for the given app name.
///
/// This function reads the `LOCALAPPDATA` environment variable and appends `app_name`.
/// It does not create the directory.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_name` is empty or whitespace only.
/// Returns [`Error::Unsupported`] if `LOCALAPPDATA` is not available.
///
/// # Examples
///
/// ```
/// let path = win_desktop_utils::local_app_data("demo-app")?;
/// assert!(path.ends_with("demo-app"));
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn local_app_data(app_name: &str) -> Result<PathBuf> {
    if app_name.trim().is_empty() {
        return Err(Error::InvalidInput("app_name cannot be empty"));
    }

    let base =
        std::env::var_os("LOCALAPPDATA").ok_or(Error::Unsupported("LOCALAPPDATA is not set"))?;

    Ok(PathBuf::from(base).join(app_name))
}

/// Returns the roaming app-data directory for the given app name and creates it if needed.
///
/// This is equivalent to calling [`roaming_app_data`] and then `create_dir_all` on the result.
///
/// # Errors
///
/// Propagates errors from [`roaming_app_data`] and directory creation.
///
/// # Examples
///
/// ```
/// let path = win_desktop_utils::ensure_roaming_app_data("demo-app")?;
/// assert!(path.ends_with("demo-app"));
/// assert!(path.exists());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn ensure_roaming_app_data(app_name: &str) -> Result<PathBuf> {
    let path = roaming_app_data(app_name)?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns the local app-data directory for the given app name and creates it if needed.
///
/// This is equivalent to calling [`local_app_data`] and then `create_dir_all` on the result.
///
/// # Errors
///
/// Propagates errors from [`local_app_data`] and directory creation.
///
/// # Examples
///
/// ```
/// let path = win_desktop_utils::ensure_local_app_data("demo-app")?;
/// assert!(path.ends_with("demo-app"));
/// assert!(path.exists());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn ensure_local_app_data(app_name: &str) -> Result<PathBuf> {
    let path = local_app_data(app_name)?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

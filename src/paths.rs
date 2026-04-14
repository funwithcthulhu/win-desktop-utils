use std::fs;
use std::path::PathBuf;

use crate::error::{Error, Result};

/// Returns the per-user roaming app-data directory for the given app name.
///
/// This does not create the directory.
pub fn roaming_app_data(app_name: &str) -> Result<PathBuf> {
    if app_name.trim().is_empty() {
        return Err(Error::InvalidInput("app_name cannot be empty"));
    }

    let base = std::env::var_os("APPDATA").ok_or(Error::Unsupported("APPDATA is not set"))?;

    Ok(PathBuf::from(base).join(app_name))
}

/// Returns the per-user local app-data directory for the given app name.
///
/// This does not create the directory.
pub fn local_app_data(app_name: &str) -> Result<PathBuf> {
    if app_name.trim().is_empty() {
        return Err(Error::InvalidInput("app_name cannot be empty"));
    }

    let base =
        std::env::var_os("LOCALAPPDATA").ok_or(Error::Unsupported("LOCALAPPDATA is not set"))?;

    Ok(PathBuf::from(base).join(app_name))
}

/// Returns the roaming app-data directory for the given app name and creates it if needed.
pub fn ensure_roaming_app_data(app_name: &str) -> Result<PathBuf> {
    let path = roaming_app_data(app_name)?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns the local app-data directory for the given app name and creates it if needed.
pub fn ensure_local_app_data(app_name: &str) -> Result<PathBuf> {
    let path = local_app_data(app_name)?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

use std::ffi::OsString;
use std::fs;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;

use crate::error::{Error, Result};
use windows::core::GUID;
use windows::Win32::System::Com::CoTaskMemFree;
use windows::Win32::UI::Shell::{
    FOLDERID_LocalAppData, FOLDERID_RoamingAppData, SHGetKnownFolderPath, KNOWN_FOLDER_FLAG,
};

fn known_folder_path(folder_id: &GUID, context: &'static str) -> Result<PathBuf> {
    let raw = unsafe { SHGetKnownFolderPath(folder_id as *const _, KNOWN_FOLDER_FLAG(0), None) }
        .map_err(|err| Error::WindowsApi {
            context,
            code: err.code().0,
        })?;

    if raw.is_null() {
        return Err(Error::WindowsApi { context, code: 0 });
    }

    let path = unsafe { PathBuf::from(OsString::from_wide(raw.as_wide())) };

    unsafe {
        CoTaskMemFree(Some(raw.0.cast()));
    }

    Ok(path)
}

/// Returns the per-user roaming app-data directory for the given app name.
///
/// This function resolves the roaming app-data known folder via `SHGetKnownFolderPath`
/// and appends `app_name`. It does not create the directory.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_name` is empty or whitespace only.
/// Returns [`Error::WindowsApi`] if the Windows known-folder lookup fails.
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

    let base = known_folder_path(
        &FOLDERID_RoamingAppData,
        "SHGetKnownFolderPath(RoamingAppData)",
    )?;
    Ok(base.join(app_name))
}

/// Returns the per-user local app-data directory for the given app name.
///
/// This function resolves the local app-data known folder via `SHGetKnownFolderPath`
/// and appends `app_name`. It does not create the directory.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_name` is empty or whitespace only.
/// Returns [`Error::WindowsApi`] if the Windows known-folder lookup fails.
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

    let base = known_folder_path(&FOLDERID_LocalAppData, "SHGetKnownFolderPath(LocalAppData)")?;
    Ok(base.join(app_name))
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

#[cfg(test)]
mod tests {
    use super::{known_folder_path, FOLDERID_LocalAppData, FOLDERID_RoamingAppData};

    #[test]
    fn known_folder_roaming_app_data_exists() {
        let path = known_folder_path(
            &FOLDERID_RoamingAppData,
            "SHGetKnownFolderPath(RoamingAppData)",
        )
        .unwrap();

        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn known_folder_local_app_data_exists() {
        let path = known_folder_path(&FOLDERID_LocalAppData, "SHGetKnownFolderPath(LocalAppData)")
            .unwrap();

        assert!(path.exists());
        assert!(path.is_dir());
    }
}

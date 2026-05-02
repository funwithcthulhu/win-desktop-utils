//! Single-instance helpers backed by named Windows mutexes.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::CreateMutexW;

use crate::error::{Error, Result};

/// Guard that keeps the named single-instance mutex alive for the current process.
///
/// Dropping this value releases the underlying mutex handle.
#[must_use = "keep this guard alive for as long as you want to hold the single-instance lock"]
#[derive(Debug)]
pub struct InstanceGuard {
    handle: HANDLE,
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

fn to_wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn validate_app_id(app_id: &str) -> Result<()> {
    if app_id.trim().is_empty() {
        return Err(Error::InvalidInput("app_id cannot be empty"));
    }

    if app_id.contains('\0') {
        return Err(Error::InvalidInput("app_id cannot contain NUL bytes"));
    }

    if app_id.contains('\\') {
        return Err(Error::InvalidInput("app_id cannot contain backslashes"));
    }

    Ok(())
}

/// Attempts to acquire a named process-wide single-instance guard.
///
/// Returns `Ok(Some(InstanceGuard))` for the first instance and `Ok(None)` if another
/// instance with the same `app_id` is already running.
///
/// The mutex name is derived from `app_id` using a `Local\...` namespace, so the
/// single-instance behavior is scoped to the current Windows session.
///
/// Keep the returned guard alive for as long as the current process should continue
/// to own the single-instance lock.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_id` is empty, contains only whitespace,
/// contains NUL bytes, or contains backslashes. Windows reserves backslashes in
/// named kernel objects for namespace separators such as `Local\` and `Global\`.
/// Returns [`Error::WindowsApi`] if `CreateMutexW` fails.
///
/// # Examples
///
/// ```
/// let app_id = format!("demo-app-{}", std::process::id());
/// let guard = win_desktop_utils::single_instance(&app_id)?;
/// assert!(guard.is_some());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[must_use = "store the returned guard for as long as the process should be considered the active instance"]
pub fn single_instance(app_id: &str) -> Result<Option<InstanceGuard>> {
    validate_app_id(app_id)?;

    let mutex_name = format!("Local\\win_desktop_utils_{app_id}");
    let mutex_name_w = to_wide_str(&mutex_name);

    let handle =
        unsafe { CreateMutexW(None, false, PCWSTR(mutex_name_w.as_ptr())) }.map_err(|e| {
            Error::WindowsApi {
                context: "CreateMutexW",
                code: e.code().0,
            }
        })?;

    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        unsafe {
            let _ = CloseHandle(handle);
        }
        Ok(None)
    } else {
        Ok(Some(InstanceGuard { handle }))
    }
}

#[cfg(test)]
mod tests {
    use super::validate_app_id;

    #[test]
    fn validate_app_id_rejects_empty_string() {
        let result = validate_app_id("");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("app_id cannot be empty"))
        ));
    }

    #[test]
    fn validate_app_id_rejects_backslashes() {
        let result = validate_app_id(r"demo\app");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "app_id cannot contain backslashes"
            ))
        ));
    }

    #[test]
    fn validate_app_id_rejects_nul_bytes() {
        let result = validate_app_id("demo\0app");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "app_id cannot contain NUL bytes"
            ))
        ));
    }
}

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::CreateMutexW;

use crate::error::{Error, Result};

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

pub fn single_instance(app_id: &str) -> Result<Option<InstanceGuard>> {
    if app_id.trim().is_empty() {
        return Err(Error::InvalidInput("app_id cannot be empty"));
    }

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

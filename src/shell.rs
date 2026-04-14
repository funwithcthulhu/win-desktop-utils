use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process::Command;

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::error::{Error, Result};

fn to_wide_os(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain(std::iter::once(0)).collect()
}

fn to_wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn shell_open_raw(target: &OsStr) -> Result<()> {
    let operation = to_wide_str("open");
    let target_w = to_wide_os(target);

    let result = unsafe {
        ShellExecuteW(
            Some(HWND::default()),
            PCWSTR(operation.as_ptr()),
            PCWSTR(target_w.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };

    let code = result.0 as isize;
    if code <= 32 {
        Err(Error::WindowsApi {
            context: "ShellExecuteW",
            code: code as i32,
        })
    } else {
        Ok(())
    }
}

pub fn open_with_default(target: impl AsRef<Path>) -> Result<()> {
    let path = target.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("target cannot be empty"));
    }

    shell_open_raw(path.as_os_str())
}

pub fn open_url(url: &str) -> Result<()> {
    if url.trim().is_empty() {
        return Err(Error::InvalidInput("url cannot be empty"));
    }

    shell_open_raw(OsStr::new(url))
}

pub fn reveal_in_explorer(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    let arg = format!("/select,{}", path.display());

    Command::new("explorer.exe").arg(arg).spawn()?;

    Ok(())
}

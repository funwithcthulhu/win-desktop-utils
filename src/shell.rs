use std::ffi::OsStr;
use std::io;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;
use std::process::Command;

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::{SHFileOperationW, ShellExecuteW, SHFILEOPSTRUCTW};
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

use crate::error::{Error, Result};

const FO_DELETE_CODE: u32 = 3;
const FOF_SILENT: u16 = 0x0004;
const FOF_NOCONFIRMATION: u16 = 0x0010;
const FOF_ALLOWUNDO: u16 = 0x0040;
const FOF_NOERRORUI: u16 = 0x0400;

fn to_wide_os(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain(std::iter::once(0)).collect()
}

fn to_wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn to_double_null_path(value: &Path) -> Vec<u16> {
    value
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
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

/// Opens a file or directory with the user's default Windows handler.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `target` is empty.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
pub fn open_with_default(target: impl AsRef<Path>) -> Result<()> {
    let path = target.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("target cannot be empty"));
    }

    shell_open_raw(path.as_os_str())
}

/// Opens a URL with the user's default browser or registered handler.
///
/// This function checks only that the input is non-empty after trimming whitespace.
/// URL validation is otherwise delegated to the Windows shell.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `url` is empty or whitespace only.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
pub fn open_url(url: &str) -> Result<()> {
    if url.trim().is_empty() {
        return Err(Error::InvalidInput("url cannot be empty"));
    }

    shell_open_raw(OsStr::new(url))
}

/// Opens Explorer and selects the requested path.
///
/// This function starts `explorer.exe` with `/select,` and the provided path.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `path` is empty.
/// Returns [`Error::Io`] if spawning `explorer.exe` fails.
pub fn reveal_in_explorer(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    Command::new("explorer.exe")
        .arg("/select,")
        .arg(path)
        .spawn()?;

    Ok(())
}

/// Sends a file or directory to the Windows Recycle Bin.
///
/// The path must be absolute and must exist.
///
/// This function uses `SHFileOperationW` with `FO_DELETE` and `FOF_ALLOWUNDO`.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `path` is empty or not absolute.
/// Returns [`Error::Io`] if the path does not exist.
/// Returns [`Error::WindowsApi`] if the shell operation fails or is aborted.
pub fn move_to_recycle_bin(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    if !path.is_absolute() {
        return Err(Error::InvalidInput("path must be absolute"));
    }

    if !path.exists() {
        return Err(Error::Io(io::Error::new(
            io::ErrorKind::NotFound,
            "path does not exist",
        )));
    }

    let from_w = to_double_null_path(path);

    let mut op = SHFILEOPSTRUCTW {
        hwnd: HWND::default(),
        wFunc: FO_DELETE_CODE,
        pFrom: PCWSTR(from_w.as_ptr()),
        pTo: PCWSTR::null(),
        fFlags: FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_SILENT,
        fAnyOperationsAborted: false.into(),
        hNameMappings: std::ptr::null_mut(),
        lpszProgressTitle: PCWSTR::null(),
    };

    let result = unsafe { SHFileOperationW(&mut op) };

    if result != 0 {
        Err(Error::WindowsApi {
            context: "SHFileOperationW(FO_DELETE)",
            code: result,
        })
    } else if op.fAnyOperationsAborted.as_bool() {
        Err(Error::WindowsApi {
            context: "SHFileOperationW(FO_DELETE) aborted",
            code: 0,
        })
    } else {
        Ok(())
    }
}

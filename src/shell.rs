//! Shell-facing helpers for opening files, URLs, Explorer selections, and the Recycle Bin.

use std::ffi::OsStr;
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

fn normalize_url(url: &str) -> Result<&str> {
    let trimmed = url.trim();

    if trimmed.is_empty() {
        return Err(Error::InvalidInput("url cannot be empty"));
    }

    if trimmed.contains('\0') {
        return Err(Error::InvalidInput("url cannot contain NUL bytes"));
    }

    Ok(trimmed)
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
/// Returns [`Error::PathDoesNotExist`] if the target path does not exist.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
///
/// # Examples
///
/// ```no_run
/// win_desktop_utils::open_with_default(r"C:\Windows\notepad.exe")?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn open_with_default(target: impl AsRef<Path>) -> Result<()> {
    let path = target.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("target cannot be empty"));
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
    }

    shell_open_raw(path.as_os_str())
}

/// Opens a URL with the user's default browser or registered handler.
///
/// Surrounding whitespace is trimmed before the URL is passed to the Windows shell.
/// URL validation is otherwise delegated to the Windows shell.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `url` is empty, whitespace only, or contains NUL bytes.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
///
/// # Examples
///
/// ```no_run
/// win_desktop_utils::open_url("https://www.rust-lang.org")?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn open_url(url: &str) -> Result<()> {
    let url = normalize_url(url)?;
    shell_open_raw(OsStr::new(url))
}

/// Opens Explorer and selects the requested path.
///
/// This function starts `explorer.exe` with `/select,` and the provided path.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `path` is empty.
/// Returns [`Error::PathDoesNotExist`] if the path does not exist.
/// Returns [`Error::Io`] if spawning `explorer.exe` fails.
///
/// # Examples
///
/// ```no_run
/// win_desktop_utils::reveal_in_explorer(r"C:\Windows\notepad.exe")?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn reveal_in_explorer(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
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
/// Returns [`Error::InvalidInput`] if `path` is empty.
/// Returns [`Error::PathNotAbsolute`] if the path is not absolute.
/// Returns [`Error::PathDoesNotExist`] if the path does not exist.
/// Returns [`Error::WindowsApi`] if the shell operation fails or is aborted.
///
/// # Examples
///
/// ```no_run
/// let path = std::env::current_dir()?.join("temporary-file.txt");
/// std::fs::write(&path, "temporary file")?;
/// win_desktop_utils::move_to_recycle_bin(&path)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn move_to_recycle_bin(path: impl AsRef<Path>) -> Result<()> {
    let path = path.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    if !path.is_absolute() {
        return Err(Error::PathNotAbsolute);
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
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

#[cfg(test)]
mod tests {
    use super::normalize_url;

    #[test]
    fn normalize_url_rejects_empty_string() {
        let result = normalize_url("");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("url cannot be empty"))
        ));
    }

    #[test]
    fn normalize_url_rejects_whitespace_only() {
        let result = normalize_url("   ");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("url cannot be empty"))
        ));
    }

    #[test]
    fn normalize_url_trims_surrounding_whitespace() {
        assert_eq!(
            normalize_url("  https://example.com/docs  ").unwrap(),
            "https://example.com/docs"
        );
    }

    #[test]
    fn normalize_url_rejects_nul_bytes() {
        let result = normalize_url("https://example.com/\0hidden");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("url cannot contain NUL bytes"))
        ));
    }
}

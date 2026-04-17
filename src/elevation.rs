use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Shell::{IsUserAnAdmin, ShellExecuteW};
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

fn quote_arg(arg: &OsStr) -> String {
    let text = arg.to_string_lossy().replace('"', "\\\"");
    format!("\"{text}\"")
}

/// Returns `true` if the current process is running elevated.
///
/// # Errors
///
/// This function currently does not produce operational errors, but it returns the
/// crate's standard [`Result`] type for API consistency.
pub fn is_elevated() -> Result<bool> {
    let is_admin = unsafe { IsUserAnAdmin() };
    Ok(is_admin.as_bool())
}

/// Relaunches the current executable with elevation using the Windows `runas` shell verb.
///
/// Arguments are passed as [`OsString`] values so Windows-native argument text is preserved
/// as well as possible before being joined for `ShellExecuteW`.
///
/// This function starts a new elevated instance of the current executable. It does not
/// terminate the current process.
///
/// # Errors
///
/// Returns [`Error::Io`] if the current executable path cannot be resolved.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
pub fn restart_as_admin(args: &[OsString]) -> Result<()> {
    let exe = std::env::current_exe()?;
    let exe_w = to_wide_os(exe.as_os_str());

    let verb_w = to_wide_str("runas");

    let joined_args = args
        .iter()
        .map(|a| quote_arg(a.as_os_str()))
        .collect::<Vec<_>>()
        .join(" ");
    let args_w = to_wide_str(&joined_args);

    let result = unsafe {
        ShellExecuteW(
            Some(HWND::default()),
            PCWSTR(verb_w.as_ptr()),
            PCWSTR(exe_w.as_ptr()),
            PCWSTR(args_w.as_ptr()),
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };

    let code = result.0 as isize;
    if code <= 32 {
        Err(Error::WindowsApi {
            context: "ShellExecuteW(runas)",
            code: code as i32,
        })
    } else {
        Ok(())
    }
}

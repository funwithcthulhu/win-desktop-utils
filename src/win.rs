//! Private Windows helpers shared by the public feature modules.

use std::ffi::OsStr;
#[cfg(any(feature = "elevation", feature = "shortcuts"))]
use std::ffi::OsString;
use std::os::windows::ffi::OsStrExt;
#[cfg(feature = "shortcuts")]
use std::path::Path;
#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
use std::thread;

#[cfg(any(feature = "elevation", feature = "shell"))]
use windows::core::PCWSTR;
#[cfg(any(feature = "elevation", feature = "shell"))]
use windows::Win32::Foundation::HWND;
#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
#[cfg(any(feature = "elevation", feature = "shell"))]
use windows::Win32::UI::Shell::ShellExecuteW;
#[cfg(any(feature = "elevation", feature = "shell"))]
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

#[cfg(any(
    feature = "elevation",
    feature = "recycle-bin",
    feature = "shell",
    feature = "shortcuts"
))]
use crate::error::{Error, Result};

#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
pub(crate) struct ComApartment;

#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
impl ComApartment {
    pub(crate) fn initialize_sta(context: &'static str) -> Result<Self> {
        let result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };

        if result.is_ok() {
            Ok(Self)
        } else {
            Err(Error::WindowsApi {
                context,
                code: result.0,
            })
        }
    }
}

#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

#[cfg(any(
    feature = "elevation",
    feature = "recycle-bin",
    feature = "shell",
    feature = "shortcuts"
))]
pub(crate) fn to_wide_os(value: &OsStr) -> Vec<u16> {
    value.encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(any(
    feature = "elevation",
    feature = "instance",
    feature = "shell",
    feature = "shortcuts"
))]
pub(crate) fn to_wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

#[cfg(any(feature = "elevation", feature = "shortcuts"))]
pub(crate) fn os_str_contains_nul(value: &OsStr) -> bool {
    value.encode_wide().any(|unit| unit == 0)
}

#[cfg(feature = "shortcuts")]
pub(crate) fn path_contains_nul(path: &Path) -> bool {
    os_str_contains_nul(path.as_os_str())
}

#[cfg(any(feature = "elevation", feature = "shortcuts"))]
pub(crate) fn quote_arg(arg: &OsStr) -> String {
    let text = arg.to_string_lossy();
    let mut quoted = String::with_capacity(text.len() + 2);
    let mut trailing_backslashes = 0usize;

    quoted.push('"');

    for ch in text.chars() {
        match ch {
            '\\' => trailing_backslashes += 1,
            '"' => {
                for _ in 0..(trailing_backslashes * 2 + 1) {
                    quoted.push('\\');
                }
                quoted.push('"');
                trailing_backslashes = 0;
            }
            _ => {
                for _ in 0..trailing_backslashes {
                    quoted.push('\\');
                }
                quoted.push(ch);
                trailing_backslashes = 0;
            }
        }
    }

    for _ in 0..(trailing_backslashes * 2) {
        quoted.push('\\');
    }
    quoted.push('"');

    quoted
}

#[cfg(any(feature = "elevation", feature = "shortcuts"))]
pub(crate) fn join_quoted_args(args: &[OsString]) -> String {
    args.iter()
        .map(|arg| quote_arg(arg.as_os_str()))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(any(feature = "elevation", feature = "shell"))]
pub(crate) fn normalize_nonempty_str<'a>(
    value: &'a str,
    empty_message: &'static str,
    nul_message: &'static str,
) -> Result<&'a str> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(Error::InvalidInput(empty_message));
    }

    if trimmed.contains('\0') {
        return Err(Error::InvalidInput(nul_message));
    }

    Ok(trimmed)
}

#[cfg(any(feature = "elevation", feature = "shell"))]
pub(crate) fn shell_execute(
    verb: &str,
    target: &OsStr,
    parameters: Option<&str>,
    context: &'static str,
) -> Result<()> {
    let verb_w = to_wide_str(verb);
    let target_w = to_wide_os(target);
    let parameters_w = parameters.map(to_wide_str);
    let parameters_ptr = parameters_w
        .as_ref()
        .map_or(PCWSTR::null(), |parameters| PCWSTR(parameters.as_ptr()));

    let result = unsafe {
        ShellExecuteW(
            Some(HWND::default()),
            PCWSTR(verb_w.as_ptr()),
            PCWSTR(target_w.as_ptr()),
            parameters_ptr,
            PCWSTR::null(),
            SW_SHOWNORMAL,
        )
    };

    let code = result.0 as isize;
    if code <= 32 {
        Err(Error::WindowsApi {
            context,
            code: code as i32,
        })
    } else {
        Ok(())
    }
}

#[cfg(any(feature = "recycle-bin", feature = "shortcuts"))]
pub(crate) fn run_in_sta<T, F>(panic_message: &'static str, work: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T> + Send + 'static,
{
    match thread::spawn(work).join() {
        Ok(result) => result,
        Err(_) => Err(Error::Unsupported(panic_message)),
    }
}

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

fn join_args_for_shell_execute(args: &[OsString]) -> String {
    args.iter()
        .map(|a| quote_arg(a.as_os_str()))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns `true` if the current process is running elevated.
///
/// # Errors
///
/// This function currently does not produce operational errors, but it returns the
/// crate's standard [`Result`] type for API consistency.
///
/// # Examples
///
/// ```no_run
/// let elevated = win_desktop_utils::is_elevated()?;
/// println!("elevated: {elevated}");
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
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
///
/// # Examples
///
/// ```no_run
/// use std::ffi::OsString;
///
/// let args = [OsString::from("--help")];
/// win_desktop_utils::restart_as_admin(&args)?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn restart_as_admin(args: &[OsString]) -> Result<()> {
    let exe = std::env::current_exe()?;
    let exe_w = to_wide_os(exe.as_os_str());

    let verb_w = to_wide_str("runas");
    let joined_args = join_args_for_shell_execute(args);
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

#[cfg(test)]
mod tests {
    use super::join_args_for_shell_execute;
    use std::ffi::OsString;

    #[test]
    fn join_args_empty_is_empty() {
        let args: [OsString; 0] = [];
        assert_eq!(join_args_for_shell_execute(&args), "");
    }

    #[test]
    fn join_args_quotes_each_argument() {
        let args = [OsString::from("alpha"), OsString::from("two words")];
        assert_eq!(
            join_args_for_shell_execute(&args),
            "\"alpha\" \"two words\""
        );
    }

    #[test]
    fn join_args_escapes_inner_quotes() {
        let args = [OsString::from("say \"hi\"")];
        assert_eq!(join_args_for_shell_execute(&args), "\"say \\\"hi\\\"\"");
    }
}

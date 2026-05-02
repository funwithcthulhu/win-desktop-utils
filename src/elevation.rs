//! Elevation helpers for checking admin state and relaunching through UAC.

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

fn join_args_for_shell_execute(args: &[OsString]) -> String {
    args.iter()
        .map(|a| quote_arg(a.as_os_str()))
        .collect::<Vec<_>>()
        .join(" ")
}

fn validate_shell_args(args: &[OsString], message: &'static str) -> Result<()> {
    if args
        .iter()
        .any(|arg| arg.as_os_str().encode_wide().any(|unit| unit == 0))
    {
        return Err(Error::InvalidInput(message));
    }

    Ok(())
}

fn validate_restart_args(args: &[OsString]) -> Result<()> {
    validate_shell_args(args, "restart arguments cannot contain NUL bytes")
}

fn validate_command_args(args: &[OsString]) -> Result<()> {
    validate_shell_args(args, "arguments cannot contain NUL bytes")
}

fn validate_executable(executable: &OsStr) -> Result<()> {
    if executable.is_empty() {
        return Err(Error::InvalidInput("executable cannot be empty"));
    }

    if executable.encode_wide().any(|unit| unit == 0) {
        return Err(Error::InvalidInput("executable cannot contain NUL bytes"));
    }

    Ok(())
}

fn normalize_shell_verb(verb: &str) -> Result<&str> {
    let trimmed = verb.trim();

    if trimmed.is_empty() {
        return Err(Error::InvalidInput("verb cannot be empty"));
    }

    if trimmed.contains('\0') {
        return Err(Error::InvalidInput("verb cannot contain NUL bytes"));
    }

    Ok(trimmed)
}

fn shell_execute_command(
    verb: &str,
    executable: &OsStr,
    args: &[OsString],
    context: &'static str,
) -> Result<()> {
    let verb_w = to_wide_str(verb);
    let executable_w = to_wide_os(executable);
    let joined_args = join_args_for_shell_execute(args);
    let args_w = if joined_args.is_empty() {
        None
    } else {
        Some(to_wide_str(&joined_args))
    };
    let args_ptr = args_w
        .as_ref()
        .map_or(PCWSTR::null(), |args| PCWSTR(args.as_ptr()));

    let result = unsafe {
        ShellExecuteW(
            Some(HWND::default()),
            PCWSTR(verb_w.as_ptr()),
            PCWSTR(executable_w.as_ptr()),
            args_ptr,
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
/// as well as possible before being joined for `ShellExecuteW` using standard Windows
/// command-line quoting rules.
///
/// This function starts a new elevated instance of the current executable. It does not
/// terminate the current process.
///
/// # Errors
///
/// Returns [`Error::Io`] if the current executable path cannot be resolved.
/// Returns [`Error::InvalidInput`] if any argument contains NUL bytes.
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
    validate_restart_args(args)?;

    let exe = std::env::current_exe()?;
    shell_execute_command("runas", exe.as_os_str(), args, "ShellExecuteW(runas)")
}

/// Launches an executable with elevation using the Windows `runas` shell verb.
///
/// Unlike [`restart_as_admin`], this function can start any executable or command
/// resolvable by the Windows shell.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `executable` is empty or contains NUL bytes,
/// or if any argument contains NUL bytes.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
///
/// # Examples
///
/// ```no_run
/// use std::ffi::OsString;
///
/// let args = [OsString::from("/c"), OsString::from("echo elevated")];
/// win_desktop_utils::run_as_admin("cmd.exe", &args)?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn run_as_admin(executable: impl AsRef<OsStr>, args: &[OsString]) -> Result<()> {
    run_with_verb("runas", executable, args)
}

/// Launches an executable through `ShellExecuteW` using an explicit shell verb.
///
/// This is useful for verbs such as `open` and `runas` when you need to pass a
/// command-line argument list.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `verb` is empty, whitespace only, or contains
/// NUL bytes, if `executable` is empty or contains NUL bytes, or if any argument
/// contains NUL bytes.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
///
/// # Examples
///
/// ```no_run
/// use std::ffi::OsString;
///
/// let args = [OsString::from("--help")];
/// win_desktop_utils::run_with_verb("open", "notepad.exe", &args)?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn run_with_verb(verb: &str, executable: impl AsRef<OsStr>, args: &[OsString]) -> Result<()> {
    let verb = normalize_shell_verb(verb)?;
    let executable = executable.as_ref();

    validate_executable(executable)?;
    validate_command_args(args)?;

    shell_execute_command(verb, executable, args, "ShellExecuteW")
}

#[cfg(test)]
mod tests {
    use super::{
        join_args_for_shell_execute, normalize_shell_verb, validate_command_args,
        validate_executable, validate_restart_args,
    };
    use std::ffi::OsStr;
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

    #[test]
    fn join_args_doubles_trailing_backslashes_inside_quotes() {
        let args = [OsString::from(r"C:\Program Files\demo\")];
        assert_eq!(
            join_args_for_shell_execute(&args),
            r#""C:\Program Files\demo\\""#
        );
    }

    #[test]
    fn validate_restart_args_rejects_nul_bytes() {
        let args = [OsString::from("hello\0world")];
        let result = validate_restart_args(&args);
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "restart arguments cannot contain NUL bytes"
            ))
        ));
    }

    #[test]
    fn validate_command_args_rejects_nul_bytes() {
        let args = [OsString::from("hello\0world")];
        let result = validate_command_args(&args);
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "arguments cannot contain NUL bytes"
            ))
        ));
    }

    #[test]
    fn validate_executable_rejects_empty_string() {
        let result = validate_executable(OsStr::new(""));
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("executable cannot be empty"))
        ));
    }

    #[test]
    fn validate_executable_rejects_nul_bytes() {
        let result = validate_executable(OsStr::new("cmd\0.exe"));
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "executable cannot contain NUL bytes"
            ))
        ));
    }

    #[test]
    fn normalize_shell_verb_rejects_empty_string() {
        let result = normalize_shell_verb("");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("verb cannot be empty"))
        ));
    }

    #[test]
    fn normalize_shell_verb_trims_surrounding_whitespace() {
        assert_eq!(normalize_shell_verb("  runas  ").unwrap(), "runas");
    }
}

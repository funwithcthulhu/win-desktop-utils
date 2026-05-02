//! Shell-facing helpers for opening files, URLs, Explorer selections, and the Recycle Bin.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{
    FileOperation, IFileOperation, IFileOperationProgressSink, IShellItem,
    SHCreateItemFromParsingName, ShellExecuteW, FOFX_RECYCLEONDELETE, FOF_ALLOWUNDO,
    FOF_NOCONFIRMATION, FOF_NOERRORUI, FOF_SILENT,
};
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

struct ComApartment;

impl ComApartment {
    fn initialize_sta() -> Result<Self> {
        let result = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) };

        if result.is_ok() {
            Ok(Self)
        } else {
            Err(Error::WindowsApi {
                context: "CoInitializeEx",
                code: result.0,
            })
        }
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}

fn shell_execute_raw(verb: &str, target: &OsStr) -> Result<()> {
    let operation = to_wide_str(verb);
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

fn shell_item_from_path(path: &Path) -> Result<IShellItem> {
    let path_w = to_wide_os(path.as_os_str());

    unsafe { SHCreateItemFromParsingName(PCWSTR(path_w.as_ptr()), None) }.map_err(|err| {
        Error::WindowsApi {
            context: "SHCreateItemFromParsingName",
            code: err.code().0,
        }
    })
}

fn validate_recycle_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("path cannot be empty"));
    }

    if !path.is_absolute() {
        return Err(Error::PathNotAbsolute);
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
    }

    Ok(())
}

fn collect_recycle_paths<I, P>(paths: I) -> Result<Vec<PathBuf>>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let mut collected = Vec::new();

    for path in paths {
        let path = path.as_ref();
        validate_recycle_path(path)?;
        collected.push(PathBuf::from(path));
    }

    if collected.is_empty() {
        Err(Error::InvalidInput("paths cannot be empty"))
    } else {
        Ok(collected)
    }
}

fn queue_recycle_item(operation: &IFileOperation, path: &Path) -> Result<()> {
    let item = shell_item_from_path(path)?;

    unsafe { operation.DeleteItem(&item, Option::<&IFileOperationProgressSink>::None) }.map_err(
        |err| Error::WindowsApi {
            context: "IFileOperation::DeleteItem",
            code: err.code().0,
        },
    )
}

fn recycle_paths_in_sta(paths: &[PathBuf]) -> Result<()> {
    let _com = ComApartment::initialize_sta()?;
    let operation: IFileOperation = unsafe {
        CoCreateInstance(&FileOperation, None, CLSCTX_INPROC_SERVER)
    }
    .map_err(|err| Error::WindowsApi {
        context: "CoCreateInstance(FileOperation)",
        code: err.code().0,
    })?;

    let flags =
        FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_NOERRORUI | FOF_SILENT | FOFX_RECYCLEONDELETE;

    unsafe { operation.SetOperationFlags(flags) }.map_err(|err| Error::WindowsApi {
        context: "IFileOperation::SetOperationFlags",
        code: err.code().0,
    })?;

    for path in paths {
        queue_recycle_item(&operation, path)?;
    }

    unsafe { operation.PerformOperations() }.map_err(|err| Error::WindowsApi {
        context: "IFileOperation::PerformOperations",
        code: err.code().0,
    })?;

    let aborted =
        unsafe { operation.GetAnyOperationsAborted() }.map_err(|err| Error::WindowsApi {
            context: "IFileOperation::GetAnyOperationsAborted",
            code: err.code().0,
        })?;

    if aborted.as_bool() {
        Err(Error::WindowsApi {
            context: "IFileOperation::PerformOperations aborted",
            code: 0,
        })
    } else {
        Ok(())
    }
}

fn run_in_shell_sta<T, F>(work: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T> + Send + 'static,
{
    match thread::spawn(work).join() {
        Ok(result) => result,
        Err(_) => Err(Error::Unsupported("shell STA worker thread panicked")),
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
    open_with_verb("open", target)
}

/// Opens a file or directory using a specific Windows shell verb.
///
/// Common verbs include `open`, `edit`, `print`, and `properties`, depending on
/// what the target path's registered handler supports.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `verb` is empty, whitespace only, or contains
/// NUL bytes, or if `target` is empty.
/// Returns [`Error::PathDoesNotExist`] if the target path does not exist.
/// Returns [`Error::WindowsApi`] if `ShellExecuteW` reports failure.
///
/// # Examples
///
/// ```no_run
/// win_desktop_utils::open_with_verb("properties", r"C:\Windows\notepad.exe")?;
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
pub fn open_with_verb(verb: &str, target: impl AsRef<Path>) -> Result<()> {
    let verb = normalize_shell_verb(verb)?;
    let path = target.as_ref();

    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput("target cannot be empty"));
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
    }

    shell_execute_raw(verb, path.as_os_str())
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
    shell_execute_raw("open", OsStr::new(url))
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
/// This function uses `IFileOperation` on a dedicated STA thread so it can request
/// recycle-bin behavior through the modern Shell API.
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
    validate_recycle_path(path)?;

    let path = PathBuf::from(path);
    run_in_shell_sta(move || recycle_paths_in_sta(std::slice::from_ref(&path)))
}

/// Sends multiple files or directories to the Windows Recycle Bin in one shell operation.
///
/// Every path must be absolute and must exist. All paths are validated before any shell
/// operation is started.
///
/// This function uses `IFileOperation` on a dedicated STA thread so it can request
/// recycle-bin behavior through the modern Shell API.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if the path collection is empty or any path is empty.
/// Returns [`Error::PathNotAbsolute`] if any path is not absolute.
/// Returns [`Error::PathDoesNotExist`] if any path does not exist.
/// Returns [`Error::WindowsApi`] if the shell operation fails or is aborted.
///
/// # Examples
///
/// ```no_run
/// let first = std::env::current_dir()?.join("temporary-file-a.txt");
/// let second = std::env::current_dir()?.join("temporary-file-b.txt");
/// std::fs::write(&first, "temporary file")?;
/// std::fs::write(&second, "temporary file")?;
/// win_desktop_utils::move_paths_to_recycle_bin([&first, &second])?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn move_paths_to_recycle_bin<I, P>(paths: I) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let paths = collect_recycle_paths(paths)?;
    run_in_shell_sta(move || recycle_paths_in_sta(&paths))
}

#[cfg(test)]
mod tests {
    use super::{collect_recycle_paths, normalize_shell_verb, normalize_url};
    use std::path::PathBuf;

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

    #[test]
    fn normalize_shell_verb_rejects_empty_string() {
        let result = normalize_shell_verb("");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("verb cannot be empty"))
        ));
    }

    #[test]
    fn normalize_shell_verb_rejects_whitespace_only() {
        let result = normalize_shell_verb("   ");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("verb cannot be empty"))
        ));
    }

    #[test]
    fn normalize_shell_verb_trims_surrounding_whitespace() {
        assert_eq!(
            normalize_shell_verb("  properties  ").unwrap(),
            "properties"
        );
    }

    #[test]
    fn normalize_shell_verb_rejects_nul_bytes() {
        let result = normalize_shell_verb("pro\0perties");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("verb cannot contain NUL bytes"))
        ));
    }

    #[test]
    fn collect_recycle_paths_rejects_empty_collection() {
        let paths: [PathBuf; 0] = [];
        let result = collect_recycle_paths(paths);
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("paths cannot be empty"))
        ));
    }
}

//! Shortcut helpers for Windows `.lnk` and `.url` files.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use windows::core::{Interface, PCWSTR};
use windows::Win32::System::Com::{CoCreateInstance, IPersistFile, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};

use crate::error::{Error, Result};
use crate::win::{
    join_quoted_args, os_str_contains_nul, path_contains_nul, run_in_sta, to_wide_os, to_wide_str,
    ComApartment,
};

/// Icon configuration for a Windows shortcut.
///
/// # Examples
///
/// ```
/// let icon = win_desktop_utils::ShortcutIcon::new(r"C:\Windows\notepad.exe", 0);
///
/// assert_eq!(icon.index, 0);
/// assert!(icon.path.ends_with("notepad.exe"));
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortcutIcon {
    /// Path to an icon resource, executable, or DLL.
    pub path: PathBuf,
    /// Zero-based icon index inside the resource.
    pub index: i32,
}

impl ShortcutIcon {
    /// Creates icon configuration for a shortcut.
    pub fn new(path: impl Into<PathBuf>, index: i32) -> Self {
        Self {
            path: path.into(),
            index,
        }
    }
}

/// Options used when creating a Windows `.lnk` shortcut.
///
/// # Examples
///
/// ```
/// let options = win_desktop_utils::ShortcutOptions::new()
///     .argument("--safe-mode")
///     .working_directory(r"C:\Windows")
///     .icon(r"C:\Windows\notepad.exe", 0)
///     .description("Open a tool");
///
/// assert_eq!(options.arguments.len(), 1);
/// assert!(options.working_directory.is_some());
/// assert!(options.icon.is_some());
/// assert_eq!(options.description.as_deref(), Some("Open a tool"));
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ShortcutOptions {
    /// Command-line arguments stored in the shortcut.
    pub arguments: Vec<OsString>,
    /// Optional working directory for the shortcut target.
    pub working_directory: Option<PathBuf>,
    /// Optional icon resource for the shortcut.
    pub icon: Option<ShortcutIcon>,
    /// Optional user-facing shortcut description.
    pub description: Option<String>,
}

impl ShortcutOptions {
    /// Creates empty shortcut options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the shortcut argument list.
    pub fn arguments<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.arguments = arguments.into_iter().map(Into::into).collect();
        self
    }

    /// Appends one shortcut argument.
    pub fn argument(mut self, argument: impl Into<OsString>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    /// Sets the shortcut working directory.
    pub fn working_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(path.into());
        self
    }

    /// Sets the shortcut icon resource.
    pub fn icon(mut self, path: impl Into<PathBuf>, index: i32) -> Self {
        self.icon = Some(ShortcutIcon::new(path, index));
        self
    }

    /// Sets the shortcut description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[derive(Clone, Debug)]
struct ShortcutRequest {
    shortcut_path: PathBuf,
    target_path: PathBuf,
    options: ShortcutOptions,
}

fn join_args_for_shortcut(args: &[OsString]) -> String {
    join_quoted_args(args)
}

fn has_extension(path: &Path, expected: &str) -> bool {
    path.extension()
        .map(|extension| extension.to_string_lossy().eq_ignore_ascii_case(expected))
        .unwrap_or(false)
}

fn validate_output_path(path: &Path, extension: &str, label: &'static str) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput(label));
    }

    if path_contains_nul(path) {
        return Err(Error::InvalidInput(
            "shortcut path cannot contain NUL bytes",
        ));
    }

    if !path.is_absolute() {
        return Err(Error::PathNotAbsolute);
    }

    if !has_extension(path, extension) {
        return Err(Error::InvalidInput(match extension {
            "lnk" => "shortcut path must use .lnk extension",
            "url" => "shortcut path must use .url extension",
            _ => "shortcut path has an unsupported extension",
        }));
    }

    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or(Error::InvalidInput(
            "shortcut path must have a parent directory",
        ))?;

    if !parent.exists() {
        return Err(Error::PathDoesNotExist);
    }

    if !parent.is_dir() {
        return Err(Error::InvalidInput(
            "shortcut path parent must be a directory",
        ));
    }

    Ok(())
}

fn validate_existing_absolute_path(
    path: &Path,
    empty_message: &'static str,
    nul_message: &'static str,
) -> Result<()> {
    if path.as_os_str().is_empty() {
        return Err(Error::InvalidInput(empty_message));
    }

    if path_contains_nul(path) {
        return Err(Error::InvalidInput(nul_message));
    }

    if !path.is_absolute() {
        return Err(Error::PathNotAbsolute);
    }

    if !path.exists() {
        return Err(Error::PathDoesNotExist);
    }

    Ok(())
}

fn validate_options(options: &ShortcutOptions) -> Result<()> {
    if options
        .arguments
        .iter()
        .any(|arg| os_str_contains_nul(arg.as_os_str()))
    {
        return Err(Error::InvalidInput(
            "shortcut arguments cannot contain NUL bytes",
        ));
    }

    if let Some(description) = &options.description {
        if description.contains('\0') {
            return Err(Error::InvalidInput(
                "shortcut description cannot contain NUL bytes",
            ));
        }
    }

    if let Some(working_directory) = &options.working_directory {
        validate_existing_absolute_path(
            working_directory,
            "working_directory cannot be empty",
            "working_directory cannot contain NUL bytes",
        )?;

        if !working_directory.is_dir() {
            return Err(Error::InvalidInput("working_directory must be a directory"));
        }
    }

    if let Some(icon) = &options.icon {
        validate_existing_absolute_path(
            &icon.path,
            "icon path cannot be empty",
            "icon path cannot contain NUL bytes",
        )?;
    }

    Ok(())
}

fn validate_url(url: &str) -> Result<&str> {
    let trimmed = url.trim();

    if trimmed.is_empty() {
        return Err(Error::InvalidInput("url cannot be empty"));
    }

    if trimmed.contains('\0') {
        return Err(Error::InvalidInput("url cannot contain NUL bytes"));
    }

    if trimmed.contains('\r') || trimmed.contains('\n') {
        return Err(Error::InvalidInput("url cannot contain line breaks"));
    }

    Ok(trimmed)
}

fn create_shortcut_in_sta(request: ShortcutRequest) -> Result<()> {
    let _com = ComApartment::initialize_sta("CoInitializeEx")?;
    let link: IShellLinkW = unsafe { CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER) }
        .map_err(|err| Error::WindowsApi {
            context: "CoCreateInstance(ShellLink)",
            code: err.code().0,
        })?;

    let target_w = to_wide_os(request.target_path.as_os_str());
    unsafe { link.SetPath(PCWSTR(target_w.as_ptr())) }.map_err(|err| Error::WindowsApi {
        context: "IShellLinkW::SetPath",
        code: err.code().0,
    })?;

    if !request.options.arguments.is_empty() {
        let arguments = join_args_for_shortcut(&request.options.arguments);
        let arguments_w = to_wide_str(&arguments);
        unsafe { link.SetArguments(PCWSTR(arguments_w.as_ptr())) }.map_err(|err| {
            Error::WindowsApi {
                context: "IShellLinkW::SetArguments",
                code: err.code().0,
            }
        })?;
    }

    if let Some(working_directory) = &request.options.working_directory {
        let working_directory_w = to_wide_os(working_directory.as_os_str());
        unsafe { link.SetWorkingDirectory(PCWSTR(working_directory_w.as_ptr())) }.map_err(
            |err| Error::WindowsApi {
                context: "IShellLinkW::SetWorkingDirectory",
                code: err.code().0,
            },
        )?;
    }

    if let Some(description) = &request.options.description {
        let description_w = to_wide_str(description);
        unsafe { link.SetDescription(PCWSTR(description_w.as_ptr())) }.map_err(|err| {
            Error::WindowsApi {
                context: "IShellLinkW::SetDescription",
                code: err.code().0,
            }
        })?;
    }

    if let Some(icon) = &request.options.icon {
        let icon_w = to_wide_os(icon.path.as_os_str());
        unsafe { link.SetIconLocation(PCWSTR(icon_w.as_ptr()), icon.index) }.map_err(|err| {
            Error::WindowsApi {
                context: "IShellLinkW::SetIconLocation",
                code: err.code().0,
            }
        })?;
    }

    let persist: IPersistFile = link.cast().map_err(|err| Error::WindowsApi {
        context: "IShellLinkW::cast(IPersistFile)",
        code: err.code().0,
    })?;
    let shortcut_w = to_wide_os(request.shortcut_path.as_os_str());

    unsafe { persist.Save(PCWSTR(shortcut_w.as_ptr()), true) }.map_err(|err| Error::WindowsApi {
        context: "IPersistFile::Save",
        code: err.code().0,
    })
}

fn run_in_shortcut_sta<T, F>(work: F) -> Result<T>
where
    T: Send + 'static,
    F: FnOnce() -> Result<T> + Send + 'static,
{
    run_in_sta("shortcut STA worker thread panicked", work)
}

/// Creates or overwrites a Windows `.lnk` shortcut.
///
/// `shortcut_path` must be an absolute `.lnk` path whose parent directory exists.
/// `target_path` must be an existing absolute path. Use [`ShortcutOptions`] to set
/// arguments, a working directory, an icon, or a description.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] for empty paths, NUL bytes, unsupported extensions,
/// invalid options, or malformed text fields.
/// Returns [`Error::PathNotAbsolute`] if a required path is relative.
/// Returns [`Error::PathDoesNotExist`] if the target path or output parent directory
/// does not exist.
/// Returns [`Error::WindowsApi`] if COM or Shell Link APIs report failure.
///
/// # Examples
///
/// ```no_run
/// let shortcut = std::env::current_dir()?.join("notepad.lnk");
/// let options = win_desktop_utils::ShortcutOptions::new()
///     .description("Open Notepad");
///
/// win_desktop_utils::create_shortcut(
///     &shortcut,
///     r"C:\Windows\notepad.exe",
///     &options,
/// )?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn create_shortcut(
    shortcut_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
    options: &ShortcutOptions,
) -> Result<()> {
    let shortcut_path = shortcut_path.as_ref();
    let target_path = target_path.as_ref();

    validate_output_path(shortcut_path, "lnk", "shortcut path cannot be empty")?;
    validate_existing_absolute_path(
        target_path,
        "target path cannot be empty",
        "target path cannot contain NUL bytes",
    )?;
    validate_options(options)?;

    let request = ShortcutRequest {
        shortcut_path: shortcut_path.to_path_buf(),
        target_path: target_path.to_path_buf(),
        options: options.clone(),
    };

    run_in_shortcut_sta(move || create_shortcut_in_sta(request))
}

/// Creates or overwrites an Internet Shortcut `.url` file.
///
/// `shortcut_path` must be an absolute `.url` path whose parent directory exists.
/// Surrounding whitespace is trimmed from `url`.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] for empty paths, malformed URLs, NUL bytes,
/// line breaks in the URL, or unsupported extensions.
/// Returns [`Error::PathNotAbsolute`] if `shortcut_path` is relative.
/// Returns [`Error::PathDoesNotExist`] if the output parent directory does not exist.
/// Returns [`Error::Io`] if writing the shortcut file fails.
///
/// # Examples
///
/// ```
/// let shortcut = std::env::temp_dir().join(format!(
///     "win-desktop-utils-docs-{}.url",
///     std::process::id()
/// ));
///
/// win_desktop_utils::create_url_shortcut(
///     &shortcut,
///     "https://docs.rs/win-desktop-utils",
/// )?;
/// std::fs::remove_file(shortcut)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn create_url_shortcut(shortcut_path: impl AsRef<Path>, url: &str) -> Result<()> {
    let shortcut_path = shortcut_path.as_ref();
    let url = validate_url(url)?;

    validate_output_path(shortcut_path, "url", "shortcut path cannot be empty")?;

    let body = format!("[InternetShortcut]\r\nURL={url}\r\n");
    std::fs::write(shortcut_path, body)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        create_url_shortcut, join_args_for_shortcut, validate_output_path, validate_url,
        ShortcutOptions,
    };
    use std::ffi::OsString;

    #[test]
    fn shortcut_options_builder_sets_values() {
        let options = ShortcutOptions::new()
            .argument("--help")
            .working_directory(r"C:\Windows")
            .icon(r"C:\Windows\notepad.exe", 0)
            .description("Demo shortcut");

        assert_eq!(options.arguments, [OsString::from("--help")]);
        assert_eq!(options.description.as_deref(), Some("Demo shortcut"));
        assert!(options.working_directory.is_some());
        assert!(options.icon.is_some());
    }

    #[test]
    fn join_args_quotes_each_argument() {
        let args = [OsString::from("alpha"), OsString::from("two words")];
        assert_eq!(join_args_for_shortcut(&args), "\"alpha\" \"two words\"");
    }

    #[test]
    fn validate_url_trims_surrounding_whitespace() {
        assert_eq!(
            validate_url("  https://example.com/docs  ").unwrap(),
            "https://example.com/docs"
        );
    }

    #[test]
    fn validate_url_rejects_line_breaks() {
        let result = validate_url("https://example.com/\r\nIconFile=bad.ico");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("url cannot contain line breaks"))
        ));
    }

    #[test]
    fn validate_output_path_rejects_relative_paths() {
        let result = validate_output_path(
            std::path::Path::new("demo.lnk"),
            "lnk",
            "shortcut path cannot be empty",
        );
        assert!(matches!(result, Err(crate::Error::PathNotAbsolute)));
    }

    #[test]
    fn validate_output_path_rejects_wrong_extension() {
        let path = std::env::temp_dir().join("demo.txt");
        let result = validate_output_path(&path, "lnk", "shortcut path cannot be empty");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "shortcut path must use .lnk extension"
            ))
        ));
    }

    #[test]
    fn create_url_shortcut_writes_url_file() {
        let path = std::env::temp_dir().join(format!(
            "win-desktop-utils-url-shortcut-test-{}.url",
            std::process::id()
        ));
        let _ = std::fs::remove_file(&path);

        create_url_shortcut(&path, " https://example.com/docs ").unwrap();

        let body = std::fs::read_to_string(&path).unwrap();
        assert_eq!(
            body,
            "[InternetShortcut]\r\nURL=https://example.com/docs\r\n"
        );

        std::fs::remove_file(path).unwrap();
    }
}

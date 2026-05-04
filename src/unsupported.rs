//! Non-Windows public API stubs.
//!
//! These definitions let cross-platform crates type-check with the same public
//! surface. Functions that require Windows return [`Error::Unsupported`].

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

const WINDOWS_ONLY: &str = "win-desktop-utils operation requires Windows";

fn unsupported<T>() -> Result<T> {
    Err(Error::Unsupported(WINDOWS_ONLY))
}

#[cfg(feature = "instance")]
/// Scope used when creating the named mutex for single-instance enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceScope {
    /// Use the current Windows session namespace (`Local\...`).
    CurrentSession,
    /// Use the global Windows namespace (`Global\...`) so instances across
    /// all sessions contend for the same named mutex.
    Global,
}

#[cfg(feature = "instance")]
/// Options for single-instance mutex acquisition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleInstanceOptions {
    app_id: String,
    scope: InstanceScope,
}

#[cfg(feature = "instance")]
impl SingleInstanceOptions {
    /// Creates options for the current Windows session namespace.
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            scope: InstanceScope::CurrentSession,
        }
    }

    /// Creates options for the current Windows session namespace.
    pub fn current_session(app_id: impl Into<String>) -> Self {
        Self::new(app_id)
    }

    /// Creates options for the global Windows namespace.
    pub fn global(app_id: impl Into<String>) -> Self {
        Self::new(app_id).scope(InstanceScope::Global)
    }

    /// Sets the mutex namespace scope.
    pub fn scope(mut self, scope: InstanceScope) -> Self {
        self.scope = scope;
        self
    }

    /// Returns the configured application ID.
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the configured mutex namespace scope.
    pub fn configured_scope(&self) -> InstanceScope {
        self.scope
    }

    /// Attempts to acquire the configured single-instance guard.
    pub fn acquire(&self) -> Result<Option<InstanceGuard>> {
        single_instance_with_options(self)
    }
}

#[cfg(feature = "instance")]
/// Guard that keeps the named single-instance mutex alive for the current process.
#[must_use = "keep this guard alive for as long as you want to hold the single-instance lock"]
#[derive(Debug)]
pub struct InstanceGuard {
    _private: (),
}

#[cfg(feature = "instance")]
/// Attempts to acquire a named process-wide single-instance guard.
pub fn single_instance(app_id: &str) -> Result<Option<InstanceGuard>> {
    let _ = app_id;
    unsupported()
}

#[cfg(feature = "instance")]
/// Attempts to acquire a named single-instance guard in the requested Windows namespace.
pub fn single_instance_with_scope(
    app_id: &str,
    scope: InstanceScope,
) -> Result<Option<InstanceGuard>> {
    let _ = (app_id, scope);
    unsupported()
}

#[cfg(feature = "instance")]
/// Attempts to acquire a named single-instance guard using [`SingleInstanceOptions`].
pub fn single_instance_with_options(
    options: &SingleInstanceOptions,
) -> Result<Option<InstanceGuard>> {
    let _ = options;
    unsupported()
}

#[cfg(feature = "paths")]
/// Returns the per-user roaming app-data directory for the given app name.
pub fn roaming_app_data(app_name: &str) -> Result<PathBuf> {
    let _ = app_name;
    unsupported()
}

#[cfg(feature = "paths")]
/// Returns the per-user local app-data directory for the given app name.
pub fn local_app_data(app_name: &str) -> Result<PathBuf> {
    let _ = app_name;
    unsupported()
}

#[cfg(feature = "paths")]
/// Returns the roaming app-data directory for the given app name and creates it if needed.
pub fn ensure_roaming_app_data(app_name: &str) -> Result<PathBuf> {
    let _ = app_name;
    unsupported()
}

#[cfg(feature = "paths")]
/// Returns the local app-data directory for the given app name and creates it if needed.
pub fn ensure_local_app_data(app_name: &str) -> Result<PathBuf> {
    let _ = app_name;
    unsupported()
}

#[cfg(feature = "app")]
/// Validated Windows desktop app identity used by the app-data and single-instance helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopApp {
    company_name: Option<String>,
    app_name: String,
    app_dir_name: String,
    app_id: String,
    instance_scope: InstanceScope,
}

#[cfg(feature = "app")]
impl DesktopApp {
    /// Creates a desktop app identity without a company namespace.
    pub fn new(app_name: impl Into<String>) -> Result<Self> {
        let app_name = validate_identity_part("app_name", app_name.into())?;

        Ok(Self {
            company_name: None,
            app_dir_name: app_name.clone(),
            app_id: app_name.clone(),
            app_name,
            instance_scope: InstanceScope::CurrentSession,
        })
    }

    /// Creates a desktop app identity grouped under a company namespace.
    pub fn with_company(
        company_name: impl Into<String>,
        app_name: impl Into<String>,
    ) -> Result<Self> {
        let company_name = validate_identity_part("company_name", company_name.into())?;
        let app_name = validate_identity_part("app_name", app_name.into())?;

        Ok(Self {
            app_dir_name: format!("{company_name}\\{app_name}"),
            app_id: format!("{company_name}.{app_name}"),
            company_name: Some(company_name),
            app_name,
            instance_scope: InstanceScope::CurrentSession,
        })
    }

    /// Sets the default single-instance mutex namespace scope used by [`Self::single_instance`].
    pub fn instance_scope(mut self, scope: InstanceScope) -> Self {
        self.instance_scope = scope;
        self
    }

    /// Returns the optional company name.
    pub fn company_name(&self) -> Option<&str> {
        self.company_name.as_deref()
    }

    /// Returns the app name.
    pub fn app_name(&self) -> &str {
        &self.app_name
    }

    /// Returns the app-data directory name used by the paths helpers.
    pub fn app_dir_name(&self) -> &str {
        &self.app_dir_name
    }

    /// Returns the default app ID used by the single-instance helpers.
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the configured single-instance scope.
    pub fn configured_instance_scope(&self) -> InstanceScope {
        self.instance_scope
    }

    /// Returns the per-user local app-data directory for this app without creating it.
    pub fn local_data_dir(&self) -> Result<PathBuf> {
        local_app_data(&self.app_dir_name)
    }

    /// Returns the per-user roaming app-data directory for this app without creating it.
    pub fn roaming_data_dir(&self) -> Result<PathBuf> {
        roaming_app_data(&self.app_dir_name)
    }

    /// Creates and returns the per-user local app-data directory for this app.
    pub fn ensure_local_data_dir(&self) -> Result<PathBuf> {
        ensure_local_app_data(&self.app_dir_name)
    }

    /// Creates and returns the per-user roaming app-data directory for this app.
    pub fn ensure_roaming_data_dir(&self) -> Result<PathBuf> {
        ensure_roaming_app_data(&self.app_dir_name)
    }

    /// Returns single-instance options for this app.
    pub fn single_instance_options(&self) -> SingleInstanceOptions {
        SingleInstanceOptions::new(self.app_id.clone()).scope(self.instance_scope)
    }

    /// Attempts to acquire the configured single-instance guard for this app.
    pub fn single_instance(&self) -> Result<Option<InstanceGuard>> {
        single_instance_with_options(&self.single_instance_options())
    }
}

#[cfg(feature = "app")]
fn validate_identity_part(label: &'static str, value: String) -> Result<String> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(Error::InvalidInput(match label {
            "company_name" => "company_name cannot be empty",
            _ => "app_name cannot be empty",
        }));
    }

    if trimmed.contains('\0') {
        return Err(Error::InvalidInput(match label {
            "company_name" => "company_name cannot contain NUL bytes",
            _ => "app_name cannot contain NUL bytes",
        }));
    }

    if trimmed
        .chars()
        .any(|ch| matches!(ch, '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*'))
    {
        return Err(Error::InvalidInput(match label {
            "company_name" => "company_name contains invalid Windows file-name characters",
            _ => "app_name contains invalid Windows file-name characters",
        }));
    }

    Ok(trimmed.to_owned())
}

#[cfg(feature = "shell")]
/// Opens a file or directory with the user's default Windows handler.
pub fn open_with_default(target: impl AsRef<Path>) -> Result<()> {
    let _ = target.as_ref();
    unsupported()
}

#[cfg(feature = "shell")]
/// Opens a file or directory using a specific Windows shell verb.
pub fn open_with_verb(verb: &str, target: impl AsRef<Path>) -> Result<()> {
    let _ = (verb, target.as_ref());
    unsupported()
}

#[cfg(feature = "shell")]
/// Opens the Windows Properties sheet for a file or directory.
pub fn show_properties(target: impl AsRef<Path>) -> Result<()> {
    let _ = target.as_ref();
    unsupported()
}

#[cfg(feature = "shell")]
/// Prints a file using its registered default print shell verb.
pub fn print_with_default(target: impl AsRef<Path>) -> Result<()> {
    let _ = target.as_ref();
    unsupported()
}

#[cfg(feature = "shell")]
/// Opens a URL with the user's default browser or registered handler.
pub fn open_url(url: &str) -> Result<()> {
    let _ = url;
    unsupported()
}

#[cfg(feature = "shell")]
/// Opens Explorer and selects the requested path.
pub fn reveal_in_explorer(path: impl AsRef<Path>) -> Result<()> {
    let _ = path.as_ref();
    unsupported()
}

#[cfg(feature = "shell")]
/// Opens the directory containing an existing file or directory.
pub fn open_containing_folder(path: impl AsRef<Path>) -> Result<()> {
    let _ = path.as_ref();
    unsupported()
}

#[cfg(feature = "recycle-bin")]
/// Sends a file or directory to the Windows Recycle Bin.
pub fn move_to_recycle_bin(path: impl AsRef<Path>) -> Result<()> {
    let _ = path.as_ref();
    unsupported()
}

#[cfg(feature = "recycle-bin")]
/// Sends multiple files or directories to the Windows Recycle Bin in one shell operation.
pub fn move_paths_to_recycle_bin<I, P>(paths: I) -> Result<()>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let _ = paths.into_iter();
    unsupported()
}

#[cfg(feature = "recycle-bin")]
/// Permanently empties the Recycle Bin for all drives without shell UI.
pub fn empty_recycle_bin() -> Result<()> {
    unsupported()
}

#[cfg(feature = "recycle-bin")]
/// Permanently empties the Recycle Bin for a specific drive root without shell UI.
pub fn empty_recycle_bin_for_root(root_path: impl AsRef<Path>) -> Result<()> {
    let _ = root_path.as_ref();
    unsupported()
}

#[cfg(feature = "shortcuts")]
/// Icon configuration for a Windows shortcut.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortcutIcon {
    /// Path to an icon resource, executable, or DLL.
    pub path: PathBuf,
    /// Zero-based icon index inside the resource.
    pub index: i32,
}

#[cfg(feature = "shortcuts")]
impl ShortcutIcon {
    /// Selects an icon resource path and zero-based icon index.
    pub fn new(path: impl Into<PathBuf>, index: i32) -> Self {
        Self {
            path: path.into(),
            index,
        }
    }
}

#[cfg(feature = "shortcuts")]
/// Options used when creating a Windows `.lnk` shortcut.
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

#[cfg(feature = "shortcuts")]
impl ShortcutOptions {
    /// Creates options with no arguments, working directory, icon, or description.
    pub fn new() -> Self {
        Self::default()
    }

    /// Replaces the command-line arguments stored in the shortcut.
    pub fn arguments<I, S>(mut self, arguments: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.arguments = arguments.into_iter().map(Into::into).collect();
        self
    }

    /// Appends one command-line argument stored in the shortcut.
    pub fn argument(mut self, argument: impl Into<OsString>) -> Self {
        self.arguments.push(argument.into());
        self
    }

    /// Sets the working directory used when the shortcut target starts.
    pub fn working_directory(mut self, path: impl Into<PathBuf>) -> Self {
        self.working_directory = Some(path.into());
        self
    }

    /// Sets the icon resource used by Explorer for the shortcut.
    pub fn icon(mut self, path: impl Into<PathBuf>, index: i32) -> Self {
        self.icon = Some(ShortcutIcon::new(path, index));
        self
    }

    /// Sets the shortcut description shown by Explorer.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

#[cfg(feature = "shortcuts")]
/// Creates or overwrites a Windows `.lnk` shortcut.
pub fn create_shortcut(
    shortcut_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
    options: &ShortcutOptions,
) -> Result<()> {
    let _ = (shortcut_path.as_ref(), target_path.as_ref(), options);
    unsupported()
}

#[cfg(feature = "shortcuts")]
/// Creates or overwrites an Internet Shortcut `.url` file.
pub fn create_url_shortcut(shortcut_path: impl AsRef<Path>, url: &str) -> Result<()> {
    let _ = (shortcut_path.as_ref(), url);
    unsupported()
}

#[cfg(feature = "elevation")]
/// Returns `true` if the current process is running elevated.
pub fn is_elevated() -> Result<bool> {
    unsupported()
}

#[cfg(feature = "elevation")]
/// Relaunches the current executable with elevation using the Windows `runas` shell verb.
pub fn restart_as_admin(args: &[OsString]) -> Result<()> {
    let _ = args;
    unsupported()
}

#[cfg(feature = "elevation")]
/// Launches an executable with elevation using the Windows `runas` shell verb.
pub fn run_as_admin(executable: impl AsRef<OsStr>, args: &[OsString]) -> Result<()> {
    let _ = (executable.as_ref(), args);
    unsupported()
}

#[cfg(feature = "elevation")]
/// Launches an executable through `ShellExecuteW` using an explicit shell verb.
pub fn run_with_verb(verb: &str, executable: impl AsRef<OsStr>, args: &[OsString]) -> Result<()> {
    let _ = (verb, executable.as_ref(), args);
    unsupported()
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "paths")]
    use super::local_app_data;
    #[cfg(feature = "app")]
    use super::DesktopApp;
    #[cfg(feature = "shortcuts")]
    use super::ShortcutOptions;

    #[cfg(feature = "app")]
    #[test]
    fn desktop_app_keeps_identity_available_on_non_windows() {
        let app = DesktopApp::with_company("Demo Company", "Demo App").unwrap();
        assert_eq!(app.company_name(), Some("Demo Company"));
        assert_eq!(app.app_dir_name(), "Demo Company\\Demo App");
        assert_eq!(app.app_id(), "Demo Company.Demo App");
    }

    #[cfg(feature = "paths")]
    #[test]
    fn path_helpers_return_unsupported_on_non_windows() {
        let result = local_app_data("Demo App");
        assert!(matches!(result, Err(crate::Error::Unsupported(_))));
    }

    #[cfg(feature = "shortcuts")]
    #[test]
    fn shortcut_options_builder_sets_values_on_non_windows() {
        let options = ShortcutOptions::new()
            .argument("--demo")
            .working_directory("/tmp")
            .description("Demo");

        assert_eq!(options.arguments.len(), 1);
        assert_eq!(options.description.as_deref(), Some("Demo"));
        assert!(options.working_directory.is_some());
    }
}

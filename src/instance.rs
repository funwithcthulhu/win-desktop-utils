//! Single-instance helpers backed by named Windows mutexes.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, GetLastError, ERROR_ALREADY_EXISTS, HANDLE};
use windows::Win32::System::Threading::CreateMutexW;

use crate::error::{Error, Result};

/// Scope used when creating the named mutex for single-instance enforcement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstanceScope {
    /// Use the current Windows session namespace (`Local\...`).
    CurrentSession,
    /// Use the global Windows namespace (`Global\...`) so instances across
    /// all sessions contend for the same named mutex.
    Global,
}

impl InstanceScope {
    fn namespace_prefix(self) -> &'static str {
        match self {
            Self::CurrentSession => "Local",
            Self::Global => "Global",
        }
    }
}

/// Options for single-instance mutex acquisition.
///
/// This builder is useful when an application wants to keep the single-instance
/// configuration close to startup code while still using the default current-session
/// behavior most of the time.
///
/// # Examples
///
/// ```
/// let options = win_desktop_utils::SingleInstanceOptions::new(format!(
///     "demo-options-{}",
///     std::process::id()
/// ))
///     .scope(win_desktop_utils::InstanceScope::CurrentSession);
///
/// let guard = win_desktop_utils::single_instance_with_options(&options)?;
/// assert!(guard.is_some());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SingleInstanceOptions {
    app_id: String,
    scope: InstanceScope,
}

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

/// Guard that keeps the named single-instance mutex alive for the current process.
///
/// Dropping this value releases the underlying mutex handle.
#[must_use = "keep this guard alive for as long as you want to hold the single-instance lock"]
#[derive(Debug)]
pub struct InstanceGuard {
    handle: HANDLE,
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

fn to_wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

fn validate_app_id(app_id: &str) -> Result<()> {
    if app_id.trim().is_empty() {
        return Err(Error::InvalidInput("app_id cannot be empty"));
    }

    if app_id.contains('\0') {
        return Err(Error::InvalidInput("app_id cannot contain NUL bytes"));
    }

    if app_id.contains('\\') {
        return Err(Error::InvalidInput("app_id cannot contain backslashes"));
    }

    Ok(())
}

fn mutex_name(app_id: &str, scope: InstanceScope) -> String {
    format!("{}\\win_desktop_utils_{app_id}", scope.namespace_prefix())
}

/// Attempts to acquire a named process-wide single-instance guard.
///
/// Returns `Ok(Some(InstanceGuard))` for the first instance and `Ok(None)` if another
/// instance with the same `app_id` is already running.
///
/// This is a convenience wrapper around [`single_instance_with_scope`] using
/// [`InstanceScope::CurrentSession`].
///
/// The mutex name is derived from `app_id` using a `Local\...` namespace, so the
/// single-instance behavior is scoped to the current Windows session.
///
/// Keep the returned guard alive for as long as the current process should continue
/// to own the single-instance lock.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_id` is empty, contains only whitespace,
/// contains NUL bytes, or contains backslashes. Windows reserves backslashes in
/// named kernel objects for namespace separators such as `Local\` and `Global\`.
/// Returns [`Error::WindowsApi`] if `CreateMutexW` fails.
///
/// # Examples
///
/// ```
/// let app_id = format!("demo-app-{}", std::process::id());
/// let guard = win_desktop_utils::single_instance(&app_id)?;
/// assert!(guard.is_some());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[must_use = "store the returned guard for as long as the process should be considered the active instance"]
pub fn single_instance(app_id: &str) -> Result<Option<InstanceGuard>> {
    single_instance_with_scope(app_id, InstanceScope::CurrentSession)
}

/// Attempts to acquire a named single-instance guard in the requested Windows namespace.
///
/// Returns `Ok(Some(InstanceGuard))` for the first instance in the selected scope and
/// `Ok(None)` if another instance with the same `app_id` is already running in that scope.
///
/// Use [`InstanceScope::CurrentSession`] to enforce a single instance per logged-in session,
/// or [`InstanceScope::Global`] to enforce a single instance across sessions.
///
/// Keep the returned guard alive for as long as the current process should continue
/// to own the single-instance lock.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if `app_id` is empty, contains only whitespace,
/// contains NUL bytes, or contains backslashes.
/// Returns [`Error::WindowsApi`] if `CreateMutexW` fails.
///
/// # Examples
///
/// ```
/// let app_id = format!("demo-app-global-{}", std::process::id());
/// let guard = win_desktop_utils::single_instance_with_scope(
///     &app_id,
///     win_desktop_utils::InstanceScope::Global,
/// )?;
/// assert!(guard.is_some());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[must_use = "store the returned guard for as long as the process should be considered the active instance"]
pub fn single_instance_with_scope(
    app_id: &str,
    scope: InstanceScope,
) -> Result<Option<InstanceGuard>> {
    validate_app_id(app_id)?;

    let mutex_name = mutex_name(app_id, scope);
    let mutex_name_w = to_wide_str(&mutex_name);

    let handle =
        unsafe { CreateMutexW(None, false, PCWSTR(mutex_name_w.as_ptr())) }.map_err(|e| {
            Error::WindowsApi {
                context: "CreateMutexW",
                code: e.code().0,
            }
        })?;

    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        unsafe {
            let _ = CloseHandle(handle);
        }
        Ok(None)
    } else {
        Ok(Some(InstanceGuard { handle }))
    }
}

/// Attempts to acquire a named single-instance guard using [`SingleInstanceOptions`].
///
/// This is equivalent to calling [`single_instance_with_scope`] with the configured
/// application ID and scope.
///
/// # Errors
///
/// Returns [`Error::InvalidInput`] if the configured `app_id` is empty, contains only
/// whitespace, contains NUL bytes, or contains backslashes.
/// Returns [`Error::WindowsApi`] if `CreateMutexW` fails.
///
/// # Examples
///
/// ```
/// let options = win_desktop_utils::SingleInstanceOptions::global(
///     format!("demo-options-{}", std::process::id()),
/// );
/// let guard = win_desktop_utils::single_instance_with_options(&options)?;
/// assert!(guard.is_some());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[must_use = "store the returned guard for as long as the process should be considered the active instance"]
pub fn single_instance_with_options(
    options: &SingleInstanceOptions,
) -> Result<Option<InstanceGuard>> {
    single_instance_with_scope(options.app_id(), options.configured_scope())
}

#[cfg(test)]
mod tests {
    use super::{mutex_name, validate_app_id, InstanceScope, SingleInstanceOptions};

    #[test]
    fn validate_app_id_rejects_empty_string() {
        let result = validate_app_id("");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("app_id cannot be empty"))
        ));
    }

    #[test]
    fn validate_app_id_rejects_backslashes() {
        let result = validate_app_id(r"demo\app");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "app_id cannot contain backslashes"
            ))
        ));
    }

    #[test]
    fn validate_app_id_rejects_nul_bytes() {
        let result = validate_app_id("demo\0app");
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "app_id cannot contain NUL bytes"
            ))
        ));
    }

    #[test]
    fn mutex_name_uses_local_namespace_for_current_session_scope() {
        assert_eq!(
            mutex_name("demo-app", InstanceScope::CurrentSession),
            "Local\\win_desktop_utils_demo-app"
        );
    }

    #[test]
    fn mutex_name_uses_global_namespace_for_global_scope() {
        assert_eq!(
            mutex_name("demo-app", InstanceScope::Global),
            "Global\\win_desktop_utils_demo-app"
        );
    }

    #[test]
    fn options_default_to_current_session_scope() {
        let options = SingleInstanceOptions::new("demo-app");
        assert_eq!(options.app_id(), "demo-app");
        assert_eq!(options.configured_scope(), InstanceScope::CurrentSession);
    }

    #[test]
    fn options_can_use_global_scope() {
        let options = SingleInstanceOptions::global("demo-app");
        assert_eq!(options.configured_scope(), InstanceScope::Global);
    }
}

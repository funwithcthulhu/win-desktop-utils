//! App-startup facade for identity, app-data paths, and single-instance guards.

use std::path::PathBuf;

use crate::error::{Error, Result};
use crate::instance::{
    single_instance_with_options, InstanceGuard, InstanceScope, SingleInstanceOptions,
};
use crate::paths::{
    ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data,
};

/// Validated Windows desktop app identity used by the app-data and single-instance helpers.
///
/// `DesktopApp` keeps a validated app identity in one place and uses it for app-data
/// paths and single-instance locking. It does not own any global state.
///
/// # Examples
///
/// ```
/// let app = win_desktop_utils::DesktopApp::new(format!(
///     "demo-app-{}",
///     std::process::id()
/// ))?;
///
/// let local = app.ensure_local_data_dir()?;
/// assert!(local.exists());
/// # Ok::<(), win_desktop_utils::Error>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopApp {
    company_name: Option<String>,
    app_name: String,
    app_dir_name: String,
    app_id: String,
    instance_scope: InstanceScope,
}

impl DesktopApp {
    /// Creates a desktop app identity without a company namespace.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] if `app_name` is empty, contains NUL bytes,
    /// or contains characters that are invalid in Windows file names.
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
    ///
    /// App-data paths are nested as `Company\App`, while the default single-instance
    /// mutex ID uses `Company.App`.
    ///
    /// # Examples
    ///
    /// ```
    /// let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
    ///
    /// assert_eq!(app.company_name(), Some("Acme"));
    /// assert_eq!(app.app_name(), "Editor");
    /// assert_eq!(app.app_dir_name(), "Acme\\Editor");
    /// assert_eq!(app.app_id(), "Acme.Editor");
    /// # Ok::<(), win_desktop_utils::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidInput`] if either identity part is empty, contains NUL
    /// bytes, or contains characters that are invalid in Windows file names.
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
    ///
    /// # Examples
    ///
    /// ```
    /// let app = win_desktop_utils::DesktopApp::new("Admin Tool")?
    ///     .instance_scope(win_desktop_utils::InstanceScope::Global);
    ///
    /// assert_eq!(
    ///     app.configured_instance_scope(),
    ///     win_desktop_utils::InstanceScope::Global,
    /// );
    /// # Ok::<(), win_desktop_utils::Error>(())
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```
    /// let app = win_desktop_utils::DesktopApp::with_company("Acme", "Editor")?;
    /// let options = app.single_instance_options();
    ///
    /// assert_eq!(options.app_id(), "Acme.Editor");
    /// assert_eq!(
    ///     options.configured_scope(),
    ///     win_desktop_utils::InstanceScope::CurrentSession,
    /// );
    /// # Ok::<(), win_desktop_utils::Error>(())
    /// ```
    pub fn single_instance_options(&self) -> SingleInstanceOptions {
        SingleInstanceOptions::new(self.app_id.clone()).scope(self.instance_scope)
    }

    /// Attempts to acquire the configured single-instance guard for this app.
    pub fn single_instance(&self) -> Result<Option<InstanceGuard>> {
        single_instance_with_options(&self.single_instance_options())
    }
}

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

#[cfg(test)]
mod tests {
    use super::{validate_identity_part, DesktopApp};
    use crate::InstanceScope;

    #[test]
    fn desktop_app_uses_app_name_for_dir_and_id() {
        let app = DesktopApp::new("Demo App").unwrap();
        assert_eq!(app.company_name(), None);
        assert_eq!(app.app_name(), "Demo App");
        assert_eq!(app.app_dir_name(), "Demo App");
        assert_eq!(app.app_id(), "Demo App");
        assert_eq!(
            app.configured_instance_scope(),
            InstanceScope::CurrentSession
        );
    }

    #[test]
    fn desktop_app_with_company_uses_nested_dir_and_dotted_id() {
        let app = DesktopApp::with_company("Demo Company", "Demo App")
            .unwrap()
            .instance_scope(InstanceScope::Global);

        assert_eq!(app.company_name(), Some("Demo Company"));
        assert_eq!(app.app_dir_name(), "Demo Company\\Demo App");
        assert_eq!(app.app_id(), "Demo Company.Demo App");
        assert_eq!(app.configured_instance_scope(), InstanceScope::Global);
    }

    #[test]
    fn validate_identity_rejects_empty_string() {
        let result = validate_identity_part("app_name", "   ".to_owned());
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput("app_name cannot be empty"))
        ));
    }

    #[test]
    fn validate_identity_rejects_nul_bytes() {
        let result = validate_identity_part("company_name", "Demo\0Company".to_owned());
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "company_name cannot contain NUL bytes"
            ))
        ));
    }

    #[test]
    fn validate_identity_rejects_path_separators() {
        let result = validate_identity_part("app_name", "Demo\\App".to_owned());
        assert!(matches!(
            result,
            Err(crate::Error::InvalidInput(
                "app_name contains invalid Windows file-name characters"
            ))
        ));
    }

    #[test]
    fn validate_identity_rejects_all_windows_file_name_reserved_characters() {
        for reserved in ['<', '>', ':', '"', '/', '\\', '|', '?', '*'] {
            let value = format!("Demo{reserved}App");
            let result = validate_identity_part("app_name", value);

            assert!(matches!(
                result,
                Err(crate::Error::InvalidInput(
                    "app_name contains invalid Windows file-name characters"
                ))
            ));
        }
    }

    #[test]
    fn validate_identity_trims_surrounding_whitespace() {
        assert_eq!(
            validate_identity_part("company_name", "  Demo Company  ".to_owned()).unwrap(),
            "Demo Company"
        );
    }
}

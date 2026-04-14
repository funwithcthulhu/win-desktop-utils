use std::ffi::OsStr;

use win_desktop_utils::{
    ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data, Error,
};

#[test]
fn roaming_app_data_rejects_empty_name() {
    let result = roaming_app_data("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn local_app_data_rejects_empty_name() {
    let result = local_app_data("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn roaming_app_data_appends_app_name() {
    let path = roaming_app_data("demo-app").unwrap();
    assert_eq!(path.file_name(), Some(OsStr::new("demo-app")));
}

#[test]
fn local_app_data_appends_app_name() {
    let path = local_app_data("demo-app").unwrap();
    assert_eq!(path.file_name(), Some(OsStr::new("demo-app")));
}

#[test]
fn ensure_roaming_app_data_creates_directory() {
    let path = ensure_roaming_app_data("demo-app").unwrap();
    assert!(path.exists());
    assert!(path.is_dir());
}

#[test]
fn ensure_local_app_data_creates_directory() {
    let path = ensure_local_app_data("demo-app").unwrap();
    assert!(path.exists());
    assert!(path.is_dir());
}

#![cfg(windows)]

use std::ffi::OsStr;
use std::path::Path;

use win_desktop_utils::{
    ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data, Error,
};

fn unique_app_name(prefix: &str) -> String {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();

    format!(
        "win-desktop-utils-test-{prefix}-{}-{unique}",
        std::process::id()
    )
}

fn assert_app_data_path_shape(path: &Path, app_name: &str) {
    assert!(path.is_absolute());
    assert_eq!(path.file_name(), Some(OsStr::new(app_name)));

    let parent = path
        .parent()
        .expect("app-data path should have a known-folder parent");
    assert!(parent.exists());
    assert!(parent.is_dir());
}

#[test]
fn roaming_app_data_rejects_empty_name() {
    let result = roaming_app_data("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn roaming_app_data_rejects_nul_bytes() {
    let result = roaming_app_data("demo\0app");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn local_app_data_rejects_empty_name() {
    let result = local_app_data("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn local_app_data_rejects_nul_bytes() {
    let result = local_app_data("demo\0app");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn roaming_app_data_appends_app_name() {
    let app_name = unique_app_name("roaming");
    let path = roaming_app_data(&app_name).unwrap();
    assert_eq!(path.file_name(), Some(OsStr::new(&app_name)));
}

#[test]
fn local_app_data_appends_app_name() {
    let app_name = unique_app_name("local");
    let path = local_app_data(&app_name).unwrap();
    assert_eq!(path.file_name(), Some(OsStr::new(&app_name)));
}

#[test]
fn roaming_app_data_returns_absolute_child_path_without_creating_it() {
    let app_name = unique_app_name("roaming-shape");
    let path = roaming_app_data(&app_name).unwrap();

    assert_app_data_path_shape(&path, &app_name);
    assert!(!path.exists());
}

#[test]
fn local_app_data_returns_absolute_child_path_without_creating_it() {
    let app_name = unique_app_name("local-shape");
    let path = local_app_data(&app_name).unwrap();

    assert_app_data_path_shape(&path, &app_name);
    assert!(!path.exists());
}

#[test]
fn ensure_roaming_app_data_creates_directory() {
    let app_name = unique_app_name("ensure-roaming");
    let path = ensure_roaming_app_data(&app_name).unwrap();
    assert!(path.exists());
    assert!(path.is_dir());

    std::fs::remove_dir_all(path).unwrap();
}

#[test]
fn ensure_local_app_data_creates_directory() {
    let app_name = unique_app_name("ensure-local");
    let path = ensure_local_app_data(&app_name).unwrap();
    assert!(path.exists());
    assert!(path.is_dir());

    std::fs::remove_dir_all(path).unwrap();
}

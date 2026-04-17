use std::path::PathBuf;

use win_desktop_utils::{
    move_to_recycle_bin, open_url, open_with_default, reveal_in_explorer, single_instance, Error,
};

#[test]
fn open_with_default_rejects_empty_path() {
    let result = open_with_default(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_url_rejects_empty_string() {
    let result = open_url("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_url_rejects_whitespace_only() {
    let result = open_url("   ");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn reveal_in_explorer_rejects_empty_path() {
    let result = reveal_in_explorer(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_to_recycle_bin_rejects_empty_path() {
    let result = move_to_recycle_bin(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_to_recycle_bin_rejects_relative_path() {
    let result = move_to_recycle_bin(PathBuf::from("relative-file.txt"));
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_to_recycle_bin_rejects_missing_absolute_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-file.tmp");
    let result = move_to_recycle_bin(path);
    assert!(matches!(result, Err(Error::Io(_))));
}

#[test]
fn single_instance_rejects_empty_app_id() {
    let result = single_instance("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

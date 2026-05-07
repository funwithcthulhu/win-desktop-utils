#![cfg(not(windows))]

use std::ffi::OsString;
use std::path::PathBuf;

use win_desktop_utils::{
    create_shortcut, create_url_shortcut, empty_recycle_bin, empty_recycle_bin_for_root,
    ensure_local_app_data, ensure_roaming_app_data, is_elevated, local_app_data,
    move_paths_to_recycle_bin, move_to_recycle_bin, open_containing_folder, open_url,
    open_with_default, open_with_verb, print_with_default, restart_as_admin, reveal_in_explorer,
    roaming_app_data, run_as_admin, run_with_verb, show_properties, DesktopApp, Error,
    InstanceScope, ShortcutOptions, SingleInstanceOptions,
};

fn assert_unsupported<T>(result: win_desktop_utils::Result<T>) {
    assert!(matches!(result, Err(Error::Unsupported(_))));
}

#[test]
fn app_operations_return_unsupported() {
    let app = DesktopApp::with_company("Acme", "Demo").unwrap();

    assert_unsupported(app.local_data_dir());
    assert_unsupported(app.roaming_data_dir());
    assert_unsupported(app.ensure_local_data_dir());
    assert_unsupported(app.ensure_roaming_data_dir());
    assert_unsupported(app.single_instance());
}

#[test]
fn path_helpers_return_unsupported() {
    assert_unsupported(local_app_data("Demo"));
    assert_unsupported(roaming_app_data("Demo"));
    assert_unsupported(ensure_local_app_data("Demo"));
    assert_unsupported(ensure_roaming_app_data("Demo"));
}

#[test]
fn single_instance_helpers_return_unsupported() {
    let options = SingleInstanceOptions::new("demo").scope(InstanceScope::CurrentSession);

    assert_unsupported(win_desktop_utils::single_instance("demo"));
    assert_unsupported(win_desktop_utils::single_instance_with_scope(
        "demo",
        InstanceScope::Global,
    ));
    assert_unsupported(win_desktop_utils::single_instance_with_options(&options));
    assert_unsupported(options.acquire());
}

#[test]
fn shell_and_recycle_bin_helpers_return_unsupported() {
    let path = PathBuf::from("/tmp/win-desktop-utils-demo.txt");

    assert_unsupported(open_with_default(&path));
    assert_unsupported(open_with_verb("open", &path));
    assert_unsupported(show_properties(&path));
    assert_unsupported(print_with_default(&path));
    assert_unsupported(open_url("https://example.com"));
    assert_unsupported(reveal_in_explorer(&path));
    assert_unsupported(open_containing_folder(&path));
    assert_unsupported(move_to_recycle_bin(&path));
    assert_unsupported(move_paths_to_recycle_bin([&path]));
    assert_unsupported(empty_recycle_bin());
    assert_unsupported(empty_recycle_bin_for_root("/"));
}

#[test]
fn shortcut_and_elevation_helpers_return_unsupported() {
    let args = [OsString::from("--demo")];
    let options = ShortcutOptions::new().argument("--demo");

    assert_unsupported(create_shortcut("/tmp/demo.lnk", "/tmp/demo.exe", &options));
    assert_unsupported(create_url_shortcut("/tmp/demo.url", "https://example.com"));
    assert_unsupported(is_elevated());
    assert_unsupported(restart_as_admin(&args));
    assert_unsupported(run_as_admin("demo.exe", &args));
    assert_unsupported(run_with_verb("open", "demo.exe", &args));
}

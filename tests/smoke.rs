#![cfg(windows)]

use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use win_desktop_utils::{
    create_shortcut, create_url_shortcut, is_elevated, move_to_recycle_bin, open_containing_folder,
    open_url, reveal_in_explorer, run_as_admin, ShortcutOptions,
};

fn unique_temp_path(extension: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "win-desktop-utils-smoke-{}-{unique}.{extension}",
        std::process::id()
    ))
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn creates_lnk_shortcut() {
    let shortcut = unique_temp_path("lnk");
    let target = std::env::current_exe().expect("test executable path should resolve");
    let options = ShortcutOptions::new()
        .argument("--smoke")
        .working_directory(target.parent().expect("test executable should have parent"))
        .description("win-desktop-utils smoke shortcut");

    create_shortcut(&shortcut, &target, &options).expect("shortcut creation should succeed");
    assert!(shortcut.exists());

    fs::remove_file(shortcut).expect("shortcut cleanup should succeed");
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn creates_url_shortcut() {
    let shortcut = unique_temp_path("url");

    create_url_shortcut(&shortcut, "https://docs.rs/win-desktop-utils")
        .expect("url shortcut creation should succeed");

    let body = fs::read_to_string(&shortcut).expect("url shortcut should be readable");
    assert!(body.contains("URL=https://docs.rs/win-desktop-utils"));

    fs::remove_file(shortcut).expect("url shortcut cleanup should succeed");
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn recycles_temp_file() {
    let path = unique_temp_path("txt");
    fs::write(&path, "win-desktop-utils smoke recycle test")
        .expect("temp smoke file should be writable");

    move_to_recycle_bin(&path).expect("moving temp smoke file to Recycle Bin should succeed");
    assert!(!path.exists());
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn checks_elevation_state() {
    let _ = is_elevated().expect("elevation check should succeed");
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn opens_explorer_when_opted_in() {
    if std::env::var_os("WIN_DESKTOP_UTILS_SMOKE_UI").is_none() {
        eprintln!("skipping Explorer UI smoke test; set WIN_DESKTOP_UTILS_SMOKE_UI=1");
        return;
    }

    let path = unique_temp_path("txt");
    fs::write(&path, "win-desktop-utils smoke Explorer test")
        .expect("temp smoke file should be writable");

    reveal_in_explorer(&path).expect("Explorer reveal should start");
    open_containing_folder(&path).expect("containing folder should open");

    fs::remove_file(path).expect("Explorer smoke file cleanup should succeed");
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn opens_url_when_opted_in() {
    if std::env::var_os("WIN_DESKTOP_UTILS_SMOKE_UI").is_none() {
        eprintln!("skipping browser UI smoke test; set WIN_DESKTOP_UTILS_SMOKE_UI=1");
        return;
    }

    open_url("https://docs.rs/win-desktop-utils").expect("URL open should start");
}

#[test]
#[ignore = "manual desktop smoke test; run with `cargo xtask smoke`"]
fn launches_elevated_command_when_opted_in() {
    if std::env::var_os("WIN_DESKTOP_UTILS_SMOKE_ELEVATION").is_none() {
        eprintln!("skipping elevation launch smoke test; set WIN_DESKTOP_UTILS_SMOKE_ELEVATION=1");
        return;
    }

    let args = [OsString::from("/c"), OsString::from("exit")];
    run_as_admin("cmd.exe", &args).expect("elevated command launch should start");
}

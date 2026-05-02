use std::ffi::OsString;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use win_desktop_utils::{
    create_shortcut, create_url_shortcut, empty_recycle_bin_for_root, move_paths_to_recycle_bin,
    move_to_recycle_bin, open_containing_folder, open_url, open_with_default, open_with_verb,
    reveal_in_explorer, run_as_admin, run_with_verb, single_instance, single_instance_with_options,
    single_instance_with_scope, Error, InstanceScope, ShortcutOptions, SingleInstanceOptions,
};

#[test]
fn open_with_default_rejects_empty_path() {
    let result = open_with_default(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_with_default_rejects_missing_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-open.tmp");
    let result = open_with_default(path);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
}

#[test]
fn open_with_verb_rejects_empty_verb() {
    let result = open_with_verb("", r"C:\Windows\notepad.exe");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_with_verb_rejects_nul_bytes_in_verb() {
    let result = open_with_verb("pro\0perties", r"C:\Windows\notepad.exe");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_with_verb_rejects_empty_path() {
    let result = open_with_verb("open", PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_with_verb_rejects_missing_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-open-verb.tmp");
    let result = open_with_verb("open", path);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
}

#[test]
fn open_containing_folder_rejects_empty_path() {
    let result = open_containing_folder(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn open_containing_folder_rejects_missing_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-containing.tmp");
    let result = open_containing_folder(path);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
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
fn open_url_rejects_nul_bytes() {
    let result = open_url("https://example.com/\0hidden");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn reveal_in_explorer_rejects_empty_path() {
    let result = reveal_in_explorer(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn reveal_in_explorer_rejects_missing_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-reveal.tmp");
    let result = reveal_in_explorer(path);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
}

#[test]
fn move_to_recycle_bin_rejects_empty_path() {
    let result = move_to_recycle_bin(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_to_recycle_bin_rejects_relative_path() {
    let result = move_to_recycle_bin(PathBuf::from("relative-file.txt"));
    assert!(matches!(result, Err(Error::PathNotAbsolute)));
}

#[test]
fn move_to_recycle_bin_rejects_missing_absolute_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-file.tmp");
    let result = move_to_recycle_bin(path);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
}

#[test]
fn move_paths_to_recycle_bin_rejects_empty_collection() {
    let paths: [PathBuf; 0] = [];
    let result = move_paths_to_recycle_bin(paths);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_paths_to_recycle_bin_rejects_empty_path() {
    let result = move_paths_to_recycle_bin([PathBuf::new()]);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn move_paths_to_recycle_bin_rejects_relative_path() {
    let result = move_paths_to_recycle_bin([PathBuf::from("relative-file.txt")]);
    assert!(matches!(result, Err(Error::PathNotAbsolute)));
}

#[test]
fn move_paths_to_recycle_bin_rejects_missing_absolute_path() {
    let path = PathBuf::from(r"C:\definitely-does-not-exist-win-desktop-utils-test-file-batch.tmp");
    let result = move_paths_to_recycle_bin([path]);
    assert!(matches!(result, Err(Error::PathDoesNotExist)));
}

#[test]
fn empty_recycle_bin_for_root_rejects_empty_path() {
    let result = empty_recycle_bin_for_root(PathBuf::new());
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn empty_recycle_bin_for_root_rejects_relative_path() {
    let result = empty_recycle_bin_for_root(PathBuf::from("relative-root"));
    assert!(matches!(result, Err(Error::PathNotAbsolute)));
}

#[test]
fn run_as_admin_rejects_empty_executable() {
    let args: [OsString; 0] = [];
    let result = run_as_admin("", &args);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn run_with_verb_rejects_empty_verb() {
    let args: [OsString; 0] = [];
    let result = run_with_verb("", "cmd.exe", &args);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn run_with_verb_rejects_nul_bytes_in_executable() {
    let args: [OsString; 0] = [];
    let result = run_with_verb("open", "cmd\0.exe", &args);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn run_with_verb_rejects_nul_bytes_in_arguments() {
    let args = [OsString::from("hello\0world")];
    let result = run_with_verb("open", "cmd.exe", &args);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn create_shortcut_rejects_empty_shortcut_path() {
    let options = ShortcutOptions::new();
    let result = create_shortcut(PathBuf::new(), r"C:\Windows\notepad.exe", &options);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn create_shortcut_rejects_relative_shortcut_path() {
    let options = ShortcutOptions::new();
    let result = create_shortcut("demo.lnk", r"C:\Windows\notepad.exe", &options);
    assert!(matches!(result, Err(Error::PathNotAbsolute)));
}

#[test]
fn create_shortcut_rejects_wrong_extension() {
    let options = ShortcutOptions::new();
    let path = std::env::temp_dir().join("win-desktop-utils-shortcut-test.txt");
    let result = create_shortcut(path, r"C:\Windows\notepad.exe", &options);
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn create_shortcut_rejects_relative_target_path() {
    let options = ShortcutOptions::new();
    let path = std::env::temp_dir().join("win-desktop-utils-shortcut-test.lnk");
    let result = create_shortcut(path, "relative-target.exe", &options);
    assert!(matches!(result, Err(Error::PathNotAbsolute)));
}

#[test]
fn create_shortcut_creates_lnk_file() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "win-desktop-utils-shortcut-test-{}-{unique}.lnk",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);

    let target = std::env::current_exe().unwrap();
    let working_directory = target.parent().unwrap();
    let options = ShortcutOptions::new()
        .argument("--help")
        .working_directory(working_directory)
        .description("win-desktop-utils test shortcut");

    create_shortcut(&path, &target, &options).unwrap();
    assert!(path.exists());

    std::fs::remove_file(path).unwrap();
}

#[test]
fn create_url_shortcut_rejects_wrong_extension() {
    let path = std::env::temp_dir().join("win-desktop-utils-url-shortcut-test.txt");
    let result = create_url_shortcut(path, "https://example.com");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn create_url_shortcut_rejects_line_breaks() {
    let path = std::env::temp_dir().join("win-desktop-utils-url-shortcut-test.url");
    let result = create_url_shortcut(path, "https://example.com/\nIconFile=bad.ico");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn single_instance_rejects_empty_app_id() {
    let result = single_instance("");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn single_instance_rejects_backslashes_in_app_id() {
    let result = single_instance(r"demo\app");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn single_instance_rejects_nul_bytes_in_app_id() {
    let result = single_instance("demo\0app");
    assert!(matches!(result, Err(Error::InvalidInput(_))));
}

#[test]
fn single_instance_with_scope_allows_global_scope() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_id = format!(
        "win-desktop-utils-test-global-{}-{}",
        std::process::id(),
        unique
    );

    let first = single_instance_with_scope(&app_id, InstanceScope::Global).unwrap();
    assert!(first.is_some());

    let second = single_instance_with_scope(&app_id, InstanceScope::Global).unwrap();
    assert!(second.is_none());
}

#[test]
fn single_instance_with_options_allows_global_scope() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_id = format!(
        "win-desktop-utils-test-options-{}-{}",
        std::process::id(),
        unique
    );
    let options = SingleInstanceOptions::global(app_id);

    let first = single_instance_with_options(&options).unwrap();
    assert!(first.is_some());

    let second = options.acquire().unwrap();
    assert!(second.is_none());
}

#[test]
fn single_instance_scopes_do_not_conflict_with_each_other() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_id = format!(
        "win-desktop-utils-test-scope-separation-{}-{}",
        std::process::id(),
        unique
    );

    let local = single_instance_with_scope(&app_id, InstanceScope::CurrentSession).unwrap();
    assert!(local.is_some());

    let global = single_instance_with_scope(&app_id, InstanceScope::Global).unwrap();
    assert!(global.is_some());
}

#[test]
fn single_instance_returns_none_when_same_app_id_is_acquired_twice() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_id = format!("win-desktop-utils-test-{}-{}", std::process::id(), unique);

    let first = single_instance(&app_id).unwrap();
    assert!(first.is_some());

    let second = single_instance(&app_id).unwrap();
    assert!(second.is_none());
}

#[test]
fn single_instance_can_be_acquired_again_after_guard_is_dropped() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let app_id = format!(
        "win-desktop-utils-test-drop-{}-{}",
        std::process::id(),
        unique
    );

    {
        let first = single_instance(&app_id).unwrap();
        assert!(first.is_some());
    }

    let second = single_instance(&app_id).unwrap();
    assert!(second.is_some());
}

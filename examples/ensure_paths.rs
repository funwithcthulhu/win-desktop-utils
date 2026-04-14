fn main() {
    let roaming = win_desktop_utils::ensure_roaming_app_data("demo-app").unwrap();
    let local = win_desktop_utils::ensure_local_app_data("demo-app").unwrap();

    println!("created/ensured roaming: {}", roaming.display());
    println!("created/ensured local: {}", local.display());
}

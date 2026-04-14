fn main() {
    let roaming = win_desktop_utils::roaming_app_data("demo-app").unwrap();
    let local = win_desktop_utils::local_app_data("demo-app").unwrap();

    println!("roaming: {}", roaming.display());
    println!("local: {}", local.display());
}

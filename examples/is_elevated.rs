fn main() {
    let elevated = win_desktop_utils::is_elevated().unwrap();
    println!("is_elevated: {}", elevated);
}

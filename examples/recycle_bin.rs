fn main() {
    let path = std::env::current_dir().unwrap().join("temp-delete-me.txt");
    std::fs::write(&path, "temporary file").unwrap();

    win_desktop_utils::move_to_recycle_bin(&path).unwrap();

    println!("sent to recycle bin: {}", path.display());
}

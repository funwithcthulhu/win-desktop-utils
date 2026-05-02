fn main() -> Result<(), Box<dyn std::error::Error>> {
    let first = std::env::current_dir()?.join("temp-delete-me-a.txt");
    let second = std::env::current_dir()?.join("temp-delete-me-b.txt");
    std::fs::write(&first, "temporary file")?;
    std::fs::write(&second, "temporary file")?;

    win_desktop_utils::move_paths_to_recycle_bin([&first, &second])?;

    println!("sent to recycle bin: {}", first.display());
    println!("sent to recycle bin: {}", second.display());

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::current_dir()?.join("temp-delete-me.txt");
    std::fs::write(&path, "temporary file")?;

    win_desktop_utils::move_to_recycle_bin(&path)?;

    println!("sent to recycle bin: {}", path.display());

    Ok(())
}

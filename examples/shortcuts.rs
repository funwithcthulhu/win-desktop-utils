fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = std::env::current_dir()?;
    let notepad = r"C:\Windows\notepad.exe";

    let lnk_path = cwd.join("notepad-demo.lnk");
    let options = win_desktop_utils::ShortcutOptions::new()
        .description("Open Notepad")
        .working_directory(r"C:\Windows")
        .icon(notepad, 0);

    win_desktop_utils::create_shortcut(&lnk_path, notepad, &options)?;
    println!("created shortcut: {}", lnk_path.display());

    let url_path = cwd.join("rust-docs.url");
    win_desktop_utils::create_url_shortcut(&url_path, "https://doc.rust-lang.org/std/")?;
    println!("created URL shortcut: {}", url_path.display());

    Ok(())
}

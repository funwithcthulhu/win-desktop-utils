fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = win_desktop_utils::DesktopApp::with_company("Acme", "Demo Utility")?;

    let _guard = match app.single_instance()? {
        Some(guard) => guard,
        None => {
            println!("{} is already running", app.app_name());
            return Ok(());
        }
    };

    let local_dir = app.ensure_local_data_dir()?;
    let state_file = local_dir.join("state.txt");
    std::fs::write(&state_file, "started\n")?;

    let docs_shortcut = local_dir.join("win-desktop-utils docs.url");
    win_desktop_utils::create_url_shortcut(&docs_shortcut, "https://docs.rs/win-desktop-utils")?;

    println!("app id: {}", app.app_id());
    println!("local data: {}", local_dir.display());
    println!("state file: {}", state_file.display());
    println!("docs shortcut: {}", docs_shortcut.display());

    Ok(())
}

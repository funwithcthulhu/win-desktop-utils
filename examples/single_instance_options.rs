fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = win_desktop_utils::SingleInstanceOptions::new("demo-app-options")
        .scope(win_desktop_utils::InstanceScope::CurrentSession);

    match options.acquire()? {
        Some(_guard) => {
            println!("first instance using options");
            println!("press Enter to exit");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s)?;
        }
        None => {
            println!("already running");
        }
    }

    Ok(())
}

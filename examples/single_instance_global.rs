fn main() -> Result<(), Box<dyn std::error::Error>> {
    match win_desktop_utils::single_instance_with_scope(
        "demo-app-global",
        win_desktop_utils::InstanceScope::Global,
    )? {
        Some(_guard) => {
            println!("first instance across all sessions");
            println!("press Enter to exit");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s)?;
        }
        None => {
            println!("already running in another session or this one");
        }
    }

    Ok(())
}

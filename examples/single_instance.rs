fn main() -> Result<(), Box<dyn std::error::Error>> {
    match win_desktop_utils::single_instance("demo-app")? {
        Some(_guard) => {
            println!("first instance");
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

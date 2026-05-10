fn main() -> Result<(), Box<dyn std::error::Error>> {
    match win_desktop_utils::single_instance("demo-app")? {
        Some(_guard) => {
            println!("single-instance guard acquired for this session");
            println!("keep the guard value alive while your app is running");
        }
        None => {
            println!("already running in this session");
        }
    }

    Ok(())
}

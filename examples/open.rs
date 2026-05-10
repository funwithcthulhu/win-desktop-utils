fn main() -> Result<(), win_desktop_utils::Error> {
    println!("opening a URL through the Windows shell");
    println!("this may switch focus or open a browser tab/window");
    win_desktop_utils::open_url("https://docs.rs/win-desktop-utils")?;

    Ok(())
}

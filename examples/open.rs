fn main() -> Result<(), win_desktop_utils::Error> {
    win_desktop_utils::show_properties(r"C:\Windows\notepad.exe")?;
    win_desktop_utils::open_containing_folder(r"C:\Windows\notepad.exe")?;
    win_desktop_utils::open_url(" https://www.rust-lang.org ")?;
    Ok(())
}

fn main() -> Result<(), win_desktop_utils::Error> {
    win_desktop_utils::reveal_in_explorer(r"C:\Windows\notepad.exe")?;
    Ok(())
}

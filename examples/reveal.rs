fn main() -> Result<(), win_desktop_utils::Error> {
    let path = r"C:\Windows\notepad.exe";

    println!("starting Explorer to select an existing file");
    println!("this does not delete, move, or modify the file: {path}");
    win_desktop_utils::reveal_in_explorer(path)?;

    Ok(())
}

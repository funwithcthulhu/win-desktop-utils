use std::ffi::OsString;

fn main() -> Result<(), win_desktop_utils::Error> {
    let args = [OsString::from("--help")];
    win_desktop_utils::run_with_verb("open", "notepad.exe", &args)?;
    Ok(())
}

use std::ffi::OsString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if win_desktop_utils::is_elevated()? {
        println!("already elevated");
        println!("press Enter to exit");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s)?;
    } else {
        println!("requesting elevation");
        let args = [OsString::from("--help")];
        win_desktop_utils::restart_as_admin(&args)?;
    }

    Ok(())
}

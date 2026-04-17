use std::ffi::OsString;

fn main() {
    if win_desktop_utils::is_elevated().unwrap() {
        println!("already elevated");
        println!("press Enter to exit");
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
    } else {
        println!("requesting elevation");
        let args: [OsString; 0] = [];
        win_desktop_utils::restart_as_admin(&args).unwrap();
    }
}

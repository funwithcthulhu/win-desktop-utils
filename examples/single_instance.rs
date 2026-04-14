fn main() {
    match win_desktop_utils::single_instance("demo-app").unwrap() {
        Some(_guard) => {
            println!("first instance");
            println!("press Enter to exit");
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
        }
        None => {
            println!("already running");
        }
    }
}

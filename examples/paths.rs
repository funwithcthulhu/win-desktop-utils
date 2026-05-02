fn main() -> Result<(), win_desktop_utils::Error> {
    let roaming = win_desktop_utils::roaming_app_data("demo-app")?;
    let local = win_desktop_utils::local_app_data("demo-app")?;

    println!("roaming: {}", roaming.display());
    println!("local: {}", local.display());

    Ok(())
}

fn main() -> Result<(), win_desktop_utils::Error> {
    let roaming = win_desktop_utils::ensure_roaming_app_data("demo-app")?;
    let local = win_desktop_utils::ensure_local_app_data("demo-app")?;

    println!("created/ensured roaming: {}", roaming.display());
    println!("created/ensured local: {}", local.display());

    Ok(())
}

fn main() -> Result<(), win_desktop_utils::Error> {
    let elevated = win_desktop_utils::is_elevated()?;
    println!("is_elevated: {}", elevated);

    Ok(())
}

fn main() -> Result<(), win_desktop_utils::Error> {
    win_desktop_utils::open_url(" https://www.rust-lang.org ")?;
    Ok(())
}

use win_desktop_utils::Error;

#[test]
fn display_for_invalid_input_is_readable() {
    let err = Error::InvalidInput("path cannot be empty");
    assert_eq!(err.to_string(), "invalid input: path cannot be empty");
}

#[test]
fn display_for_path_not_absolute_is_readable() {
    let err = Error::PathNotAbsolute;
    assert_eq!(err.to_string(), "path must be absolute");
}

#[test]
fn display_for_path_does_not_exist_is_readable() {
    let err = Error::PathDoesNotExist;
    assert_eq!(err.to_string(), "path does not exist");
}

#[test]
fn display_for_windows_api_includes_context_and_code() {
    let err = Error::WindowsApi {
        context: "ShellExecuteW",
        code: 5,
    };
    assert_eq!(
        err.to_string(),
        "Windows API error in ShellExecuteW (code 5)"
    );
}

#[test]
fn io_error_exposes_source() {
    let err = Error::Io(std::io::Error::other("disk problem"));
    assert!(std::error::Error::source(&err).is_some());
}

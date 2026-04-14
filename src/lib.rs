pub mod elevation;
pub mod error;
pub mod instance;
pub mod paths;
pub mod shell;

pub use error::{Error, Result};

pub use elevation::{is_elevated, restart_as_admin};
pub use instance::{single_instance, InstanceGuard};
pub use paths::{ensure_local_app_data, ensure_roaming_app_data, local_app_data, roaming_app_data};
pub use shell::{open_url, open_with_default, reveal_in_explorer};

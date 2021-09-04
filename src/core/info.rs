pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

pub fn os_is_windows() -> bool {
    cfg!(target_os="windows")
}

/* pub fn os_is_linux() -> bool {
    cfg!(target_os="linux")
} */
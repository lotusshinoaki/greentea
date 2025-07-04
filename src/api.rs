#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use self::windows::*;

#[cfg(not(target_os = "windows"))]
pub mod others;
#[cfg(not(target_os = "windows"))]
pub use self::others::*;

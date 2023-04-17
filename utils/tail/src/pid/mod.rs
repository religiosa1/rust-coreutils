#[cfg(target_family = "unix")]
mod pid_unix;
#[cfg(target_family = "unix")]
pub use pid_unix::PidChecker;

#[cfg(target_family = "windows")]
mod pid_windows;
#[cfg(target_family = "windows")]
pub use pid_windows::PidChecker;

#[cfg(not(any(target_family = "unix", target_family = "windows")))]
mod pid_unknown;
#[cfg(not(any(target_family = "unix", target_family = "windows")))]
pub use pid_unknown::PidChecker;

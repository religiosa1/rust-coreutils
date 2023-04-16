#[cfg(target_family = "unix")]
mod pid_unix;
#[cfg(target_family = "unix")]
pub use pid_unix::check_pid;

#[cfg(not(target_family = "unix"))]
mod pid_unknown;
#[cfg(not(target_family = "unix"))]
pub use pid_unknown::check_pid;

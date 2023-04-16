use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

/** Determining a process status by sending an empty signal */
pub fn check_pid(pid: i32) -> bool {
    match kill(Pid::from_raw(pid), Signal::null()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

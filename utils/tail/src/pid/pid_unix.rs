use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

struct PidChecker {
    pid: Pid,
}
impl PidChecker {
    pub fn new(pid: u32) -> Self {
        Self {
            pid: Pid::from_raw(pid),
        }
    }
    /** Determining a process status by sending an empty signal */
    pub fn check_pid(&self) -> bool {
        match kill(self.pid, Signal::null()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

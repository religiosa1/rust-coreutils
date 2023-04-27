use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;

pub struct PidChecker {
    pid: Pid,
}
impl PidChecker {
    pub fn new(pid: u32) -> Result<Self, ()> {
        Self {
            pid: Pid::from_raw(pid.try_into().unwrap()),
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

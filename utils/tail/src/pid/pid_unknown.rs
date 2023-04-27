pub struct PidChecker;
impl PidChecker {
    pub fn new(_pid: u32) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "--pid argument isn't supported on the current platform",
        ))
    }

    pub fn check_pid(&self, _pid: u32) {
        panic!("--pid argument isn't supported on the current platform")
    }
}

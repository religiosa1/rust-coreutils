use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, WAIT_FAILED, WAIT_OBJECT_0};
use windows::Win32::System::Threading::{OpenProcess, WaitForSingleObject, PROCESS_SYNCHRONIZE};

pub struct PidChecker {
    handle: HANDLE,
}

impl PidChecker {
    pub fn new(pid: u32) -> windows::core::Result<Self> {
        let h = unsafe { OpenProcess(PROCESS_SYNCHRONIZE, BOOL(0), pid)? };
        Ok(Self { handle: h })
    }

    pub fn check_pid(&self) -> bool {
        unsafe {
            let status = WaitForSingleObject(self.handle, 0);
            !matches!(status, WAIT_OBJECT_0 | WAIT_FAILED)
        }
    }
}

impl Drop for PidChecker {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}

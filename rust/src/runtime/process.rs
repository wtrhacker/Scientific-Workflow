//! Shared ownership and cleanup of external-program process groups.

use std::process::{Child, Command};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};

static PROCESS_GROUPS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

// Keep the registry locked across launch and force-exit to prevent late children.
/// Spawn a command and retain ownership of its process group until cleanup.
///
/// On Unix, the caller must configure `command.process_group(0)` before launch.
pub fn spawn(command: &mut Command) -> std::io::Result<ProcessTree> {
    let mut groups = PROCESS_GROUPS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let child = command.spawn()?;
    groups.push(child.id());
    Ok(ProcessTree(child, false, false))
}

pub(crate) fn force_exit() -> ! {
    let groups = PROCESS_GROUPS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    #[cfg(unix)]
    for &pid in groups.iter() {
        // SAFETY: only groups created and retained by this runtime are registered.
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }
    #[cfg(unix)]
    for &pid in groups.iter() {
        // SAFETY: these are our direct children; another supervisor may have reaped them.
        unsafe {
            libc::waitpid(pid as i32, std::ptr::null_mut(), 0);
        }
    }
    std::process::exit(130)
}

/// An owned child and its process group, normally cleaned up on drop.
pub struct ProcessTree(pub(super) Child, bool, bool);
impl ProcessTree {
    /// Return the process-group leader's operating-system PID.
    #[cfg(feature = "external-tools")]
    pub fn id(&self) -> u32 {
        self.0.id()
    }

    /// Poll and reap the direct child without blocking.
    #[cfg(feature = "external-tools")]
    pub fn try_wait(&mut self) -> std::io::Result<Option<std::process::ExitStatus>> {
        self.0.try_wait()
    }

    pub(super) fn suspend(&mut self, paused: bool) -> std::io::Result<()> {
        if self.2 == paused {
            return Ok(());
        }
        #[cfg(unix)]
        {
            // SAFETY: only this task's exclusively owned process group is targeted.
            let result = unsafe {
                libc::kill(
                    -(self.0.id() as i32),
                    if paused { libc::SIGSTOP } else { libc::SIGCONT },
                )
            };
            if result != 0 {
                let error = std::io::Error::last_os_error();
                if error.raw_os_error() != Some(libc::ESRCH) {
                    return Err(error);
                }
            }
            self.2 = paused;
            Ok(())
        }
        #[cfg(not(unix))]
        {
            Err(std::io::Error::other(
                "disk pause of non-cooperative programs requires Unix process groups",
            ))
        }
    }
    pub(super) fn stop(&mut self) {
        if self.1 {
            return;
        }
        self.1 = true;
        let _ = self.suspend(false);
        #[cfg(unix)]
        // SAFETY: process_group(0) creates an exclusively owned group led by this child.
        unsafe {
            libc::kill(-(self.0.id() as i32), libc::SIGTERM);
        }
        let deadline = Instant::now() + Duration::from_millis(150);
        while self.0.try_wait().ok().flatten().is_none() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(5));
        }
        #[cfg(unix)]
        // SAFETY: cleanup targets only the group created for this task.
        unsafe {
            libc::kill(-(self.0.id() as i32), libc::SIGKILL);
        }
        let _ = self.0.kill();
        let _ = self.0.wait();
        PROCESS_GROUPS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .retain(|id| *id != self.0.id());
    }
}
impl Drop for ProcessTree {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(feature = "external-tools")]
impl ProcessTree {
    /// Defer automatic drop cleanup while an external resource requires teardown.
    ///
    /// The caller must eventually invoke [`Self::stop_after_teardown`]. Dropping
    /// a deferred tree intentionally leaves its processes running.
    pub fn disarm_cleanup(&mut self) {
        self.1 = true;
    }
    /// Restore normal cleanup and stop the group after external resources clear.
    pub fn stop_after_teardown(&mut self) {
        self.1 = false;
        self.stop();
    }
    /// Explicitly stop or continue the group, including after external signals.
    pub fn force_suspension(&mut self, paused: bool) -> std::io::Result<()> {
        self.2 = !paused;
        self.suspend(paused)
    }
}

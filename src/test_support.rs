//! Helpers shared by unit tests.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

/// Serializes tests that write fake executables or start processes.
///
/// When one test thread starts a process while another thread still has a
/// freshly written script open for writing, the child inherits that write
/// handle until it execs. Running the script then fails with ETXTBSY ("Text
/// file busy", os error 26). Every test that writes a fake program or reaches
/// code that spawns one holds this lock for its whole body, so a write and a
/// fork never overlap (#12).
pub(crate) fn exec_lock() -> MutexGuard<'static, ()> {
    static LOCK: Mutex<()> = Mutex::new(());
    // A failed test poisons the lock; the next test can still use it safely.
    LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Writes an executable `#!/bin/sh` script at `path`. Call it while holding
/// [`exec_lock`].
pub(crate) fn fake_program(path: &Path, body: &str) {
    fs::write(path, format!("#!/bin/sh\n{body}\n")).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

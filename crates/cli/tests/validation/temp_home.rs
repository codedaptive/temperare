//! `TempHome`: an empty folder made for one run of `cctao` to use as HOME, so no acceptance run reads the real
//! ~/.config or writes the real ~/.local. Removed when dropped, pass or fail.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Tells apart the homes made by one test process, whose cases run on several threads at once.
static NEXT: AtomicUsize = AtomicUsize::new(0);

pub struct TempHome(PathBuf);

impl TempHome {
    /// A new empty folder under the system's temporary folder, named for this process and a counter.
    pub fn new() -> TempHome {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("cctao-validation-home-{}-{n}", std::process::id()));
        std::fs::create_dir(&dir).unwrap_or_else(|e| panic!("could not make {}: {e}", dir.display()));
        TempHome(dir)
    }

    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempHome {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

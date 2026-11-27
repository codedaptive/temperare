//! `ScratchDir`: the folder one Periphery run builds and writes in, living as long as the value. It reads and writes
//! the disk, so its test is the zone integration test tests/periphery.rs and it has no unit-test sibling.

use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// Numbers the folders this process makes, so two runs at once never share a name.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// A new folder `cctao-periphery-<pid>-<n>` under the system's temporary folder, holding the build, the scratch paths of
/// the local packages' dump-packages (`dep-<n>`), the package description and the empty config of one run; dropping it
/// removes the folder and everything in it
/// (`scratch_dir_drop.rs`).
#[derive(Debug)]
pub(crate) struct ScratchDir {
    pub(crate) path: PathBuf,
}

impl ScratchDir {
    /// Creates the folder under `std::env::temp_dir()`, new, never over an existing one, so the value only ever owns
    /// a folder it made.
    pub(crate) fn new() -> io::Result<ScratchDir> {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("cctao-periphery-{}-{n}", std::process::id()));
        std::fs::create_dir(&path)?;
        Ok(ScratchDir { path })
    }
}

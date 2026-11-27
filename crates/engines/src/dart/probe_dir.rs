//! `ProbeDir`: the folder of one dart probe run, living as long as the value. It writes the disk, so its test is the
//! zone integration test tests/dart.rs and it has no unit-test sibling.

use std::io;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

/// Numbers the folders this process makes, so no two probes share one: each probe's command then differs by its
/// folder, and `run_group`, which replays a command it has already run, never replays one probe for another.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// A new folder `cctao-dart-probe-<pid>-<n>` under the system's temporary folder holding the probe package's files
/// (`files`, the paths to write in it, each added before its write so a failed write is removed too); dropping it
/// removes those files, then the folder (`probe_dir_drop.rs`).
#[derive(Debug)]
pub(crate) struct ProbeDir {
    pub(crate) path: PathBuf,
    pub(crate) files: Vec<PathBuf>,
}

impl ProbeDir {
    /// Creates the folder under `std::env::temp_dir()`, new, never over an existing one, so the value only ever owns
    /// a folder it made, and writes each of `files` (a name and its text) in it. On an error, what was made so far is
    /// removed as the value drops.
    pub(crate) fn new(files: &[(&str, String)]) -> io::Result<ProbeDir> {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!("cctao-dart-probe-{}-{n}", std::process::id()));
        std::fs::create_dir(&path)?;
        let mut dir = ProbeDir { path, files: Vec::new() };
        for (name, text) in files {
            let file = dir.path.join(name);
            dir.files.push(file.clone());
            std::fs::write(&file, text)?;
        }
        Ok(dir)
    }
}

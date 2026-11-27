//! `apply_install`: carries out an install plan on the disk: the binary and the skill files.
//!
//! No sibling unit test: it writes to the disk, which a unit test may not, so the zone integration target
//! `tests/install` (placement.rs, harnesses.rs, uninstall.rs) proves it on a temporary HOME.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use super::make_dirs::make_dirs;
use super::write_through_temp::write_through_temp;
use crate::{InstallPlan, RecordEntry, SKILL_TEXT};

/// Copies `exe` to `plan.place` and writes `SKILL_TEXT` to each skill file, making missing folders first. `placed`
/// comes in as the earlier install's record: anything at the binary path or a skill path that it does not name as
/// `RecordEntry::File` is not cctao's, so it is kept untouched and not recorded, with a line `kept <path>: ...` on
/// `out`. A path the record names is replaced (a reinstall). The binary and each skill file are written through a
/// temporary file beside them (`write_through_temp`: created exclusively as `.<name>.<pid>.<n>.installing`, so nothing
/// there is followed, and only a temporary file this run created is removed on a failure), so a path is never left
/// half-written, and each goes on `placed` right after its rename. Each folder created goes on `placed` as soon as it
/// exists, so the record is right even when a later step fails. Each step done is a line on `out`. Write errors on
/// `out` are ignored.
pub(crate) fn apply_install(
    plan: &InstallPlan,
    exe: &Path,
    placed: &mut Vec<RecordEntry>,
    out: &mut dyn Write,
) -> io::Result<()> {
    if let Some(place) = &plan.place {
        make_parent(place, placed)?;
        if !kept_foreign(place, placed, out)? {
            write_through_temp(place, &mut File::open(exe)?, true)?;
            placed.push(RecordEntry::File(place.clone()));
            let _ = writeln!(out, "placed {}", place.display());
        }
    }
    for skill in &plan.skills {
        make_parent(skill, placed)?;
        if kept_foreign(skill, placed, out)? {
            continue;
        }
        write_through_temp(skill, &mut SKILL_TEXT.as_bytes(), false)?;
        placed.push(RecordEntry::File(skill.clone()));
        let _ = writeln!(out, "wired skill {}", skill.display());
    }
    Ok(())
}

/// `make_dirs` on the folder holding `path`.
fn make_parent(path: &Path, placed: &mut Vec<RecordEntry>) -> io::Result<()> {
    match path.parent() {
        Some(parent) => make_dirs(parent, placed),
        None => Ok(()),
    }
}

/// True when something is at `path` (a link is not followed) that `placed` does not name as a file install placed;
/// it is kept, and a line on `out` says so.
fn kept_foreign(path: &Path, placed: &[RecordEntry], out: &mut dyn Write) -> io::Result<bool> {
    match fs::symlink_metadata(path) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
        Ok(_) if placed.contains(&RecordEntry::File(path.to_path_buf())) => Ok(false),
        Ok(_) => {
            let _ = writeln!(
                out,
                "kept {}: it was not placed by cctao install; move it away and run cctao install again",
                path.display()
            );
            Ok(true)
        }
    }
}

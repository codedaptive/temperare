//! `apply_install`: carries out an install plan on the disk: the binary, the `tm` link, the skill files.
//!
//! No sibling ZUT: it writes to the disk, which a unit test may not (rule 11), so the zone integration target
//! `tests/install` (placement.rs, harnesses.rs, uninstall.rs) proves it on a temporary HOME.

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

use super::make_dirs::make_dirs;
use super::write_through_temp::write_through_temp;
use crate::{InstallPlan, RecordEntry, SKILL_TEXT};

/// Copies `exe` to `plan.place`, makes the `tm` link (on unix; off unix it says on `out` that none was made and records
/// none) and writes `SKILL_TEXT` to each skill file, making missing folders first. `placed` comes in as the earlier
/// install's record: anything at the binary path or a skill path that it does not name as `RecordEntry::File` is not
/// temperare's, so it is kept untouched and not recorded, with a line `kept <path>: ...` on `out`; a kept binary also
/// means no `tm` link, with a line `tm not linked: <binary> was kept`. A path the record names is replaced (a
/// reinstall). The binary and each skill file are written through a temporary file beside them (`write_through_temp`:
/// created exclusively as `.<name>.<pid>.<n>.installing`, so nothing there is followed, and only a temporary file this
/// run created is removed on a failure), so a path is never left half-written, and each goes on `placed` right after
/// its rename. Each folder created goes on `placed` as soon as it exists, so the record is right even when a later step
/// fails; the link goes on as `RecordEntry::Link` with its target, whether made now or found already pointing there.
/// Each step done is a line on `out`. Something at the link path that is not already a symlink to the target is kept
/// and not recorded, with a line saying so. Write errors on `out` are ignored.
pub(crate) fn apply_install(
    plan: &InstallPlan,
    exe: &Path,
    placed: &mut Vec<RecordEntry>,
    out: &mut dyn Write,
) -> io::Result<()> {
    let mut link = plan.link.as_ref();
    if let Some(place) = &plan.place {
        make_parent(place, placed)?;
        if kept_foreign(place, placed, out)? {
            let _ = writeln!(out, "tm not linked: {} was kept", place.display());
            link = None;
        } else {
            write_through_temp(place, &mut File::open(exe)?, true)?;
            placed.push(RecordEntry::File(place.clone()));
            let _ = writeln!(out, "placed {}", place.display());
        }
    }
    if let Some((link, target)) = link {
        make_parent(link, placed)?;
        if make_link(link, target, out)? {
            placed.push(RecordEntry::Link(link.clone(), target.clone()));
            let _ = writeln!(out, "linked {} -> {}", link.display(), target.display());
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
                "kept {}: it was not placed by temperare install; move it away and run temperare install again",
                path.display()
            );
            Ok(true)
        }
    }
}

/// Makes `link` a symlink to `target`. True when the link is in place (made now, or already a symlink to
/// `target`); false when something else is at `link`, which is kept and named on `out`, or, off unix, when nothing
/// is there and no link is made (named on `out`).
fn make_link(link: &Path, target: &Path, out: &mut dyn Write) -> io::Result<bool> {
    match fs::symlink_metadata(link) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => new_link(link, target, out),
        Err(e) => Err(e),
        Ok(meta) if meta.file_type().is_symlink() && fs::read_link(link)? == target => Ok(true),
        Ok(_) => {
            let _ = writeln!(
                out,
                "kept {}: it is not a link to {}, so tm was not linked; move it away and run temperare install again",
                link.display(),
                target.display()
            );
            Ok(false)
        }
    }
}

/// Makes `link` a symlink to `target`, where nothing is at `link`.
#[cfg(unix)]
fn new_link(link: &Path, target: &Path, _out: &mut dyn Write) -> io::Result<bool> {
    std::os::unix::fs::symlink(target, link)?;
    Ok(true)
}

/// Off unix no link is made: says so on `out` and returns false.
#[cfg(not(unix))]
fn new_link(link: &Path, _target: &Path, out: &mut dyn Write) -> io::Result<bool> {
    let _ = writeln!(out, "not linked {}: links are made on unix only", link.display());
    Ok(false)
}

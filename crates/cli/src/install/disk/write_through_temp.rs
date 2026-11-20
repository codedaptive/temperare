//! `write_through_temp`: puts a whole file at a path through a temporary file beside it, never a partial one.
//!
//! No sibling unit test: it writes to the disk, which a unit test may not, so the zone integration target
//! `tests/install` proves it on a temporary HOME: placement.rs and harnesses.rs for the files install places, record.rs
//! for the install record `run_install` writes through it, temp_mode.rs for the temporary file's mode, end_modes.rs
//! for the placed files' modes under umask 002 and 077.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// The temporary file's mode while it is written: owner-only, so no group or other user (root aside) can read or
/// write it.
const TEMP_MODE: u32 = 0o600;

/// Copies `source` into a new temporary file beside `dest` and renames it over `dest`, so `dest` is either what it
/// was or the whole new file. The temporary file is `.<name>.<pid>.<n>.installing` (`name` is `dest`'s file name,
/// `pid` this process's id, `n` counting up from 0 past names already taken); it is created exclusively and written
/// through that handle, so nothing already there, a symlink included, is followed or overwritten. On unix the
/// temporary file is created and then set to 0o600 through that handle while it is written; after the copy and
/// before the rename it is set to 0o755 with `executable`, else 0o644. Both modes are set explicitly through
/// the handle, so the umask does not reduce them. On a failure only the temporary file this call created is removed.
pub(crate) fn write_through_temp(dest: &Path, source: &mut dyn Read, executable: bool) -> io::Result<()> {
    let (temp, mut file) = create_temp(dest)?;
    let written = set_mode(&file, TEMP_MODE)
        .and_then(|()| io::copy(source, &mut file))
        .and_then(|_| set_mode(&file, final_mode(executable)));
    drop(file);
    let result = written.and_then(|()| fs::rename(&temp, dest));
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

/// Creates the first free `.<name>.<pid>.<n>.installing` beside `dest`, open for writing. On unix its create mode is
/// 0o600, so the umask can only narrow it and it is never wider than owner-only; `write_through_temp` then sets 0o600
/// exactly through the handle.
fn create_temp(dest: &Path) -> io::Result<(PathBuf, File)> {
    let name = dest.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let pid = std::process::id();
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(TEMP_MODE);
    }
    let mut n: u64 = 0;
    loop {
        let temp = dest.with_file_name(format!(".{name}.{pid}.{n}.installing"));
        match options.open(&temp) {
            Ok(file) => return Ok((temp, file)),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => n += 1,
            Err(e) => return Err(e),
        }
    }
}

/// The mode a placed file ends with: 0o755 for the binary, 0o644 for skill files and the install record.
fn final_mode(executable: bool) -> u32 {
    if executable { 0o755 } else { 0o644 }
}

/// Sets `file` to exactly `mode`; set_permissions on the handle is not reduced by the umask, as a create mode is.
#[cfg(unix)]
fn set_mode(file: &File, mode: u32) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    file.set_permissions(fs::Permissions::from_mode(mode))
}

/// Off unix there is no mode bit to set.
#[cfg(not(unix))]
fn set_mode(_file: &File, _mode: u32) -> io::Result<()> {
    Ok(())
}

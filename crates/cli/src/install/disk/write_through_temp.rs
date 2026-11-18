//! `write_through_temp`: puts a whole file at a path through a temporary file beside it, never a partial one.
//!
//! No sibling ZUT: it writes to the disk, which a unit test may not (rule 11), so the zone integration target
//! `tests/install` proves it on a temporary HOME: placement.rs and harnesses.rs for the files install places, record.rs
//! for the install record `run_install` writes through it.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// Copies `source` into a new temporary file beside `dest` and renames it over `dest`, so `dest` is either what it
/// was or the whole new file. The temporary file is `.<name>.<pid>.<n>.installing` (`name` is `dest`'s file name,
/// `pid` this process's id, `n` counting up from 0 past names already taken); it is created exclusively and written
/// through that handle, so nothing already there, a symlink included, is followed or overwritten. With
/// `executable`, on unix the temporary file is set to mode 0o755 through that handle after the copy and before the
/// rename; set explicitly, the mode is not reduced by the umask. On a failure only the temporary file this call
/// created is removed.
pub(crate) fn write_through_temp(dest: &Path, source: &mut dyn Read, executable: bool) -> io::Result<()> {
    let (temp, mut file) = create_temp(dest)?;
    let written = io::copy(source, &mut file).and_then(|_| set_executable(&file, executable));
    drop(file);
    let result = written.and_then(|()| fs::rename(&temp, dest));
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

/// Creates the first free `.<name>.<pid>.<n>.installing` beside `dest`, open for writing.
fn create_temp(dest: &Path) -> io::Result<(PathBuf, File)> {
    let name = dest.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default();
    let pid = std::process::id();
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
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

/// With `executable`, sets `file` to mode 0o755; set_permissions is not reduced by the umask, as a create mode is.
#[cfg(unix)]
fn set_executable(file: &File, executable: bool) -> io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    if executable {
        file.set_permissions(fs::Permissions::from_mode(0o755))
    } else {
        Ok(())
    }
}

/// Off unix there is no mode bit to set.
#[cfg(not(unix))]
fn set_executable(_file: &File, _executable: bool) -> io::Result<()> {
    Ok(())
}

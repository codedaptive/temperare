//! `append_under_home`: adds one line to a file in a folder under HOME, as every file tm writes there is written.
//! Tested through append_savings_log in integration/append_savings_log_tests.rs, its same-file check in
//! integration/same_file_tests.rs, and through the telemetry file in tests/world/telemetry_real.rs.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use super::same_file;

/// Appends `line` and a newline, as one `write_all`, to `<home>/<folders...>/<file_name>`, making missing folders.
/// When any of `folders` (each taken under the one before it, the first under HOME) or `file_name` is a symbolic link
/// (dangling included), or is not a folder or a regular file as expected, when it is looked at, nothing is written.
/// On unix a link at the file is also refused by the open itself, and a file swapped after its look is caught. A
/// folder swapped for a link after its look and before the file is opened is still not caught. HOME itself is not
/// looked at. Does nothing when there is no HOME or it is empty; any failure is silent.
pub(crate) fn append_under_home(home: Option<&Path>, folders: &[&str], file_name: &str, line: &str) {
    let Some(home) = home.filter(|home| !home.as_os_str().is_empty()) else {
        return;
    };
    // These files are a convenience: a check never fails or prints because one could not be written.
    let _ = append(home, folders, file_name, line);
}

fn append(home: &Path, folders: &[&str], file_name: &str, line: &str) -> io::Result<()> {
    let mut folder = home.to_path_buf();
    for name in folders {
        folder.push(name);
        ensure_folder(&folder)?;
    }
    // One write_all of line and newline together, on a file opened in append mode: each write goes at the end, and
    // the line goes in one write unless the OS writes it short, so runs at the same time do not interleave lines in
    // practice.
    open_file(&folder.join(file_name))?.write_all(format!("{line}\n").as_bytes())
}

/// Makes `folder` when nothing is there; refuses a symbolic link or anything that is not a folder. The look at
/// `folder` uses `symlink_metadata`, so a link there is refused. The folders above it are checked one at a time by
/// `append`, from HOME down; HOME itself is not looked at.
fn ensure_folder(folder: &Path) -> io::Result<()> {
    match fs::symlink_metadata(folder) {
        Ok(looked) if looked.is_dir() => Ok(()),
        Ok(_) => Err(refusal(folder)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => fs::create_dir(folder),
        Err(e) => Err(e),
    }
}

/// Opens `file` for appending. When nothing is there it is created exclusively (`create_new`); when a regular file
/// is there it is opened without create. On unix both opens carry `O_NOFOLLOW`, so a symbolic link at `file` fails
/// the open itself, and `O_NONBLOCK`, so a FIFO swapped in cannot block it. The handle is kept only when its own
/// metadata is a regular file and, for a file that was there, when `same_file` finds it is the file looked at. A
/// symbolic link or anything else at `file` is refused.
fn open_file(file: &Path) -> io::Result<File> {
    let (opened_file, looked) = match fs::symlink_metadata(file) {
        Ok(looked) if looked.is_file() => (append_options().open(file)?, Some(looked)),
        Ok(_) => return Err(refusal(file)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => (append_options().create_new(true).open(file)?, None),
        Err(e) => return Err(e),
    };
    let opened = opened_file.metadata()?;
    if opened.is_file() && looked.map_or(true, |looked| same_file(&looked, &opened)) {
        Ok(opened_file)
    } else {
        Err(refusal(file))
    }
}

/// Append mode; on unix also `O_NOFOLLOW | O_NONBLOCK` (see `open_file`).
fn append_options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.append(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::custom_flags(&mut options, libc::O_NOFOLLOW | libc::O_NONBLOCK);
    options
}

/// `<path> is not where tm writes under HOME`: a link or the wrong kind of entry on the file's path.
fn refusal(path: &Path) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{} is not where tm writes under HOME", path.display()))
}

//! `append_savings_log`: adds one line to the savings log under HOME.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

use super::same_file;

/// The folders from HOME down to the log, outermost first.
const FOLDERS: [&str; 3] = [".local", "state", "temperare"];

/// Appends `line` and a newline, in one write, to `<home>/.local/state/temperare/savings.jsonl`, making missing
/// folders. When `.local`, `.local/state`, `.local/state/temperare` or `savings.jsonl` is a symbolic link (dangling
/// included), or is not a folder or a regular file as expected, when it is looked at, nothing is written. On unix a
/// link at `savings.jsonl` is also refused by the open itself, and a log file swapped after its look is caught. A
/// folder swapped for a link after its look and before the log is opened is still not caught. HOME itself is not
/// looked at. Does nothing when there is no HOME; any failure is silent.
pub fn append_savings_log(home: Option<&Path>, line: &str) {
    let Some(home) = home.filter(|home| !home.as_os_str().is_empty()) else {
        return;
    };
    // The log is a convenience: a check never fails or prints because it could not be written.
    let _ = append(home, line);
}

fn append(home: &Path, line: &str) -> io::Result<()> {
    let mut folder = home.to_path_buf();
    for name in FOLDERS {
        folder.push(name);
        ensure_folder(&folder)?;
    }
    // One write_all of line and newline together, on a file opened in append mode.
    open_log(&folder.join("savings.jsonl"))?.write_all(format!("{line}\n").as_bytes())
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

/// Opens the log for appending. When nothing is there it is created exclusively (`create_new`); when a regular file
/// is there it is opened without create. On unix both opens carry `O_NOFOLLOW`, so a symbolic link at `log` fails
/// the open itself, and `O_NONBLOCK`, so a FIFO swapped in cannot block it. The handle is kept only when its own
/// metadata is a regular file and, for a log that was there, when `same_file` finds it is the file looked at. A
/// symbolic link or anything else at `log` is refused.
fn open_log(log: &Path) -> io::Result<File> {
    let (file, looked) = match fs::symlink_metadata(log) {
        Ok(looked) if looked.is_file() => (log_options().open(log)?, Some(looked)),
        Ok(_) => return Err(refusal(log)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => (log_options().create_new(true).open(log)?, None),
        Err(e) => return Err(e),
    };
    let opened = file.metadata()?;
    if opened.is_file() && looked.map_or(true, |looked| same_file(&looked, &opened)) {
        Ok(file)
    } else {
        Err(refusal(log))
    }
}

/// Append mode; on unix also `O_NOFOLLOW | O_NONBLOCK` (see `open_log`).
fn log_options() -> OpenOptions {
    let mut options = OpenOptions::new();
    options.append(true);
    #[cfg(unix)]
    std::os::unix::fs::OpenOptionsExt::custom_flags(&mut options, libc::O_NOFOLLOW | libc::O_NONBLOCK);
    options
}

/// `<path> is not where the savings log is written`: a link or the wrong kind of entry on the log's path.
fn refusal(path: &Path) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{} is not where the savings log is written", path.display()))
}

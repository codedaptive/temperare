//! `open_in_folder`: opens a rule YAML by walking down from "/", following no symlink on the way.
//!
//! Why `unsafe` (S-41): std has no `openat` and no way to refuse a symlink in a middle component, so on Unix this
//! file calls libc's `openat` directly. Each `unsafe` block holds that one call or the one wrap of its result.

use std::fs::File;
use std::path::Path;

/// The open regular file at `path`, which must lie below `folder` (canonical, the trust root). `Err` is the stderr
/// text.
///
/// On Unix, "/" is opened as a directory, then each component of `folder` and then of `path` below it is opened from
/// the handle of the one before, without following a symlink, so the file returned is the one reached from "/" at the
/// moment of each open, whatever the path resolves to before or after. The walk needs read permission on every folder
/// from "/" down to `folder`: a folder on that walk that cannot be opened is a failed read naming that folder. A
/// `folder` that is not absolute, a `path` not below `folder`, or a component of either that is not a plain name, is
/// outside the folder. A symlink met at any component, or a folder on the walk replaced by something that is not a
/// folder, is refused as changed while it was opened; any other failed open (a missing component among them) cannot
/// be read. The last component is opened without blocking, so a FIFO cannot hold the walk, and anything it names
/// that is not a regular file (`path` equal to `folder` included) is refused. Off Unix, `path` is opened as it
/// stands, with no such walk: the caller's path checks are all that hold there.
#[cfg(unix)]
pub(crate) fn open_in_folder(folder: &Path, path: &Path, check: &str) -> Result<File, String> {
    use std::os::fd::AsRawFd;
    let parts = parts_below(folder, path, check)?;
    let mut handle = open_folder(folder, check)?;
    let file = match parts.split_last() {
        None => File::from(handle),
        Some((last, dirs)) => {
            for dir in dirs {
                handle = open_at(handle.as_raw_fd(), dir, DIR_FLAGS).map_err(|err| refusal(err, true, check))?;
            }
            let flags = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC;
            File::from(open_at(handle.as_raw_fd(), last, flags).map_err(|err| refusal(err, false, check))?)
        }
    };
    match file.metadata() {
        Ok(meta) if meta.is_file() => Ok(file),
        Ok(_) => Err(format!("ast-grep rule {check} is not a regular file")),
        Err(err) => Err(format!("cannot read ast-grep rule {check}: {err}")),
    }
}

/// Off Unix: `path` opened as it stands; `rule_yaml_path` checked it before.
#[cfg(not(unix))]
pub(crate) fn open_in_folder(_folder: &Path, path: &Path, check: &str) -> Result<File, String> {
    File::open(path).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))
}

/// The flags every folder below "/" is opened with: a directory, never through a symlink.
#[cfg(unix)]
const DIR_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC;

/// The outside-the-folder refusal.
#[cfg(unix)]
fn outside(check: &str) -> String {
    format!("ast-grep rule {check} is outside the rule file's folder")
}

/// The plain-name components of `path` below `folder`, or the outside-the-folder refusal.
#[cfg(unix)]
fn parts_below<'a>(folder: &Path, path: &'a Path, check: &str) -> Result<Vec<&'a std::ffi::OsStr>, String> {
    use std::path::Component;
    path.strip_prefix(folder)
        .map_err(|_| outside(check))?
        .components()
        .map(|part| match part {
            Component::Normal(name) => Ok(name),
            _ => Err(outside(check)),
        })
        .collect()
}

/// A handle on `folder`, reached by opening "/" and then each plain-name component of `folder` from the handle of
/// the one before, following no symlink. A symlink or a non-folder met is refused as a swap, as in `refusal`; any
/// other failed open names the folder it was opening.
#[cfg(unix)]
fn open_folder(folder: &Path, check: &str) -> Result<std::os::fd::OwnedFd, String> {
    use std::os::fd::AsRawFd;
    use std::path::Component;
    let mut parts = folder.components();
    if parts.next() != Some(Component::RootDir) {
        return Err(outside(check));
    }
    let mut dir = Path::new("/").to_path_buf();
    let mut handle = open_at(libc::AT_FDCWD, dir.as_os_str(), libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)
        .map_err(|err| folder_refusal(err, &dir, check))?;
    for part in parts {
        let Component::Normal(name) = part else {
            return Err(outside(check));
        };
        dir.push(name);
        handle = open_at(handle.as_raw_fd(), name, DIR_FLAGS).map_err(|err| folder_refusal(err, &dir, check))?;
    }
    Ok(handle)
}

/// The stderr text for a failed open of the folder `dir` on the walk from "/": a swap as in `refusal`, any other
/// error a failed read naming `dir`.
#[cfg(unix)]
fn folder_refusal(err: std::io::Error, dir: &Path, check: &str) -> String {
    match err.raw_os_error() {
        Some(libc::ELOOP | libc::ENOTDIR) => refusal(err, true, check),
        _ => format!("cannot read ast-grep rule {check}: cannot open folder {}: {err}", dir.display()),
    }
}

/// The stderr text for a failed open of one component. ELOOP is a symlink where a plain entry was opened, and
/// ENOTDIR on a middle component is a folder replaced by something else: both mean the tree changed under the walk.
/// Any other error, a missing component included, is a failed read.
#[cfg(unix)]
fn refusal(err: std::io::Error, middle: bool, check: &str) -> String {
    match err.raw_os_error() {
        Some(libc::ELOOP) => format!("ast-grep rule {check} changed while it was opened"),
        Some(libc::ENOTDIR) if middle => format!("ast-grep rule {check} changed while it was opened"),
        _ => format!("cannot read ast-grep rule {check}: {err}"),
    }
}

/// `openat(dir, name, flags)` as an owned handle. A `name` holding a NUL byte cannot be opened and is an error.
#[cfg(unix)]
fn open_at(
    dir: std::os::fd::RawFd,
    name: &std::ffi::OsStr,
    flags: libc::c_int,
) -> std::io::Result<std::os::fd::OwnedFd> {
    use std::os::fd::FromRawFd;
    use std::os::unix::ffi::OsStrExt;
    let name = std::ffi::CString::new(name.as_bytes())?;
    // SAFETY: `name` is a NUL-terminated string that lives past the call, `dir` is AT_FDCWD or an open descriptor
    // the caller holds for the whole call, and no flag here creates a file, so openat reads no mode argument.
    let fd = unsafe { libc::openat(dir, name.as_ptr(), flags) };
    if fd < 0 {
        return Err(std::io::Error::last_os_error());
    }
    // SAFETY: `fd` was just returned open by openat and nothing else owns it, so the OwnedFd is its only owner.
    Ok(unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) })
}

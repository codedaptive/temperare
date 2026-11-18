//! `open_in_folder`: opens a rule YAML by walking down from its folder's handle, following no symlink on the way.
//!
//! Why `unsafe` (S-41): std has no `openat` and no way to refuse a symlink in a middle component, so on Unix this
//! file calls libc's `openat` directly. Each `unsafe` block holds that one call or the one wrap of its result.

use std::fs::File;
use std::path::Path;

/// The open file at `path`, which must lie below `folder` (canonical, the trust root). `Err` is the stderr text.
///
/// On Unix, `folder` is opened as a directory and each component of `path` below it is opened from the handle of
/// the one before, without following a symlink, so the file returned is the one reached inside `folder` at the
/// moment of each open, whatever the path resolves to before or after. A `path` not below `folder`, or with a
/// component that is not a plain name, is outside the folder. A symlink met at any component, or a middle component
/// replaced by something that is not a folder, is refused as changed while it was opened; any other failed open (a
/// missing component among them) cannot be read. Off Unix, `path` is opened as it stands, with no such walk: the
/// caller's path checks are all that hold there.
#[cfg(unix)]
pub(crate) fn open_in_folder(folder: &Path, path: &Path, check: &str) -> Result<File, String> {
    use std::os::fd::AsRawFd;
    let parts = parts_below(folder, path, check)?;
    let mut handle = open_at(libc::AT_FDCWD, folder.as_os_str(), libc::O_RDONLY | libc::O_DIRECTORY | libc::O_CLOEXEC)
        .map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    let Some((last, dirs)) = parts.split_last() else {
        return Ok(File::from(handle));
    };
    let dir_flags = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC;
    for dir in dirs {
        handle = open_at(handle.as_raw_fd(), dir, dir_flags).map_err(|err| refusal(err, true, check))?;
    }
    open_at(handle.as_raw_fd(), last, libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC)
        .map(File::from)
        .map_err(|err| refusal(err, false, check))
}

/// Off Unix: `path` opened as it stands; `rule_yaml_path` checked it before.
#[cfg(not(unix))]
pub(crate) fn open_in_folder(_folder: &Path, path: &Path, check: &str) -> Result<File, String> {
    File::open(path).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))
}

/// The plain-name components of `path` below `folder`, or the outside-the-folder refusal.
#[cfg(unix)]
fn parts_below<'a>(folder: &Path, path: &'a Path, check: &str) -> Result<Vec<&'a std::ffi::OsStr>, String> {
    use std::path::Component;
    let outside = || format!("ast-grep rule {check} is outside the rule file's folder");
    path.strip_prefix(folder)
        .map_err(|_| outside())?
        .components()
        .map(|part| match part {
            Component::Normal(name) => Ok(name),
            _ => Err(outside()),
        })
        .collect()
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
fn open_at(dir: std::os::fd::RawFd, name: &std::ffi::OsStr, flags: libc::c_int) -> std::io::Result<std::os::fd::OwnedFd> {
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

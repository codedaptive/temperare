//! `shell_files_with`: the shell scripts git tracks under a rule's folder, reading an extensionless file's first
//! bytes through a given reader.

use super::is_shell_shebang;
use crate::host::Host;
use crate::ran::ran;
use crate::types::Stop;
use procio::Cmd;
use std::path::{Path, PathBuf};

/// The extensions of the scripts shellcheck checks.
const SHELL_EXTENSIONS: [&str; 4] = ["sh", "bash", "ksh", "dash"];

/// The scripts git tracks under `folder`, relative to it, in git's order: `git ls-files -z -t --cached --deleted` run
/// in `folder` through `ran` (a listing, not the check run, so it is not counted). Kept: the paths tagged `H`
/// (tracked) minus every path tagged `R` (tracked but deleted from disk, which shellcheck would fail to read), whose
/// extension is `sh`, `bash`, `ksh` or `dash`, or which have no extension and whose first bytes, as `head` reads them
/// from `folder` joined with the path, are a shell shebang (`is_shell_shebang`). A path `head` cannot read (`None`)
/// is left out, never `Failed`; `head` is asked only for the extensionless paths. On unix each path keeps git's
/// bytes, so a name that is not UTF-8 reaches shellcheck exactly; elsewhere a name that is not UTF-8 is left out, never
/// `Failed`. An exit other than 0 is `Failed` with git's exit code and its stderr.
pub(crate) fn shell_files_with(
    folder: &Path,
    host: &Host,
    head: &dyn Fn(&Path) -> Option<Vec<u8>>,
) -> Result<Vec<PathBuf>, Stop> {
    let args = ["ls-files", "-z", "-t", "--cached", "--deleted"];
    let cmd = Cmd {
        args: args.into_iter().map(Into::into).collect(),
        cwd: Some(folder.to_path_buf()),
        ..Cmd::new("git")
    };
    let out = ran(&cmd, host)?;
    if out.code != Some(0) {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let stderr = if stderr.is_empty() { "git ls-files printed nothing on stderr".to_string() } else { stderr.into_owned() };
        return Err(Stop::Failed { code: out.code, stderr });
    }
    let entries: Vec<&[u8]> = out.stdout.split(|&b| b == 0).filter(|entry| !entry.is_empty()).collect();
    let deleted: Vec<&[u8]> = entries.iter().filter_map(|entry| entry.strip_prefix(b"R ")).collect();
    Ok(entries
        .iter()
        .filter_map(|entry| entry.strip_prefix(b"H "))
        .filter(|path| !deleted.contains(path))
        .filter_map(path_of)
        .filter(|path| match path.extension() {
            Some(ext) => SHELL_EXTENSIONS.iter().any(|shell| ext == *shell),
            None => head(&folder.join(path)).is_some_and(|bytes| is_shell_shebang(&bytes)),
        })
        .collect())
}

/// The path git printed as `bytes`: on unix those exact bytes.
#[cfg(unix)]
fn path_of(bytes: &[u8]) -> Option<PathBuf> {
    use std::os::unix::ffi::OsStrExt;
    Some(PathBuf::from(std::ffi::OsStr::from_bytes(bytes)))
}

/// The path git printed as `bytes`: off unix, `None` when they are not UTF-8, since a path there is not raw bytes.
#[cfg(not(unix))]
fn path_of(bytes: &[u8]) -> Option<PathBuf> {
    std::str::from_utf8(bytes).ok().map(PathBuf::from)
}

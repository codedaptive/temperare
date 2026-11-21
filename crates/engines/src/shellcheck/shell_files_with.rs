//! `shellcheck::shell_files_with`: the shell scripts under a rule's folder among a listing of the repository's files,
//! asking given probes whether each is a regular file and what its first bytes are.

use super::is_shell_shebang;
use std::path::{Path, PathBuf};

/// The extensions of the scripts shellcheck checks.
const SHELL_EXTENSIONS: [&str; 4] = ["sh", "bash", "ksh", "dash"];

/// The scripts among `listed` (paths relative to `top`, the repository's top) that sit under `folder`, in `listed`'s
/// order and relative to `folder`. A path is kept when `regular` says it is a regular file (`shell_files` gives a
/// probe that says no to any path reached through a symlink), and its extension is `sh`, `bash`, `ksh` or `dash`, or it has no extension and the
/// first bytes `head` reads are a shell shebang (`is_shell_shebang`). A path `head` cannot read (`None`) is left out.
/// Both probes are given `top` joined with the path; a path outside `folder` is asked nothing, and `head` is asked
/// only for the extensionless regular files.
pub(crate) fn shell_files_with(
    folder: &Path,
    top: &Path,
    listed: &[PathBuf],
    regular: &dyn Fn(&Path) -> bool,
    head: &dyn Fn(&Path) -> Option<Vec<u8>>,
) -> Vec<PathBuf> {
    listed
        .iter()
        .filter_map(|path| {
            let full = top.join(path);
            let relative = full.strip_prefix(folder).ok()?.to_path_buf();
            let script = regular(&full)
                && match relative.extension() {
                    Some(ext) => SHELL_EXTENSIONS.iter().any(|shell| ext == *shell),
                    None => head(&full).is_some_and(|bytes| is_shell_shebang(&bytes)),
                };
            script.then_some(relative)
        })
        .collect()
}

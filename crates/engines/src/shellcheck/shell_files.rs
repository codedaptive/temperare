//! `shell_files`: the shell scripts under a rule's folder that shellcheck is given.

use super::{read_head, shell_files_with};
use crate::host::Host;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// The scripts git tracks under `folder`, relative to it, as `shell_files_with` lists them: by extension, and a file
/// with no extension when its first line, read from disk by `read_head`, is a shell shebang. Off unix `read_head`
/// reads nothing, so a file with no extension is left out there.
pub(crate) fn shell_files(folder: &Path, host: &Host) -> Result<Vec<PathBuf>, Stop> {
    shell_files_with(folder, host, &read_head)
}

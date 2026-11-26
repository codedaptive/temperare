//! `strip_verbatim_disk`: a verbatim disk path (`\\?\C:\...`) in the plain disk form `windows_path` gives.

use std::path::{Path, PathBuf};

/// `path` without its leading `\\?\` when its text starts `\\?\X:\` (X an ASCII letter); any other path (UNC
/// `\\?\UNC\...`, a bare `\\?\X:`, no prefix, text that is not UTF-8) as given. Pure text work. Why: the canonical
/// rule folder on Windows is verbatim (`\\?\C:\...`) while `windows_path` gives `C:\...`, and `rel_to_folder`
/// compares them as `Path` prefixes, which a Disk and a VerbatimDisk prefix never match. Limit: the drive letter is
/// compared as std's Windows `Path` compares it, so an engine printing `c:` against a `C:` folder is kept or dropped by
/// that comparison; the gammaAI run shows which.
pub(super) fn strip_verbatim_disk(path: &Path) -> PathBuf {
    match path.to_str().and_then(|text| text.strip_prefix(r"\\?\")) {
        Some(rest) if matches!(rest.as_bytes(), [letter, b':', b'\\', ..] if letter.is_ascii_alphabetic()) => {
            rest.into()
        }
        _ => path.into(),
    }
}

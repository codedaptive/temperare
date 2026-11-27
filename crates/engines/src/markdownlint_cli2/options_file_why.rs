//! `options_file_why`: why a markdownlint-cli2 options file stops a rule, if one is where markdownlint-cli2 reads it.

use std::path::{Path, PathBuf};

/// The options files markdownlint-cli2 0.24.2 reads in each folder it visits, in its order.
const OPTIONS_NAMES: [&str; 4] =
    [".markdownlint-cli2.jsonc", ".markdownlint-cli2.yaml", ".markdownlint-cli2.cjs", ".markdownlint-cli2.mjs"];

/// The why for the first options file `is_file` finds (given the absolute path) in `folder`, then in each folder from
/// `folder` down to each of `files` (relative to `folder`), in `files`' order; None when there is none. Those are the
/// folders markdownlint-cli2 reads options in when run in `folder` on `files` (measured: never above its working
/// folder, never in HOME). The files are found by their fixed names, not by git, since markdownlint-cli2 reads an
/// untracked or ignored one too. An options file overrides cctao's own options module key by key (measured: its
/// `fix: true` rewrote a checked file), so any one stops the rule; the why names it relative to `folder`.
pub(crate) fn options_file_why(folder: &Path, files: &[PathBuf], is_file: &dyn Fn(&Path) -> bool) -> Option<String> {
    let below = files.iter().flat_map(|file| {
        let mut folders: Vec<&Path> = file.ancestors().skip(1).filter(|dir| !dir.as_os_str().is_empty()).collect();
        folders.reverse();
        folders
    });
    let found = std::iter::once(Path::new(""))
        .chain(below)
        .flat_map(|dir| OPTIONS_NAMES.iter().map(move |name| dir.join(name)))
        .find(|path| is_file(&folder.join(path)))?;
    Some(format!(
        "markdownlint-cli2 options file {} would override cctao's run (fixing files, ignoring files, inline config, \
         output): remove it; exceptions go in the rule file",
        found.display()
    ))
}

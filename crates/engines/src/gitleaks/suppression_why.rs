//! `suppression_why`: why gitleaks would silence findings outside the rule file in a rule's folder, if it would. Reads
//! the disk, so it has no unit test: tests/gitleaks.rs (ignore file, config allowlist, extended allowlist, inline
//! extended allowlist, config own rule, mixed-case allowlist, config not TOML, dotted allowlist key) tests it.

use super::{config_why, ignore_listed};
use std::collections::VecDeque;
use std::path::Path;

/// The most config files read: the config and the files it extends. gitleaks 8.30.1 errors on a config whose extend
/// file extends again, so three files reads past any chain it accepts.
const FILES: usize = 3;

const EXCEPTION: &str = "the rule file is the only home for an exception, so cctao reads no findings under it";

/// Why `folder` (gitleaks' working folder) carries an exception gitleaks would apply, or None:
/// 1. a queue of config files: first the one gitleaks reads, `config` (joined to `folder`; an absolute path is kept)
///    or else `.gitleaks.toml`, then every extend path `config_why` gives for each file read, joined the same way
///    (gitleaks resolves an extend path from its working folder). Each is read once, in queue order, to `FILES` files
///    read: the first Err of `config_why` (the file does not parse, or declares an allowlist) is the why. A path that
///    names no file cctao can read is skipped: gitleaks reports a missing extend file itself;
/// 2. else a `.gitleaksignore` in `folder` that `ignore_listed`: "`.gitleaksignore` in the rule's folder lists
///    findings to ignore; ...". gitleaks reads that file whatever `--gitleaks-ignore-path` says, so cctao cannot switch
///    it off and fails the rule instead.
pub(crate) fn suppression_why(folder: &Path, config: Option<&str>) -> Option<String> {
    let mut queue = VecDeque::from([config.unwrap_or(".gitleaks.toml").to_string()]);
    let mut read = Vec::new();
    while read.len() < FILES {
        let Some(name) = queue.pop_front() else { break };
        let path = folder.join(&name);
        if read.contains(&path) {
            continue;
        }
        let Some(text) = text_of(&path) else { continue };
        read.push(path);
        match config_why(&name, &text) {
            Ok(extends) => queue.extend(extends),
            Err(why) => return Some(why),
        }
    }
    text_of(&folder.join(".gitleaksignore"))
        .filter(|text| ignore_listed(text))
        .map(|_| format!("`.gitleaksignore` in the rule's folder lists findings to ignore; {EXCEPTION}"))
}

/// The text of `path` when it is a file cctao can read, invalid UTF-8 replaced; None otherwise. Only a regular file is
/// read, so a path naming a folder or a pipe never blocks.
fn text_of(path: &Path) -> Option<String> {
    path.is_file().then(|| std::fs::read(path).ok()).flatten().map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
}

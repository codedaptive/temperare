//! `mypy_config`: the config file mypy itself would read when run in a folder, found through a read probe.

use std::path::{Path, PathBuf};
use toml::{Table, Value};

/// mypy's config file names in the order it tries them in one directory, each with what makes the file count.
const CANDIDATES: [(&str, fn(&str) -> bool); 4] =
    [("mypy.ini", any), (".mypy.ini", any), ("pyproject.toml", has_tool_mypy), ("setup.cfg", has_mypy_section)];

/// The config file mypy 2.4.1 would read when run in `folder`, found through `read` (a file's contents; None when it
/// is absent or unreadable). Each directory from `folder` up to and including `top` (the repository's top; `folder`
/// alone when it is not under `top`) is searched in mypy's order:
/// - `mypy.ini`, then `.mypy.ini`: each counts whenever `read` gives its contents;
/// - `pyproject.toml`: counts only when it parses as TOML and has a `tool.mypy` table;
/// - `setup.cfg`: counts only when some line, trimmed, is exactly `[mypy]`.
///
/// The first that counts, as its full path; None when none does. The user's home config is never looked at: cctao pins
/// the config it finds, so mypy never reads one outside the repository.
pub(crate) fn mypy_config(folder: &Path, top: &Path, read: &dyn Fn(&Path) -> Option<String>) -> Option<PathBuf> {
    let dirs: Vec<&Path> = if folder.starts_with(top) {
        folder.ancestors().take_while(|dir| dir.starts_with(top)).collect()
    } else {
        vec![folder]
    };
    dirs.into_iter().find_map(|dir| {
        CANDIDATES
            .into_iter()
            .map(|(name, counts)| (dir.join(name), counts))
            .find(|(path, counts)| read(path).is_some_and(|text| counts(&text)))
            .map(|(path, _)| path)
    })
}

/// Any contents count.
fn any(_: &str) -> bool {
    true
}

/// Whether `text` parses as TOML with a `tool.mypy` table.
fn has_tool_mypy(text: &str) -> bool {
    text.parse::<Table>()
        .is_ok_and(|table| table.get("tool").and_then(|tool| tool.get("mypy")).is_some_and(Value::is_table))
}

/// Whether some line of `text`, trimmed, is exactly `[mypy]`.
fn has_mypy_section(text: &str) -> bool {
    text.lines().any(|line| line.trim() == "[mypy]")
}

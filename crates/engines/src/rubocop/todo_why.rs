//! `todo_why`: why a rule is refused when the repository's rubocop todo file names one of its cops.

use std::ffi::OsStr;
use std::path::Path;

/// The why for the first `.rubocop_todo.yml` that `show` (a `rubocop --show-cops --debug` dump) says the config
/// reads, on its `configuration from <path>` and `Inheriting configuration from <path>` lines, whose text (`read`)
/// has a line containing one of `check`'s comma-separated cops. The why names the file, the 1-based line and the cop.
/// `None` when no such file names a cop of `check`, and for a todo file `read` gives `None` for. rubocop's todo file
/// is a list of exceptions (per-cop `Exclude`s, raised `Max` limits) that neither `--only` nor
/// `--ignore-disable-comments` undoes, and exceptions belong in the rule file. A cop name the file holds only in a
/// comment matches too: that makes the rule `Failed`, never clean.
pub(crate) fn todo_why(show: &str, check: &str, read: &dyn Fn(&Path) -> Option<String>) -> Option<String> {
    show.lines()
        .filter_map(|line| Some(Path::new(line.split_once("configuration from ")?.1)))
        .filter(|path| path.file_name() == Some(OsStr::new(".rubocop_todo.yml")))
        .find_map(|path| {
            let text = read(path)?;
            let (at, cop) = text
                .lines()
                .enumerate()
                .find_map(|(at, line)| Some((at, check.split(',').find(|cop| line.contains(cop))?)))?;
            Some(format!(
                "rubocop's todo file {} names {cop} on line {}: a todo file is a list of exceptions, and exceptions go in the rule file",
                path.display(),
                at + 1
            ))
        })
}

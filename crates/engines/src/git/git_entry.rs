//! `GIT`: git as an entry, counting the lines a change adds.

use super::{git_refusal, read_test_ratio, run_git};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use crate::types::ParseError;
use rules::Engine;

/// git run by its own flow, `run_git`, for the one check `test_ratio` (`git_refusal` refuses any other before
/// anything runs). It counts the lines the change adds against the check's base (`Host::base`): procio's
/// `added_lines` for tracked files and every line of each untracked file git does not ignore, each file in the
/// rule's scope counted as test when `test_paths` matches it and as code otherwise. A change that adds at least
/// `floor` lines of test and more than `max_ratio` lines of test per line of code gets one finding, at the first
/// added line of the test file with the most added lines. Nothing is parsed.
///
/// The params, read by `read_test_ratio` and refused before anything runs when missing, out of range or unknown:
/// `max_ratio` (a number from 0.1 to 100), `floor` (a whole number from 0 to 100000) and `test_paths` (a non-empty
/// list of globs). With no base the rule is not available with `ONLY_WITH_BASE_WHY`, never clean. A git rule runs
/// alone (`shares_with`). No check run is counted, so its `raw_chars` is `None`: the counts come from git's own
/// listings, which are not output a reader would otherwise read, and counting the diff text would inflate the
/// savings. The flow never reads engine output, so `read` only says so; `entry_cmd` and `args`, `after` and
/// `config` are not used. git never says it does not know a check, so `unknown` gives `None`. The version is the
/// word after "git version" in `git --version`.
pub static GIT: Entry = Entry {
    name: "git",
    engine: Engine::Git,
    program: "git",
    needs: &["git"],
    never: None,
    args: &[],
    pick: Pick::None,
    params: Params::Flow { names: &["max_ratio", "floor", "test_paths"], check: |p| read_test_ratio(p).map(drop) },
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: git_refusal,
    unknown: |_, _| None,
    read: |_, _, _, _| Err(ParseError { why: "git output is never read: the git entry's flow counts the change".to_string() }),
    run: run_git,
};

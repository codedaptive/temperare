//! `union_args`: the argument(s) that pick several checks in one run of an engine.

use crate::entry::{Entry, Pick};
use rules::Engine;
use std::ffi::OsString;

/// The pick argument(s) of one run of `entry` over `checks`, each check once (a repeat after its first is dropped),
/// in the order given. `Pick::Arg(flag)`: for ruff one argument, `{flag}` then the checks joined by ",", the comma
/// list a ruff check already is (`E711,F401`), so the run is one a single rule could have asked for; for any other
/// engine one argument `{flag}{check}` per check (clippy's `--warn=` and swiftlint's `--only-rule=` each take one
/// name and can be given again). `Pick::File` and `Pick::None`: none, as such a run is not rewritten (ast-grep shares only runs of
/// one rule YAML).
pub(crate) fn union_args(entry: &Entry, checks: &[&str]) -> Vec<OsString> {
    let mut distinct: Vec<&str> = Vec::new();
    for check in checks {
        if !distinct.contains(check) {
            distinct.push(check);
        }
    }
    match entry.pick {
        Pick::Arg(flag) if entry.engine == Engine::Ruff => vec![OsString::from(format!("{flag}{}", distinct.join(",")))],
        Pick::Arg(flag) => distinct.iter().map(|check| OsString::from(format!("{flag}{check}"))).collect(),
        Pick::File(_) | Pick::None => Vec::new(),
    }
}

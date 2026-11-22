//! `c_listed`: clang-tidy's no-file outcome for a rule, its folder's C-family sources listed through a given lister.

use crate::no_script::no_script;
use crate::types::Stop;
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// For an `Engine::ClangTidy` rule with a resolved `folder`: the sources `sources` lists for it, and `no_script`'s
/// `NoFiles` when none is one `rule` applies to; `None` when some is, or when the listing fails (the flow lists again
/// and stops with its error). Any other engine, or no `folder`, gives `None` without calling `sources`.
pub(crate) fn c_listed(
    rule: &Rule,
    folder: Option<&Path>,
    sources: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Option<Stop> {
    let (Engine::ClangTidy, Some(folder)) = (rule.engine, folder) else {
        return None;
    };
    sources(folder).ok().and_then(|files| no_script(rule, &files))
}

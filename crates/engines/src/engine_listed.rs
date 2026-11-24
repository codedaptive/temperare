//! `engine_listed`: an engine's no-file outcome for a rule, its folder's files listed through a given lister.

use crate::no_script::no_script;
use crate::types::Stop;
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// For a rule of `engine` with a resolved `folder`: the files `sources` lists for it, and `no_script`'s `NoFiles` when
/// none is one `rule` applies to; `None` when some is, or when the listing fails (the flow of every engine served here
/// but dart lists again and stops with its error; dart's lists no sources, and its own `repo_top` call stops when git
/// cannot give the top). Any other engine, or no `folder`, gives `None` without calling `sources`. The one copy of
/// this judgement, shared by `engine_hits_with` (its step 1) and `no_file_rules_listed`.
pub(crate) fn engine_listed(
    engine: Engine,
    rule: &Rule,
    folder: Option<&Path>,
    sources: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Option<Stop> {
    let Some(folder) = folder.filter(|_| rule.engine == engine) else {
        return None;
    };
    sources(folder).ok().and_then(|files| no_script(rule, &files))
}

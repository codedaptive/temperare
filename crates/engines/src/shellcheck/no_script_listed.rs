//! `no_script_listed`: ShellCheck's no-file outcome for a rule, its folder's scripts listed through a given lister.

use crate::no_script::no_script;
use crate::types::Stop;
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// For an `Engine::ShellCheck` rule with a resolved `folder`: the scripts `scripts` lists for it, and `no_script`'s
/// `NoFiles` when none is one `rule` applies to; `None` when some is, or when the listing fails. Any other engine, or
/// no `folder`, gives `None` without calling `scripts`. The one copy of this judgement, shared by `engine_hits_with`
/// (its step 1) and the crate's no-file listing.
pub(crate) fn no_script_listed(
    rule: &Rule,
    folder: Option<&Path>,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Option<Stop> {
    let (Engine::ShellCheck, Some(folder)) = (rule.engine, folder) else {
        return None;
    };
    scripts(folder).ok().and_then(|listed| no_script(rule, &listed))
}

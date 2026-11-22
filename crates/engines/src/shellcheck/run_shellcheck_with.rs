//! `run_shellcheck_with`: shellcheck's flow for one rule, against a given list of shellcheck codes.

use super::{parse_shellcheck, read_code_list, shellcheck_cmd};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::no_script::no_script;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical), `codes` being the text of temperare's list of shellcheck codes and
/// `list` the lister of the scripts under a folder. The central `refused` has already settled the rule's refusals and
/// params (ShellCheck takes none). Each step can stop the rule:
/// 1. a list `read_code_list` cannot read is `Failed`;
/// 2. the scripts `list` gives for `folder`, or its `Stop`; when `no_script` finds none of them the rule applies to,
///    nothing runs and the rule is its `NoFiles`, whether its code is listed or not. `engine_hits_with` settles this
///    before availability too, but its own listing can error and fall through to this flow, so this check is then the
///    only guard against running shellcheck with no script;
/// 3. a check not in the list is `NotListed`, its why naming the check and the list's version; `stopped` adds the
///    engine and the version `engine_versions` read before the rules ran. Nothing runs for it, so a code not listed is
///    never clean. Otherwise the check run is given every listed script;
/// 4. the check run of `shellcheck_cmd` on those scripts with the rule's config, in `folder`, through `check_run` so
///    it is counted, read by `parse_shellcheck`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_shellcheck_with(
    codes: &str,
    list: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let code_list = read_code_list(codes).map_err(|e| Stop::Failed {
        code: None,
        stderr: format!("temperare's list of {} codes cannot be read: {e}", entry.name),
    })?;
    let files = list(folder)?;
    // The only guard against running ShellCheck with no files when `engine_hits_with`'s first listing errored and fell
    // through to this flow.
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    if !code_list.codes.contains(&rule.check.as_str()) {
        let (check, name, version) = (&rule.check, entry.name, code_list.version);
        let said = format!("{check} is not in temperare's list of {name} codes (taken from {name} {version})");
        return Err(Stop::NotListed { said });
    }
    let out = check_run(&shellcheck_cmd(rule.config.as_deref(), &files, folder), host)?;
    parse_shellcheck(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

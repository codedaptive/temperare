//! `run_shellcheck_with`: shellcheck's flow for one rule, against a given list of shellcheck codes.

use super::{not_in_list_why, parse_shellcheck, read_code_list, shellcheck_cmd, shellcheck_refusal};
use crate::check_run::check_run;
use crate::engine_version::engine_version;
use crate::entry::{param_args, param_why, Entry};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical), `codes` being the text of temperare's list of shellcheck codes and
/// `list` the lister of the scripts under a folder; each step can stop the rule:
/// 1. a check `shellcheck_refusal` refuses is `Failed` with no exit code and the refusal; nothing runs;
/// 2. params `param_args` cannot use are `Failed` with `param_why`'s text, and params it can use are `Failed` too,
///    since shellcheck's run has no place for them and a param is never dropped; nothing runs;
/// 3. a list `read_code_list` cannot read is `Failed`;
/// 4. a check not in the list is `NotAvailable` with `not_in_list_why`, the installed version read by the entry's
///    probe (not counted). It comes before the files are listed, so a code not listed is never clean;
/// 5. the scripts `list` gives for `folder`, or its `Stop`; with none, nothing is checked and the rule has no hits;
/// 6. the check run of `shellcheck_cmd` on those scripts with the rule's config, in `folder`, through `check_run` so
///    it is counted, read by `parse_shellcheck`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_shellcheck_with(
    codes: &str,
    list: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    if let Some(why) = shellcheck_refusal(&rule.check) {
        return Err(failed(why));
    }
    let args = param_args(entry, &rule.params).map_err(|err| failed(param_why(entry.name, err)))?;
    if !args.is_empty() {
        return Err(failed(format!("{} passes no params to its run", entry.name)));
    }
    let code_list = read_code_list(codes)
        .map_err(|e| failed(format!("temperare's list of {} codes cannot be read: {e}", entry.name)))?;
    if !code_list.codes.contains(&rule.check.as_str()) {
        let version = engine_version(entry, folder, host);
        return Err(Stop::NotAvailable { why: not_in_list_why(entry.name, version.as_deref(), &rule.check, &code_list) });
    }
    let files = list(folder)?;
    if files.is_empty() {
        return Ok(Vec::new());
    }
    let out = check_run(&shellcheck_cmd(rule.config.as_deref(), &files, folder), host)?;
    parse_shellcheck(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}

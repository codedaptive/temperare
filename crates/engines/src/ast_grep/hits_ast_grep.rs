//! `hits_ast_grep`: one ast-grep rule, its params filled, run over its rule file's folder and read into hits.

use super::{fill_params, read_rule_yaml};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry, TempFile};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use crate::ParamError;
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical); `entry` is `AST_GREP`. The check must name a file inside `folder`,
/// read by `read_rule_yaml`: an absolute check, one that cannot be canonicalized or opened, one whose canonical path is
/// outside `folder` (through `..` or a symlink), and on Unix one whose walk from "/" (`open_in_folder`) met a
/// symlink, found a folder replaced by something else or could not open a folder, one that is not a regular file,
/// one unlinked while it was opened or with more than one hard link (`rule_yaml_text`) are
/// `Failed` with no exit code, and nothing is run. The rule YAML's `{{name}}`s are filled from
/// `rule.params`, and the filled copy written to a temporary file that the entry's command (`entry_cmd`, the copy's
/// path as the pick) runs in `folder` through `host`; the copy is removed before this returns, whatever the outcome.
/// A YAML that cannot be read, a param it names that the rule does not give or cannot fill, a param the rule gives
/// that it never names, and a copy that cannot be written are `Failed` with no exit code; ast-grep absent is
/// `NotAvailable`; output the entry's reader cannot read
/// as ast-grep's JSON lines is `Failed` with its exit code. The ast-grep run is counted (`check_run`).
pub(crate) fn hits_ast_grep(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let check = &rule.check;
    let text = read_rule_yaml(folder, check).map_err(failed)?;
    let filled = fill_params(&text, &rule.params).map_err(|err| failed(param_why(err, check)))?;
    let temp = TempFile::write("ast-grep", "yml", &filled)
        .map_err(|err| failed(format!("cannot write the filled ast-grep rule {check}: {err}")))?;
    let out = check_run(&entry_cmd(entry, temp.path.as_os_str(), &[], None, folder), host)?;
    drop(temp);
    (entry.read)(&out, check, folder, folder).map_err(|err| parse_failed(&out, err))
}

/// The stderr text for a param that could not fill the rule YAML `check`.
fn param_why(err: ParamError, check: &str) -> String {
    match err {
        ParamError::Unknown { name } => format!("unknown param {{{{{name}}}}} in {check}"),
        ParamError::Bad { name, why } => format!("param {name} in {check}: {why}"),
    }
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}

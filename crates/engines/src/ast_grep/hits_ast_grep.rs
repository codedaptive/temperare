//! `hits_ast_grep`: one ast-grep rule, its params filled, run over its rule file's folder and read into hits.

use super::{ast_grep_cmd, fill_params, parse_ast_grep, TempRule};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use crate::ParamError;
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical). The rule YAML `folder/<check>` is read, its `{{name}}`s filled
/// from `rule.params`, and the filled copy written to a temporary file that ast-grep runs through `host`; the copy
/// is removed before this returns, whatever the outcome. A YAML that cannot be read, a param it names that the rule
/// does not give or cannot fill, and a copy that cannot be written are `Failed` with no exit code; ast-grep absent
/// is `NotAvailable`; output that is not ast-grep's JSON lines is `Failed` with its exit code.
pub(crate) fn hits_ast_grep(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let check = &rule.check;
    let text = std::fs::read_to_string(folder.join(check))
        .map_err(|err| failed(format!("cannot read ast-grep rule {check}: {err}")))?;
    let filled = fill_params(&text, &rule.params).map_err(|err| failed(param_why(err, check)))?;
    let temp = TempRule::write(&filled)
        .map_err(|err| failed(format!("cannot write the filled ast-grep rule {check}: {err}")))?;
    let out = ran(&ast_grep_cmd(&temp.path, folder), host)?;
    drop(temp);
    parse_ast_grep(&out, folder, folder).map_err(|err| parse_failed(&out, err))
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

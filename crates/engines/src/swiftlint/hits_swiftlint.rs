//! `hits_swiftlint`: one swiftlint rule run over its rule file's folder, read into hits.

use super::{parse_swiftlint, swiftlint_cmd, unknown_rule};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical): swiftlint runs there through `host` with the rule's check and,
/// when the rule gives one, its config. swiftlint absent is `NotAvailable`; a run that could not start, or output
/// that is not swiftlint's JSON or puts a checked violation outside `folder`, is `Failed`. A check swiftlint does not
/// know is `Failed` with swiftlint's exit code and a message naming the check and quoting swiftlint's warning line,
/// not its list of valid ids. A check of `custom_rules` is `Failed` before anything runs: under `--only-rule`
/// that name runs no custom rule, so swiftlint reports nothing and the rule could match nothing.
pub(crate) fn hits_swiftlint(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    if rule.check == "custom_rules" {
        return Err(Stop::Failed {
            code: None,
            stderr: "swiftlint check `custom_rules` names no rule swiftlint runs under --only-rule: name the custom \
                     rule's own identifier"
                .to_string(),
        });
    }
    let out = ran(&swiftlint_cmd(&rule.check, rule.config.as_deref(), folder), host)?;
    parse_swiftlint(&out, &rule.check, folder, folder).map_err(|err| match unknown_rule(&out) {
        Some(_) => Stop::Failed { code: out.code, stderr: err.why },
        None => parse_failed(&out, err),
    })
}

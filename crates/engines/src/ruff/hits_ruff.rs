//! `hits_ruff`: one ruff rule run over its rule file's folder, read into hits.

use super::{parse_ruff, ruff_cmd, ruff_refusal};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical): ruff runs there through `host` with the rule's check and, when the
/// rule gives one, its config. A check tm cannot match to its findings (`ruff_refusal`) is `Failed` with no exit code,
/// and nothing runs (settled S-35). ruff absent is `NotAvailable`; a run that could not start, or output that is not
/// ruff's JSON, is `Failed`.
pub(crate) fn hits_ruff(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    if let Some(why) = ruff_refusal(&rule.check) {
        return Err(Stop::Failed { code: None, stderr: why });
    }
    let out = ran(&ruff_cmd(&rule.check, rule.config.as_deref(), folder), host)?;
    parse_ruff(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

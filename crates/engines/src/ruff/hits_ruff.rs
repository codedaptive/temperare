//! `hits_ruff`: one ruff rule run over its rule file's folder, read into hits.

use super::{parse_ruff, ruff_cmd};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical): ruff runs there through `host` with the rule's check and, when the
/// rule gives one, its config. ruff absent is `NotAvailable`; a run that could not start, or output that is not
/// ruff's JSON, is `Failed`.
pub(crate) fn hits_ruff(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let out = ran(&ruff_cmd(&rule.check, rule.config.as_deref(), folder), host)?;
    parse_ruff(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

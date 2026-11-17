//! `hits_swiftlint`: one swiftlint rule run over its rule file's folder, read into hits.

use super::{parse_swiftlint, swiftlint_cmd};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical): swiftlint runs there through `host` with the rule's check and,
/// when the rule gives one, its config. swiftlint absent is `NotAvailable`; a run that could not start, or output
/// that is not swiftlint's JSON, is `Failed`.
pub(crate) fn hits_swiftlint(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let out = ran(&swiftlint_cmd(&rule.check, rule.config.as_deref(), folder), host)?;
    parse_swiftlint(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

//! `Report`: what a check came to, the `--json` body.

use crate::{Counts, EngineVersion, Failed, NotApplicable, NotAvailable, RuleCount, Silenced};
use procio::Oid;
use rules::Finding;
use serde::{Serialize, Serializer};

/// The report of one check. `exceptions_in_force`, `not_applicable`, `date`, `silenced_by_rule` and `engines` are
/// always serialized, 0, "" and `[]` included; `base` is left out when the check had no base commit and otherwise is
/// the commit id's hex text.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Report {
    /// The kept findings: on new lines when there is a base, and covered by no exception.
    pub findings: Vec<Finding>,
    pub silenced: Vec<Silenced>,
    /// How many exceptions the rule file grants, whether or not any finding used one.
    pub exceptions_in_force: usize,
    pub not_available: Vec<NotAvailable>,
    pub failed: Vec<Failed>,
    #[serde(serialize_with = "oid_text", skip_serializing_if = "Option::is_none")]
    pub base: Option<Oid>,
    /// The rules the check does not apply, in rule-file order, each with why: none is a finding or "not available".
    pub not_applicable: Vec<NotApplicable>,
    /// The UTC day the exceptions were looked up on (`RuleFile::exception_on`), as "YYYY-MM-DD".
    pub date: String,
    /// The silenced findings counted by rule: rule-file order, then rules the file lacks; only rules with any.
    pub silenced_by_rule: Vec<RuleCount>,
    /// One per engine that ran (found and probed for its version), in the order of each one's first applicable rule in
    /// the rule file; its version is `None` when it could not be read. An absent engine is left out of this list; its
    /// rules show under `not_available` or `not_applicable`, or are dropped with no base when only_with_base.
    /// Filled by the caller: `build_report` leaves it empty.
    pub engines: Vec<EngineVersion>,
    pub counts: Counts,
}

/// The id's hex text: `procio::Oid` has no `Serialize`.
fn oid_text<S: Serializer>(base: &Option<Oid>, serializer: S) -> Result<S::Ok, S::Error> {
    match base {
        Some(oid) => serializer.serialize_str(oid.as_str()),
        None => serializer.serialize_none(),
    }
}

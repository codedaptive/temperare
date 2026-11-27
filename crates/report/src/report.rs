//! `Report`: what a check came to, the `--json` body.

use crate::{
    Counts, CoversNothing, EngineVersion, Expired, Failed, NotApplicable, NotAvailable, RuleCount, RuleRecord, Silenced,
};
use crate::Oid;
use rules::Finding;
use serde::{Serialize, Serializer};

/// The report of one check. `exceptions_in_force`, `covers_nothing`, `not_applicable`, `expired`, `date`,
/// `silenced_by_rule` and `engines` are always serialized, 0, "" and `[]` included; `base` is left out when the check
/// had no base commit and otherwise is the commit id's hex text. A watch rule's outcomes reach no list and no count,
/// only `record`, which is never serialized, so the `--json` body does not carry it.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Report {
    /// The kept findings: on new lines when there is a base, and covered by no exception.
    pub findings: Vec<Finding>,
    pub silenced: Vec<Silenced>,
    /// How many of the rule file's exceptions hold on the report's day (none of `expired`), whether or not any finding
    /// used one.
    pub exceptions_in_force: usize,
    pub not_available: Vec<NotAvailable>,
    pub failed: Vec<Failed>,
    #[serde(serialize_with = "oid_text", skip_serializing_if = "Option::is_none")]
    pub base: Option<Oid>,
    /// The repository's own rules that cover nothing, in rule-file order: none is a finding, "not available" or clean.
    pub covers_nothing: Vec<CoversNothing>,
    /// The rules the check does not apply, in rule-file order, each with why: none is a finding or "not available".
    pub not_applicable: Vec<NotApplicable>,
    /// The exceptions past their `expires` day on the report's day, in rule-file order.
    pub expired: Vec<Expired>,
    /// The UTC day the exceptions were looked up on (`RuleFile::exception_on`), as "YYYY-MM-DD".
    pub date: String,
    /// The silenced findings counted by rule: rule-file order, then rules the file lacks; only rules with any.
    pub silenced_by_rule: Vec<RuleCount>,
    /// One per engine that ran (found and probed for its version), in the order of each one's first applicable rule in
    /// the rule file; its version is `None` when it could not be read. An absent engine is left out of this list; its
    /// rules other than watch rules show under `not_available`, `not_applicable` or `covers_nothing`.
    /// Filled by the caller: `build_report_with_no_files` leaves it empty.
    pub engines: Vec<EngineVersion>,
    pub counts: Counts,
    /// The telemetry record's rules list (see `RuleRecord`), filled by `build_report_with_no_files`. Not serialized,
    /// so `--json` does not carry it; the caller reads it.
    #[serde(skip)]
    pub record: Vec<RuleRecord>,
}

/// The id's hex text: `Oid` has no `Serialize`.
fn oid_text<S: Serializer>(base: &Option<Oid>, serializer: S) -> Result<S::Ok, S::Error> {
    match base {
        Some(oid) => serializer.serialize_str(oid.as_str()),
        None => serializer.serialize_none(),
    }
}

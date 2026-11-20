//! `Report`: what a check came to, the `--json` body.

use crate::{Counts, Failed, NotAvailable, Silenced};
use procio::Oid;
use rules::Finding;
use serde::{Serialize, Serializer};

/// The report of one check. `exceptions_in_force` is always serialized, 0 included; `base` is left out when the
/// check had no base commit and otherwise is the commit id's hex text.
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
    pub counts: Counts,
}

/// The id's hex text: `procio::Oid` has no `Serialize`.
fn oid_text<S: Serializer>(base: &Option<Oid>, serializer: S) -> Result<S::Ok, S::Error> {
    match base {
        Some(oid) => serializer.serialize_str(oid.as_str()),
        None => serializer.serialize_none(),
    }
}

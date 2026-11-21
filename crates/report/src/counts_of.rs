//! `Counts::of`: the counts of a report's lists.

use crate::Counts;
use rules::{Finding, Kind, Severity};

impl Counts {
    /// Counts the kept `findings` (all, by severity, by kind) and carries the other lists' lengths as given.
    pub(crate) fn of(
        findings: &[Finding],
        silenced: usize,
        not_available: usize,
        failed: usize,
        not_applicable: usize,
        covers_nothing: usize,
    ) -> Counts {
        let count = |keep: fn(&Finding) -> bool| findings.iter().filter(|f| keep(f)).count();
        Counts {
            findings: findings.len(),
            errors: count(|f| f.severity == Severity::Error),
            warnings: count(|f| f.severity == Severity::Warning),
            quality: count(|f| f.kind == Kind::Quality),
            security: count(|f| f.kind == Kind::Security),
            silenced,
            not_available,
            failed,
            not_applicable,
            covers_nothing,
        }
    }
}

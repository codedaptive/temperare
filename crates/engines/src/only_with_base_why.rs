//! `ONLY_WITH_BASE_WHY`: why a rule that reports only what a change adds is not available without a base.

/// The `Stop::NotAvailable` why of an entry that needs the check's base (`Host::base`) and has none. The same text
/// as report's why for such rules, so a reader sees one sentence whichever side found it missing.
pub(crate) const ONLY_WITH_BASE_WHY: &str = "reports only what a change adds; run with --base";

//! `Availability`: whether an engine can be run here.

/// Whether an engine can be run here: the programs its entry needs are found (ESLint's own may be the rule file's
/// folder's `node_modules/.bin/eslint`, the rest on PATH), except Tessera, which is never available.
/// `NotAvailable` is never "clean": its `why` says what is missing and is the text a rule that needs the engine
/// reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Availability {
    Available,
    NotAvailable { why: String },
}

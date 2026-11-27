//! `CaseVerdict`: what one case of a rule came to under `cctao baseline test`.

/// One case's verdict: it holds (the rule gave what the case expects), it is broken (`why` says what went wrong), the
/// rule's engine could not be run here (`why` says why, and `cause` is engines' cause for it, `None` when the run
/// carried none), or the case is not applicable and was not run (`why` says why, for example the case gives no value
/// for a required param).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CaseVerdict {
    Holds,
    Broken { why: String },
    NotAvailable { why: String, cause: Option<engines::NotAvailableCause> },
    NotApplicable { why: String },
}

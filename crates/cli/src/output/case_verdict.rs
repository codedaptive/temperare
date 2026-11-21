//! `CaseVerdict`: what one case of a rule came to under `tm baseline test`.

/// One case's verdict: it holds (the rule gave what the case expects), it is broken (`why` says what went wrong), the
/// rule's engine could not be run here (`why` says why, and `cause` is engines' cause for it, `None` when the run
/// carried none), or the rule is set aside and was not run (`why` is rules' reason, for example its required params
/// are not given).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CaseVerdict {
    Holds,
    Broken { why: String },
    NotAvailable { why: String, cause: Option<engines::NotAvailableCause> },
    NotApplicable { why: String },
}

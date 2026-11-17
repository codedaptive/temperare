//! `ParamError`: a rule's params could not fill its template.

/// Why a `{{name}}` in a rule template could not be filled from the rule's params.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamError {
    /// The template names a param the rule does not give.
    Unknown { name: String },
    /// The param is there but is not a string or a list of strings; `why` says what it is.
    Bad { name: String, why: String },
}

//! `ParamError`: a rule's params could not be used: to fill its template, or by a clippy rule.

/// Why a rule's params could not be used: a `{{name}}` in a rule template could not be filled from them, or a clippy
/// rule's params are not just an `allow` list of strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamError {
    /// The template names a param the rule does not give.
    Unknown { name: String },
    /// The param is there but cannot be used (a template param that is not a string or a list of strings; for clippy, a
    /// param other than `allow`, or an `allow` that is not a list of strings); `why` says what is wrong.
    Bad { name: String, why: String },
}

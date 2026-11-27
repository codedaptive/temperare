//! `ParamError`: a rule's params could not be used: to fill its template, or as arguments to its engine.

/// Why a rule's params could not be used: a `{{name}}` in a rule template could not be filled from them, a param the
/// template never names was given, or the params are not what the engine takes (any param for an engine that takes
/// none; for clippy, anything but an `allow` list of strings).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamError {
    /// The template names a param the rule does not give.
    Unknown { name: String },
    /// The param is there but cannot be used (a template param that is not a string or a list of strings, or that the
    /// template never names; a param the engine does not take, which is any param for an engine that takes none; for
    /// clippy, an `allow` that is not a list of strings); `why` says what is wrong.
    Bad { name: String, why: String },
}

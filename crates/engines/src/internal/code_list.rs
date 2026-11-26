//! `CodeList`: an engine's codes as temperare lists them.

/// The codes a rule may name, for an engine that never says it does not know a code: a code not in `codes` is not
/// available. Engine-neutral; `read_code_list` reads it from its text form. `source` and `version` name where the codes
/// were copied from and at which engine release.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CodeList<'a> {
    pub(crate) source: &'a str,
    pub(crate) version: &'a str,
    pub(crate) codes: Vec<&'a str>,
}

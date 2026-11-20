//! `BaselineSet`: one version of a baseline set that ships inside `tm`.

/// The set `name` at `version`, whose rules are the rule-file `text`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BaselineSet {
    pub(crate) name: &'static str,
    pub(crate) version: u32,
    pub(crate) text: &'static str,
}

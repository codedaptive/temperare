//! `BaselineSet`: one version of a baseline set that ships inside `tm`.

/// The set `name` at `version`, whose rules are the rule-file `text`, and the ast-grep YAML files it carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BaselineSet {
    pub(crate) name: &'static str,
    pub(crate) version: u32,
    pub(crate) text: &'static str,
    /// Each YAML file the set carries: its path relative to the set's folder, as a set rule's `check` names it, and
    /// the file's text.
    pub(crate) yaml: &'static [(&'static str, &'static str)],
}

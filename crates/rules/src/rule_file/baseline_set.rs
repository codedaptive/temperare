//! `BaselineSet`: one version of a baseline set that ships inside `tm`.

/// The set `name` at `version`, whose rules are the rule-file `text`, and the ast-grep YAML files and case files it
/// carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct BaselineSet {
    pub(crate) name: &'static str,
    pub(crate) version: u32,
    /// Whether a rule file's `baseline` may take the set: one not ready is refused there, its rules not yet proven.
    /// `baseline_rules` loads every set, ready or not; `tm baseline test` takes it when cli has the command.
    pub(crate) ready: bool,
    pub(crate) text: &'static str,
    /// Each YAML file the set carries: its path relative to the set's folder, as a set rule's `check` names it, and
    /// the file's text.
    pub(crate) yaml: &'static [(&'static str, &'static str)],
    /// Each case file the set carries: its path relative to the set's folder,
    /// `cases/<rule id>/<failing|passing>/<file>` where `<file>` is as the rule's case lists it in `files`, and the
    /// file's text.
    pub(crate) cases: &'static [(&'static str, &'static str)],
}

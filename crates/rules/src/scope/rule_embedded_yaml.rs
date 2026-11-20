//! `Rule::embedded_yaml`: the ast-grep YAML a baseline set carries for the rule.

use super::Rule;

impl Rule {
    /// The text of the rule's ast-grep YAML when the baseline set it came from carries it in tm, for the engine to
    /// run; None for a repository's own rule, whose `check` names a file in the repository, and for a set rule of
    /// any other engine.
    pub fn embedded_yaml(&self) -> Option<&'static str> {
        self.embedded_text
    }
}

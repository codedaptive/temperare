//! `BaselineSet::case_for`: the text of a case file the set carries.

use super::baseline_set::BaselineSet;

impl BaselineSet {
    /// The text of the first carried case file whose path equals `path` exactly, or None when the set carries none by
    /// that path. Paths are compared as written: "cases/tm-a/failing/./a.rs" does not find "cases/tm-a/failing/a.rs".
    pub(crate) fn case_for(&self, path: &str) -> Option<&'static str> {
        self.cases.iter().find(|(carried, _)| *carried == path).map(|(_, text)| *text)
    }
}

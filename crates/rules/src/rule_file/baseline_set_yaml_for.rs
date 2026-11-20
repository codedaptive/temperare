//! `BaselineSet::yaml_for`: the text of a YAML file the set carries.

use super::baseline_set::BaselineSet;

impl BaselineSet {
    /// The text of the first carried YAML file whose path equals `check` exactly, or None when the set carries none
    /// by that path. Paths are compared as written: "./x.yml" does not find "x.yml".
    pub(crate) fn yaml_for(&self, check: &str) -> Option<&'static str> {
        self.yaml.iter().find(|(path, _)| *path == check).map(|(_, text)| *text)
    }
}

//! `Harness::from_name`: the harness a `--target` name names.

use super::Harness;

impl Harness {
    /// The harness `name` names, exactly as `Harness::name` writes it; `None` for any other text.
    pub fn from_name(name: &str) -> Option<Harness> {
        Harness::ALL.into_iter().find(|harness| harness.name() == name)
    }
}

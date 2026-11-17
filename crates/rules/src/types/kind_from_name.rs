//! `Kind::from_name`: a kind from its rule-file name.

use super::Kind;

impl Kind {
    /// The kind named `name` in a rule file ("quality" or "security"), or None for any other text.
    pub fn from_name(name: &str) -> Option<Kind> {
        match name {
            "quality" => Some(Kind::Quality),
            "security" => Some(Kind::Security),
            _ => None,
        }
    }
}

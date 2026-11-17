//! `Oid::as_str`: the id's text.

use super::Oid;

impl Oid {
    /// The id as 40 or 64 lowercase hex digits.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

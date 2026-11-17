//! `Oid::parse`: the one way to make an `Oid`.

use super::Oid;

impl Oid {
    /// The id in `text`, when `text` is exactly 40 or 64 ASCII hex digits; `None` otherwise. Uppercase is accepted
    /// and stored lowercase, so equal ids compare equal. Nothing is trimmed: a caller reading git's output strips
    /// the newline itself, so stray whitespace is never silently taken as part of an id.
    pub fn parse(text: &str) -> Option<Oid> {
        let valid = matches!(text.len(), 40 | 64) && text.bytes().all(|b| b.is_ascii_hexdigit());
        valid.then(|| Oid(text.to_ascii_lowercase()))
    }
}

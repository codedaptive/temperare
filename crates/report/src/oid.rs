//! `Oid`: a git object id.

/// A git object id, always 40 (SHA-1) or 64 (SHA-256) lowercase hex digits. Outside this crate only `Oid::parse` makes
/// one, so the text is always valid, and two ids for the same object compare equal whatever case git or the caller wrote.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Oid(pub(crate) String);

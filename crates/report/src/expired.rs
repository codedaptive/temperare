//! `Expired`: an exception past its `expires` day.

use rules::RelPath;
use serde::Serialize;

/// An exception past its `expires` day: it silences nothing. Serialized with its fields in this order, `what` as null
/// when None.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Expired {
    pub rule: String,
    pub path: RelPath,
    pub what: Option<String>,
    /// The exception's last day, as "YYYY-MM-DD".
    pub expires: String,
    pub reason: String,
    pub approved: String,
}

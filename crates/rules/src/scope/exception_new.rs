//! `Exception::new`: an exception built in code.

use super::Exception;
use crate::RelPath;

impl Exception {
    /// An exception to `rule` at `path` (normalised by `RelPath::new`), with no `what`: it covers every finding of the
    /// rule at that path. A caller sets `what` on the result when it needs one.
    pub fn new(rule: &str, path: &str, reason: &str, approved: &str) -> Exception {
        Exception {
            rule: rule.to_string(),
            path: RelPath::new(path),
            what: None,
            reason: reason.to_string(),
            approved: approved.to_string(),
        }
    }
}

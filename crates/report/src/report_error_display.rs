//! `Display` for `ReportError`: one line saying what failed.

use crate::ReportError;
use std::fmt;

impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportError::Git(err) => write!(f, "git: {err}"),
            ReportError::Folder { folder, why } => write!(f, "rule file folder {}: {why}", folder.display()),
        }
    }
}

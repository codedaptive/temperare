//! `std::error::Error` for `ReportError`: the git error is the source.

use crate::ReportError;
use std::error::Error;

impl Error for ReportError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ReportError::Git(err) => Some(err),
            ReportError::Folder { .. } => None,
        }
    }
}

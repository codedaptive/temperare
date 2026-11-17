//! `std::error::Error` for `ProcError`, so callers can use `?` into boxed errors and walk the cause.

use std::error::Error;

use crate::ProcError;

impl Error for ProcError {
    /// The OS error behind a `Spawn`; the other variants have no underlying cause.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProcError::Spawn { io, .. } => Some(io),
            ProcError::NotFound { .. } | ProcError::TimedOut { .. } => None,
        }
    }
}

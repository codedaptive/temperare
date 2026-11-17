//! `Display` for `ProcError`: one line naming the program and what went wrong.

use std::fmt;

use crate::ProcError;

impl fmt::Display for ProcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcError::NotFound { program } => write!(f, "{}: not found", program.to_string_lossy()),
            ProcError::Spawn { program, io } => write!(f, "{}: could not run: {io}", program.to_string_lossy()),
            ProcError::TimedOut { program, after } => {
                write!(f, "{}: timed out after {after:?}", program.to_string_lossy())
            }
        }
    }
}

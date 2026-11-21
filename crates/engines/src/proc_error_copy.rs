//! `proc_error_copy`: a second `ProcError` equal in what it says to the first.

use procio::ProcError;
use std::io;

/// The same variant as `err`, naming the same program: `TimedOut` with the same `after`; `Spawn` with an `io::Error`
/// of the same kind and the same message (an OS error's code is not kept, only its words). `ProcError` is not
/// `Clone`, and one failed shared run must reach every rule that shares it as the same `Stop` (`ran` reads
/// `NotFound` as `NotOnPath`, naming the program, and the others as failed, quoting their message).
pub(crate) fn proc_error_copy(err: &ProcError) -> ProcError {
    match err {
        ProcError::NotFound { program } => ProcError::NotFound { program: program.clone() },
        ProcError::Spawn { program, io: why } => {
            ProcError::Spawn { program: program.clone(), io: io::Error::new(why.kind(), why.to_string()) }
        }
        ProcError::TimedOut { program, after } => ProcError::TimedOut { program: program.clone(), after: *after },
    }
}

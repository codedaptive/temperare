//! `spawn_reader`: drain a pipe on its own thread.

use std::io::{self, Read};
use std::thread::{self, JoinHandle};

/// Reads `r` to its end on a new thread and hands back every byte. One per output pipe lets both drain at once,
/// so a child that fills one pipe while `run` waits on the other never blocks.
pub(crate) fn spawn_reader<R: Read + Send + 'static>(mut r: R) -> JoinHandle<io::Result<Vec<u8>>> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        r.read_to_end(&mut bytes)?;
        Ok(bytes)
    })
}

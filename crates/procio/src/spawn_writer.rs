//! `spawn_writer`: feed a pipe on its own thread, then close it.

use std::io::{self, Write};
use std::thread::{self, JoinHandle};

/// Writes all of `bytes` to `w` on a new thread, then drops `w`, which closes the pipe so the child sees the end
/// of its input. On its own thread so a child that writes output before reading its input cannot deadlock `run`.
/// A broken pipe counts as done: the child need not read its stdin.
pub(crate) fn spawn_writer<W: Write + Send + 'static>(mut w: W, bytes: Vec<u8>) -> JoinHandle<io::Result<()>> {
    thread::spawn(move || match w.write_all(&bytes) {
        Err(err) if err.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        written => written,
    })
}

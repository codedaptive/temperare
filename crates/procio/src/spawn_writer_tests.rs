use std::io::{self, Write};
use std::sync::{Arc, Mutex};

use crate::spawn_writer::spawn_writer;

/// A writer that keeps what it is given in a buffer the test also holds.
struct Shared(Arc<Mutex<Vec<u8>>>);

impl Write for Shared {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// A writer whose every write fails with the given kind.
struct Failing(io::ErrorKind);

impl Write for Failing {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::Error::from(self.0))
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn every_byte_reaches_the_writer() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    spawn_writer(Shared(Arc::clone(&buffer)), b"hello\nprocio\n".to_vec()).join().unwrap().unwrap();
    assert_eq!(*buffer.lock().unwrap(), b"hello\nprocio\n");
}

#[test]
fn the_writer_is_dropped_when_done() {
    let buffer = Arc::new(Mutex::new(Vec::new()));
    spawn_writer(Shared(Arc::clone(&buffer)), b"x".to_vec()).join().unwrap().unwrap();
    assert_eq!(Arc::strong_count(&buffer), 1);
}

#[test]
fn a_broken_pipe_is_ok() {
    assert!(spawn_writer(Failing(io::ErrorKind::BrokenPipe), b"x".to_vec()).join().unwrap().is_ok());
}

#[test]
fn another_write_error_is_returned() {
    let err = spawn_writer(Failing(io::ErrorKind::PermissionDenied), b"x".to_vec()).join().unwrap().unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::PermissionDenied);
}

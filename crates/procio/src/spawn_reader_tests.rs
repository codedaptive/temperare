use std::io::{self, Cursor, Read};

use crate::spawn_reader::spawn_reader;

/// A reader whose every read fails.
struct Failing;

impl Read for Failing {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("read failed"))
    }
}

#[test]
fn reads_everything() {
    let bytes = spawn_reader(Cursor::new(b"one\ntwo\n".to_vec())).join().unwrap().unwrap();
    assert_eq!(bytes, b"one\ntwo\n");
}

#[test]
fn reads_past_a_pipe_buffer() {
    let input: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
    let bytes = spawn_reader(Cursor::new(input.clone())).join().unwrap().unwrap();
    assert_eq!(bytes.len(), 200_000);
    assert_eq!(bytes, input);
}

#[test]
fn an_empty_reader_gives_no_bytes() {
    assert_eq!(spawn_reader(Cursor::new(Vec::new())).join().unwrap().unwrap(), b"");
}

#[test]
fn a_read_error_is_returned() {
    let err = spawn_reader(Failing).join().unwrap().unwrap_err();
    assert_eq!(err.to_string(), "read failed");
}

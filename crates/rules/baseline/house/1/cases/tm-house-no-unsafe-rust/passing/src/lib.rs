//! The header byte of a frame.

/// The first byte of `frame`, or `None` when it is empty.
pub fn header(frame: &[u8]) -> Option<u8> {
    frame.first().copied()
}

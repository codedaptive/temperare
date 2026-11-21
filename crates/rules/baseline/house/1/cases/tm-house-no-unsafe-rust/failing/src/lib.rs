//! The header byte of a frame.

/// The first byte of `frame`, or `None` when it is empty.
pub fn header(frame: &[u8]) -> Option<u8> {
    if frame.is_empty() {
        return None;
    }
    Some(unsafe { *frame.get_unchecked(0) })
}

//! The size of a file in kilobytes.

/// `bytes` in whole kilobytes.
pub fn kilobytes(bytes: u64) -> u32 {
    (bytes / 1024) as u32
}

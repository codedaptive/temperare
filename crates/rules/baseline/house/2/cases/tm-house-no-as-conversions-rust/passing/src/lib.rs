//! The size of a file in kilobytes.

use std::num::TryFromIntError;

/// `bytes` in whole kilobytes, or an error when that does not fit in a `u32`.
pub fn kilobytes(bytes: u64) -> Result<u32, TryFromIntError> {
    u32::try_from(bytes / 1024)
}

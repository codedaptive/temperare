//! The port a server listens on, read from its configuration text.

use std::num::ParseIntError;

/// The port in `text`, such as "8080".
pub fn port(text: &str) -> Result<u16, ParseIntError> {
    text.trim().parse()
}

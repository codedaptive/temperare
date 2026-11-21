//! The port a server listens on, read from its configuration text.

/// The port in `text`, such as "8080".
pub fn port(text: &str) -> u16 {
    text.trim().parse().unwrap()
}

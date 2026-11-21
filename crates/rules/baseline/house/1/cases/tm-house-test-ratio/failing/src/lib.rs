//! A config's port: read from its text and written back.
/// The port in `text`, or None when it is not a number from 1 to 65535.
pub fn parse_port(text: &str) -> Option<u16> {
    text.trim().parse().ok().filter(|port| *port != 0)
}
/// `port` as the line a config file holds.
pub fn format_port(port: u16) -> String {
    format!("port = {port}")
}

//! A config's port: read from its text, checked against the ports a server may take, and written back.
/// The lowest port a server run without privileges may take.
pub const FIRST_UNPRIVILEGED: u16 = 1024;
/// The port in `text`, or None when it is not a number from 1 to 65535.
pub fn parse_port(text: &str) -> Option<u16> {
    text.trim().parse().ok().filter(|port| *port != 0)
}
/// `port` as the line a config file holds.
pub fn format_port(port: u16) -> String {
    format!("port = {port}")
}
/// Whether `port` needs privileges to listen on.
pub fn is_privileged(port: u16) -> bool {
    port < FIRST_UNPRIVILEGED
}
/// The port of a config's `text`: the line `port = N`, or `default` when there is none.
pub fn port_of(text: &str, default: u16) -> Option<u16> {
    match text.lines().find_map(|line| line.strip_prefix("port =")) {
        Some(value) => parse_port(value),
        None => Some(default),
    }
}
/// The ports in `text` that a server without privileges may not take, in the order they appear.
pub fn privileged_ports(text: &str) -> Vec<u16> {
    text.lines()
        .filter_map(|line| line.strip_prefix("port ="))
        .filter_map(parse_port)
        .filter(|port| is_privileged(*port))
        .collect()
}
/// `ports` as the lines a config file holds, one per port.
pub fn format_ports(ports: &[u16]) -> String {
    let lines: Vec<String> = ports.iter().map(|port| format_port(*port)).collect();
    lines.join("\n")
}

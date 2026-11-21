fn describe_port(port: u16) -> String {
    format!("port {port} is open for connections")
}

#[test]
fn describes_an_open_port() {
    assert_eq!(describe_port(8080), "port 8080 is open for connections");
}

struct Description {
    port: u16,
    open: bool,
}

fn describe_port(port: u16) -> Description {
    Description { port, open: true }
}

#[test]
fn describes_an_open_port() {
    let description = describe_port(8080);
    assert_eq!(description.port, 8080);
    assert!(description.open, "port 8080 should be open for connections");
}

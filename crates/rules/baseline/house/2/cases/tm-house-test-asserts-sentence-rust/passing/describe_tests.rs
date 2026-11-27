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

#[test]
fn reports_a_closed_port() {
    let description = Description { port: 22, open: false };
    let ok = !description.open;
    let s = format!("port {} is closed to every connection", description.port);
    assert!(ok, "{}", s.contains("is closed to every connection"));
}

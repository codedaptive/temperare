fn parse_port(text: &str) -> Option<u16> {
    text.parse().ok()
}

#[test]
fn reads_a_port() {
    assert_eq!(parse_port("8080"), Some(8080));
}

#[test]
#[ignore]
fn refuses_a_port_out_of_range() {
    assert_eq!(parse_port("70000"), None);
}

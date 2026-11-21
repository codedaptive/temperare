fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse()
}

#[test]
fn reads_a_port() {
    assert!(parse_port("8080").is_ok());
}

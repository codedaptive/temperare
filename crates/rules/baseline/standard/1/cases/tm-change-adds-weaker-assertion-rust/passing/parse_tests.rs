fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse()
}

#[test]
fn reads_a_port() {
    assert_eq!(parse_port("8080"), Ok(8080));
}

#[test]
fn reads_the_same_port_twice() {
    let first = parse_port("8080");
    let second = parse_port("8080");
    assert!(first == second && first.is_ok());
}

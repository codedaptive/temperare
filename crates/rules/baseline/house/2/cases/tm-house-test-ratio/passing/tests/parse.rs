use case::parse_port;
#[test]
fn reads_a_port() {
    assert_eq!(parse_port("8080"), Some(8080));
}
#[test]
fn trims_spaces() {
    assert_eq!(parse_port(" 8080 "), Some(8080));
}
#[test]
fn refuses_zero() {
    assert_eq!(parse_port("0"), None);
}
#[test]
fn refuses_a_word() {
    assert_eq!(parse_port("http"), None);
}
#[test]
fn refuses_past_the_top() {
    assert_eq!(parse_port("65536"), None);
}
#[test]
fn reads_the_top() {
    assert_eq!(parse_port("65535"), Some(65535));
}
#[test]
fn refuses_a_sign() {
    assert_eq!(parse_port("-1"), None);
}
#[test]
fn refuses_an_empty_text() {
    assert_eq!(parse_port(""), None);
}

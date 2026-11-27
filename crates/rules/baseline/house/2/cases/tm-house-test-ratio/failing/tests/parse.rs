use case::{parse_port, port_or};
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
#[test]
fn port_or_reads_a_port() {
    assert_eq!(port_or("8080", 80), 8080);
}
#[test]
fn port_or_trims_spaces() {
    assert_eq!(port_or(" 8080 ", 80), 8080);
}
#[test]
fn port_or_reads_one() {
    assert_eq!(port_or("1", 80), 1);
}
#[test]
fn port_or_reads_the_top() {
    assert_eq!(port_or("65535", 80), 65535);
}
#[test]
fn port_or_defaults_on_zero() {
    assert_eq!(port_or("0", 80), 80);
}
#[test]
fn port_or_defaults_on_a_word() {
    assert_eq!(port_or("http", 80), 80);
}
#[test]
fn port_or_defaults_past_the_top() {
    assert_eq!(port_or("65536", 80), 80);
}
#[test]
fn port_or_defaults_on_a_sign() {
    assert_eq!(port_or("-1", 80), 80);
}
#[test]
fn port_or_defaults_on_an_empty_text() {
    assert_eq!(port_or("", 80), 80);
}
#[test]
fn port_or_keeps_another_default() {
    assert_eq!(port_or("", 443), 443);
}

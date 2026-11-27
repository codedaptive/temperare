use case::format_port;
#[test]
fn writes_a_port() {
    assert_eq!(format_port(8080), "port = 8080");
}
#[test]
fn writes_the_top() {
    assert_eq!(format_port(65535), "port = 65535");
}
#[test]
fn writes_one() {
    assert_eq!(format_port(1), "port = 1");
}

use super::Oid;

#[test]
fn display_writes_the_lowercase_text() {
    let oid = Oid::parse("ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789");
    assert_eq!(
        oid.map(|o| o.to_string()),
        Some("abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789".to_string())
    );
}

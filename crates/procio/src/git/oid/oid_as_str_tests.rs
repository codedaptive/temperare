use super::Oid;

#[test]
fn as_str_is_the_lowercase_text() {
    let oid = Oid::parse("FEDCBA9876543210FEDCBA9876543210FEDCBA98");
    assert_eq!(oid.as_ref().map(Oid::as_str), Some("fedcba9876543210fedcba9876543210fedcba98"));
}

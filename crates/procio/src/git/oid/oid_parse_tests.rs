use super::Oid;

const SHA1: &str = "0123456789abcdef0123456789abcdef01234567";
const SHA256: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

#[test]
fn forty_hex_digits_is_an_id() {
    assert_eq!(Oid::parse(SHA1).map(|o| o.0), Some(SHA1.to_string()));
}

#[test]
fn sixty_four_hex_digits_is_an_id() {
    assert_eq!(Oid::parse(SHA256).map(|o| o.0), Some(SHA256.to_string()));
}

#[test]
fn thirty_nine_digits_is_not_an_id() {
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef0123456"), None);
}

#[test]
fn forty_one_digits_is_not_an_id() {
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef012345678"), None);
}

#[test]
fn sixty_three_and_sixty_five_digits_are_not_ids() {
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcde"), None);
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0"), None);
}

#[test]
fn every_length_but_forty_and_sixty_four_is_refused() {
    for len in (0..=80).filter(|n| *n != 40 && *n != 64) {
        assert_eq!(Oid::parse(&"a".repeat(len)), None, "length {len}");
    }
}

#[test]
fn empty_is_not_an_id() {
    assert_eq!(Oid::parse(""), None);
}

#[test]
fn uppercase_is_accepted_and_stored_lowercase() {
    let oid = Oid::parse("0123456789ABCDEF0123456789ABCDEF01234567");
    assert_eq!(oid.map(|o| o.0), Some("0123456789abcdef0123456789abcdef01234567".to_string()));
}

#[test]
fn mixed_case_equals_lowercase() {
    assert_eq!(Oid::parse("0123456789AbCdEf0123456789aBcDeF01234567"), Oid::parse(SHA1));
}

#[test]
fn a_non_hex_digit_is_refused() {
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef0123456g"), None);
    assert_eq!(Oid::parse("z123456789abcdef0123456789abcdef01234567"), None);
}

#[test]
fn surrounding_whitespace_is_not_trimmed() {
    assert_eq!(Oid::parse("0123456789abcdef0123456789abcdef01234567\n"), None);
    assert_eq!(Oid::parse(" 0123456789abcdef0123456789abcdef0123456"), None);
}

#[test]
fn non_ascii_of_forty_bytes_is_refused() {
    // 20 two-byte characters: forty bytes, none of them a hex digit.
    assert_eq!(Oid::parse(&"é".repeat(20)), None);
}

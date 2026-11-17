use super::unquote_c_path;

#[test]
fn octal_escapes_are_one_byte_each() {
    assert_eq!(unquote_c_path(br#""caf\303\251.rs""#), Some(vec![b'c', b'a', b'f', 0xC3, 0xA9, b'.', b'r', b's']));
}

#[test]
fn quote_and_backslash_escapes() {
    assert_eq!(unquote_c_path(br#""a\"b\\c""#), Some(br#"a"b\c"#.to_vec()));
}

#[test]
fn letter_escapes() {
    assert_eq!(unquote_c_path(br#""\a\b\t\n\v\f\r""#), Some(vec![0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D]));
}

#[test]
fn plain_bytes_and_spaces_stand_for_themselves() {
    assert_eq!(unquote_c_path(br#""dir/a b.rs""#), Some(b"dir/a b.rs".to_vec()));
}

#[test]
fn largest_and_smallest_octal() {
    assert_eq!(unquote_c_path(br#""\377\000""#), Some(vec![0xFF, 0x00]));
}

#[test]
fn not_starting_with_a_quote_is_none() {
    assert_eq!(unquote_c_path(b"plain.rs"), None);
}

#[test]
fn no_closing_quote_is_none() {
    assert_eq!(unquote_c_path(br#""open.rs"#), None);
}

#[test]
fn bytes_after_the_closing_quote_are_none() {
    assert_eq!(unquote_c_path(br#""a.rs"x"#), None);
}

#[test]
fn unknown_escape_is_none() {
    assert_eq!(unquote_c_path(br#""a\q.rs""#), None);
}

#[test]
fn octal_with_a_bad_digit_is_none() {
    assert_eq!(unquote_c_path(br#""a\38.rs""#), None);
}

#[test]
fn octal_with_too_few_digits_is_none() {
    assert_eq!(unquote_c_path(br#""a\30""#), None);
}

#[test]
fn octal_past_a_byte_is_none() {
    assert_eq!(unquote_c_path(br#""\400""#), None);
}

#[test]
fn escaped_closing_quote_does_not_close() {
    assert_eq!(unquote_c_path(br#""a\""#), None);
}

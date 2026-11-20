//! Unit tests for kind_from_name.rs: "quality", "security", and None for anything else.

use super::Kind;

#[test]
fn quality() {
    assert_eq!(Kind::from_name("quality"), Some(Kind::Quality));
}

#[test]
fn security() {
    assert_eq!(Kind::from_name("security"), Some(Kind::Security));
}

#[test]
fn unknown_names_are_none() {
    for name in ["rule", "", "Security", "QUALITY", "sec", "style"] {
        assert_eq!(Kind::from_name(name), None, "{name:?}");
    }
}

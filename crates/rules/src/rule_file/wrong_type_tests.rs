//! ZUT for wrong_type.rs.

use super::wrong_type::wrong_type;
use toml::de::DeTable;

#[test]
fn names_the_key_the_expected_and_the_found_type_at_the_value() {
    let text = "paths = 3\n";
    let doc = DeTable::parse(text).unwrap();
    let p = wrong_type(text, "paths", doc.get_ref().get("paths").unwrap(), "an array of strings");
    assert_eq!((p.line, p.col), (1, 9));
    assert_eq!(p.what, "`paths` must be an array of strings, not integer");
    assert!(p.fix.contains("paths"), "{}", p.fix);
}

//! ZUT for hit.rs: plain data; equality covers every field, the column included.

use super::Hit;
use rules::RelPath;

fn hit(col: Option<u32>) -> Hit {
    Hit { path: RelPath::new("src/a.rs"), line: Some(3), col, what: "equality checks against true are unnecessary".to_string() }
}

#[test]
fn hits_compare_by_every_field() {
    assert_eq!(hit(Some(8)), hit(Some(8)).clone());
    assert_ne!(hit(Some(8)), hit(None));
    assert_ne!(hit(None), Hit { line: Some(4), ..hit(None) });
    assert_ne!(hit(None), Hit { path: RelPath::new("src/b.rs"), ..hit(None) });
    assert_ne!(hit(None), Hit { what: "other".to_string(), ..hit(None) });
}

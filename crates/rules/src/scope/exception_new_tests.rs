//! Unit tests for exception_new.rs: the fields come out as given, the path normalised, no `what` and no `expires`.

use crate::{Exception, RelPath};

#[test]
fn the_fields_come_out_as_given_with_no_what_and_no_expires() {
    let e = Exception::new("no-unwrap", "./src\\a.rs", "startup only", "Bob, 2026-11-17");
    assert_eq!(e.rule, "no-unwrap");
    assert_eq!(e.path, RelPath::new("src/a.rs"));
    assert_eq!(e.what, None);
    assert_eq!(e.expires, None);
    assert_eq!(e.reason, "startup only");
    assert_eq!(e.approved, "Bob, 2026-11-17");
}

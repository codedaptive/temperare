//! Unit tests for exception.rs.

use crate::{Exception, RelPath};

fn exception(path: &str, what: Option<&str>) -> Exception {
    Exception {
        rule: "no-unwrap".to_string(),
        path: RelPath::new(path),
        what: what.map(str::to_string),
        expires: None,
        reason: "startup only".to_string(),
        approved: "Bob, 2026-11-17".to_string(),
    }
}

#[test]
fn exceptions_with_the_same_fields_are_equal() {
    assert_eq!(exception("src/a.rs", Some("x")), exception("./src/a.rs", Some("x")));
}

//! ZUT for kind_value.rs.

use super::kind_value::kind_value;
use crate::{Kind, Problem};
use toml::de::DeTable;

/// `kind_value` on the whole of `text` as one table.
fn read(text: &str) -> (Option<Kind>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    (kind_value(text, doc.get_ref(), &mut out), out)
}

#[test]
fn an_absent_kind_is_quality() {
    let (kind, out) = read("id = \"r\"\n");
    assert_eq!(kind, Some(Kind::Quality));
    assert_eq!(out, vec![]);
}

#[test]
fn security_is_read() {
    let (kind, out) = read("kind = \"security\"\n");
    assert_eq!(kind, Some(Kind::Security));
    assert_eq!(out, vec![]);
}

#[test]
fn an_unknown_kind_is_none_with_a_problem_at_the_value() {
    let (kind, out) = read("id = \"r\"\nkind = \"style\"\n");
    assert_eq!(kind, None);
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 8,
            what: "unknown kind \"style\"".to_string(),
            fix: "Use \"quality\" or \"security\".".to_string(),
        }]
    );
}

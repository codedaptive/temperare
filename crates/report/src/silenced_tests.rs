//! ZUT for silenced.rs: the exact JSON text, with and without the exception's `what`.

use crate::test_json::to_json;
use crate::Silenced;
use rules::{Engine, Exception, Finding, Kind, RelPath, Severity};

fn silenced(what: Option<&str>) -> Silenced {
    Silenced {
        finding: Finding {
            rule: "no-unwrap".to_string(),
            engine: Engine::Clippy,
            kind: Kind::Quality,
            severity: Severity::Error,
            owner: None,
            path: RelPath::new("src/a.rs"),
            line: 3,
            col: Some(9),
            what: "used unwrap".to_string(),
            fix: "Return a typed error.".to_string(),
        },
        exception: Exception {
            rule: "no-unwrap".to_string(),
            path: RelPath::new("src/a.rs"),
            what: what.map(str::to_string),
            reason: "startup only".to_string(),
            approved: "Bob, 2026-11-17".to_string(),
        },
    }
}

const FINDING: &str = r#"{"rule":"no-unwrap","engine":"clippy","kind":"quality","severity":"error","path":"src/a.rs","line":3,"col":9,"what":"used unwrap","fix":"Return a typed error."}"#;

#[test]
fn an_exception_with_what_serializes_every_field() {
    let expected = format!(
        r#"{{"finding":{FINDING},"exception":{{"rule":"no-unwrap","path":"src/a.rs","what":"unwrap","reason":"startup only","approved":"Bob, 2026-11-17"}}}}"#
    );
    assert_eq!(to_json(&silenced(Some("unwrap"))), expected);
}

#[test]
fn an_exception_without_what_leaves_it_out() {
    let expected = format!(
        r#"{{"finding":{FINDING},"exception":{{"rule":"no-unwrap","path":"src/a.rs","reason":"startup only","approved":"Bob, 2026-11-17"}}}}"#
    );
    assert_eq!(to_json(&silenced(None)), expected);
}

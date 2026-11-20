//! Unit tests for silenced.rs: the exact JSON text, with and without the exception's `what`.

use crate::test_json::to_json;
use crate::Silenced;
use rules::{Engine, Exception, Finding, Kind, RelPath, Severity};

fn silenced(what: Option<&str>) -> Silenced {
    let mut exception = Exception::new("no-unwrap", "src/a.rs", "startup only", "maintainer, 2026-11-17");
    exception.what = what.map(str::to_string);
    Silenced {
        finding: Finding::new(
            "no-unwrap",
            (Engine::Clippy, Kind::Quality, Severity::Error),
            RelPath::new("src/a.rs"),
            3,
            Some(9),
            "used unwrap",
            "Return a typed error.",
        ),
        exception,
    }
}

const FINDING: &str = r#"{"rule":"no-unwrap","engine":"clippy","kind":"quality","severity":"error","path":"src/a.rs","line":3,"col":9,"what":"used unwrap","fix":"Return a typed error."}"#;

#[test]
fn an_exception_with_what_serializes_every_field() {
    let expected = format!(
        r#"{{"finding":{FINDING},"exception":{{"rule":"no-unwrap","path":"src/a.rs","what":"unwrap","reason":"startup only","approved":"maintainer, 2026-11-17"}}}}"#
    );
    assert_eq!(to_json(&silenced(Some("unwrap"))), expected);
}

#[test]
fn an_exception_without_what_leaves_it_out() {
    let expected = format!(
        r#"{{"finding":{FINDING},"exception":{{"rule":"no-unwrap","path":"src/a.rs","reason":"startup only","approved":"maintainer, 2026-11-17"}}}}"#
    );
    assert_eq!(to_json(&silenced(None)), expected);
}

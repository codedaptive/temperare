//! ZUT for exception_covers_finding.rs: same rule, same path, and `what`, when given, contained in the finding's.

use crate::{Engine, Exception, Finding, Kind, RelPath, Severity};

fn finding(rule: &str, path: &str, what: &str) -> Finding {
    Finding {
        rule: rule.to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line: 3,
        col: Some(9),
        what: what.to_string(),
        fix: "Return a typed error.".to_string(),
    }
}

#[test]
fn an_exception_covers_a_finding_of_its_rule_and_path_whose_what_contains_its_own() {
    // (case, exception what, finding rule, finding path, finding what, covers)
    let rows = [
        ("match", None, "no-unwrap", "src/a.rs", "anything", true),
        ("other rule", None, "no-expect", "src/a.rs", "anything", false),
        ("other path", None, "no-unwrap", "src/b.rs", "anything", false),
        ("what contained", Some("unwrap"), "no-unwrap", "src/a.rs", "used `unwrap()` on a Result", true),
        ("what not contained", Some("expect"), "no-unwrap", "src/a.rs", "used `unwrap()` on a Result", false),
    ];
    for (case, what, rule, path, finding_what, covers) in rows {
        let e = Exception { what: what.map(str::to_string), ..Exception::new("no-unwrap", "src/a.rs", "legacy", "Bob") };
        assert_eq!(e.covers_finding(&finding(rule, path, finding_what)), covers, "{case}");
    }
}

//! Unit tests for counts_of.rs: the kept findings counted by severity and kind, the other lists' lengths carried each
//! to its own count (every carried value unequal, so a swap of any two arguments fails).

use crate::Counts;
use rules::{Engine, Finding, Kind, RelPath, Severity};

fn finding(kind: Kind, severity: Severity) -> Finding {
    Finding::new("r", (Engine::Ruff, kind, severity), RelPath::new("a.py"), 1, None, "w", "f")
}

#[test]
fn findings_are_counted_by_severity_and_kind_and_the_rest_carried() {
    let findings = vec![
        finding(Kind::Quality, Severity::Error),
        finding(Kind::Security, Severity::Error),
        finding(Kind::Security, Severity::Warning),
        finding(Kind::Security, Severity::Error),
    ];
    let rows = [
        (
            findings,
            [5, 6, 7, 8, 9, 10],
            Counts {
                findings: 4,
                errors: 3,
                warnings: 1,
                quality: 1,
                security: 3,
                silenced: 5,
                not_available: 6,
                failed: 7,
                not_applicable: 8,
                covers_nothing: 9,
                expired: 10,
            },
        ),
        (
            Vec::new(),
            [6, 1, 5, 2, 4, 3],
            Counts {
                findings: 0,
                errors: 0,
                warnings: 0,
                quality: 0,
                security: 0,
                silenced: 6,
                not_available: 1,
                failed: 5,
                not_applicable: 2,
                covers_nothing: 4,
                expired: 3,
            },
        ),
    ];
    for (findings, [silenced, not_available, failed, not_applicable, covers_nothing, expired], expected) in rows {
        let counts = Counts::of(&findings, silenced, not_available, failed, not_applicable, covers_nothing, expired);
        assert_eq!(counts, expected, "{} findings", findings.len());
    }
}

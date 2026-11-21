//! Unit tests for judge_case.rs: each outcome of a failing and of a passing case, the whys written out by hand. Every row is
//! given the cause `EngineNotOnPath`; only the not-available verdict carries it.

use super::judge_case::judge_case;
use crate::output::CaseVerdict;
use engines::NotAvailableCause::EngineNotOnPath;
use rules::{Engine, ExpectedFinding, Finding, Kind, RelPath, RuleOutcome, Severity};

fn finding(rule: &str, path: &str, line: u32) -> Finding {
    Finding::new(rule, (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new(path), line, None, "w", "f")
}

fn broken(why: &str) -> CaseVerdict {
    CaseVerdict::Broken { why: why.to_string() }
}

#[test]
fn judge_case_rows() {
    let expected = ExpectedFinding { rule_id: "tm-zeta".to_string(), path: "./pkg/a.py".to_string(), line: 3 };
    let found = |list: Vec<Finding>| RuleOutcome::Findings(list);
    let failed = |code: Option<i32>, stderr: &str| RuleOutcome::Failed {
        rule: "tm-zeta".to_string(),
        engine: Engine::Ruff,
        code,
        stderr: stderr.to_string(),
    };
    let rows = [
        ("failing held", true, found(vec![finding("tm-zeta", "pkg/a.py", 3)]), CaseVerdict::Holds),
        ("failing gave none", true, found(vec![]), broken("gave no finding; expected tm-zeta at pkg/a.py:3")),
        (
            "failing, wrong line",
            true,
            found(vec![finding("tm-zeta", "pkg/a.py", 4)]),
            broken("gave tm-zeta at pkg/a.py:4; expected tm-zeta at pkg/a.py:3"),
        ),
        (
            "failing, wrong path",
            true,
            found(vec![finding("tm-zeta", "pkg/b.py", 3)]),
            broken("gave tm-zeta at pkg/b.py:3; expected tm-zeta at pkg/a.py:3"),
        ),
        (
            "failing, wrong rule",
            true,
            found(vec![finding("tm-alpha", "pkg/a.py", 3)]),
            broken("gave tm-alpha at pkg/a.py:3; expected tm-zeta at pkg/a.py:3"),
        ),
        (
            "failing, two findings, one of them right",
            true,
            found(vec![finding("tm-zeta", "pkg/a.py", 3), finding("tm-zeta", "pkg/a.py", 9)]),
            broken("gave tm-zeta at pkg/a.py:3, tm-zeta at pkg/a.py:9; expected tm-zeta at pkg/a.py:3"),
        ),
        ("passing held", false, found(vec![]), CaseVerdict::Holds),
        (
            "passing gave one",
            false,
            found(vec![finding("tm-zeta", "pkg/a.py", 3)]),
            broken("gave tm-zeta at pkg/a.py:3; expected no finding"),
        ),
        (
            "not available",
            true,
            RuleOutcome::NotAvailable {
                rule: "tm-zeta".to_string(),
                engine: Engine::Ruff,
                why: "ruff not found on PATH".to_string(),
            },
            CaseVerdict::NotAvailable { why: "ruff not found on PATH".to_string(), cause: Some(EngineNotOnPath) },
        ),
        (
            "failed with a code, stderr's last non-blank line",
            false,
            failed(Some(2), "warning: first\nerror: bad config\n  \n"),
            broken("the rule failed, exit 2: error: bad config"),
        ),
        ("failed with no code, empty stderr", true, failed(None, ""), broken("the rule failed, no exit code")),
    ];
    let normalized_expected = ExpectedFinding { rule_id: "tm-zeta".into(), path: "src//./a.py".into(), line: 3 };
    let normalized_row = ("failing, normalized expected path", true,
        found(vec![finding("tm-zeta", "src/a.py", 3)]), CaseVerdict::Holds, &normalized_expected);
    for (name, failing, outcome, want, expected) in rows.into_iter()
        .map(|(name, failing, outcome, want)| (name, failing, outcome, want, &expected))
        .chain(std::iter::once(normalized_row))
    {
        assert_eq!(judge_case(failing, &outcome, Some(EngineNotOnPath), expected), want, "{name}");
    }
}

use crate::{rules_record, CoversNothing, NotApplicable, RecordFinding, RuleRecord, RuleState, Silenced};
use rules::{Engine, Exception, Finding, Kind, RelPath, Rule, RuleFile, Severity};
use std::path::PathBuf;

#[test]
fn records_follow_rule_order_states_and_rule_specific_findings() {
    let mut rules = RuleFile::new(PathBuf::from("."));
    let mut ran = Rule::new("ran", Engine::Ruff, "lint");
    ran.severity = Severity::Warning;
    ran.position = 0;
    let mut aside = Rule::new("aside", Engine::SwiftLint, "lint");
    aside.position = 1;
    let mut unavailable = Rule::new("unavailable", Engine::Clippy, "lint");
    unavailable.position = 2;
    let mut failed = Rule::new("failed", Engine::Clippy, "lint");
    failed.position = 3;
    let mut covered = Rule::new("covered", Engine::Clippy, "lint");
    covered.position = 4;
    let mut missing = Rule::new("missing", Engine::Clippy, "lint");
    missing.position = 5;
    rules.rules = vec![covered, missing, failed, unavailable, ran];
    rules.not_applicable_rules = vec![aside];
    let outcomes = vec![
        ("ran".into(), RuleState::Ran, None),
        ("aside".into(), RuleState::NotAvailable, Some("engine missing".into())),
        ("unavailable".into(), RuleState::NotAvailable, Some("absent".into())),
        ("unavailable".into(), RuleState::Failed, None),
        ("failed".into(), RuleState::Failed, None),
        ("covered".into(), RuleState::Ran, None),
    ];
    let not_applicable = vec![NotApplicable { rule: "aside".into(), engine: Engine::SwiftLint, why: "set aside".into() }];
    let covers = vec![CoversNothing { rule: "covered".into(), engine: Engine::Clippy }];
    let no_files = vec![("covered".into(), "no files".into()), ("covered".into(), "later".into())];
    let found = |rule, path, line, what| Finding::new(rule, (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new(path), line, None, what, "fix");
    let kept = vec![found("ran", "z.rs", Some(9), "first"), found("failed", "other.rs", Some(2), "other"), found("ran", "a.rs", None, "second")];
    let silenced = vec![
        Silenced { finding: found("ran", "s.rs", Some(1), "hidden"), exception: Exception::new("ran", "s.rs", "reason", "approved") },
        Silenced { finding: found("ran", "s.rs", Some(2), "hidden"), exception: Exception::new("ran", "s.rs", "reason", "approved") },
        Silenced { finding: found("failed", "s.rs", Some(3), "hidden"), exception: Exception::new("failed", "s.rs", "reason", "approved") },
    ];
    let record = rules_record(&rules, &outcomes, (&covers, &not_applicable), &no_files, &kept, &silenced);
    assert_eq!(record, vec![
        RuleRecord { rule: "ran".into(), engine: Engine::Ruff, severity: "warning".into(), state: RuleState::Ran, why: None, findings: vec![
            RecordFinding { path: RelPath::new("z.rs"), line: Some(9), what: "first".into() },
            RecordFinding { path: RelPath::new("a.rs"), line: None, what: "second".into() },
        ], silenced: 2 },
        RuleRecord { rule: "aside".into(), engine: Engine::SwiftLint, severity: "error".into(), state: RuleState::NotApplicable, why: Some("set aside".into()), findings: vec![], silenced: 0 },
        RuleRecord { rule: "unavailable".into(), engine: Engine::Clippy, severity: "error".into(), state: RuleState::NotAvailable, why: Some("absent".into()), findings: vec![], silenced: 0 },
        RuleRecord { rule: "failed".into(), engine: Engine::Clippy, severity: "error".into(), state: RuleState::Failed, why: None, findings: vec![
            RecordFinding { path: RelPath::new("other.rs"), line: Some(2), what: "other".into() },
        ], silenced: 1 },
        RuleRecord { rule: "covered".into(), engine: Engine::Clippy, severity: "error".into(), state: RuleState::CoversNothing, why: Some("no files".into()), findings: vec![], silenced: 0 },
    ]);
}

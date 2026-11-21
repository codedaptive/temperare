//! Unit tests for assemble_report.rs: order, the new-only filter before exceptions, counts, per-rule silenced counts,
//! the carried date and lists, the engines list left empty, the not-applicable rules with the no-base drop, and the
//! flagged rules: own ones cover nothing, baseline ones are not applicable, which wins, and their outcomes dropped,
//! a set-aside rule's apart; the expired exceptions, judged on the day, against those in force.

use crate::{assemble_report, RuleCount};
use procio::Oid;
use rules::{Date, Engine, Exception, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
use std::path::PathBuf;

fn finding(rule: &str, path: &str, line: impl Into<Option<u32>>, col: Option<u32>) -> Finding {
    let grade = (Engine::Clippy, Kind::Quality, Severity::Error);
    Finding::new(rule, grade, RelPath::new(path), line, col, "violation", "Fix it.")
}

fn graded(kind: Kind, severity: Severity, line: u32) -> Finding {
    let mut found = finding("r", "a.rs", line, None);
    found.kind = kind;
    found.severity = severity;
    found
}

fn exception(rule: &str, path: &str) -> Exception {
    Exception::new(rule, path, "approved use", "maintainer, 2026-11-17")
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    let mut rules = RuleFile::new(PathBuf::from("."));
    rules.exceptions = exceptions;
    rules
}

fn day() -> Date {
    Date::new(2026, 11, 20).unwrap()
}

fn date() -> String {
    "2026-11-20".to_string()
}

fn with_rules(ids: &[&str], exceptions: Vec<Exception>) -> RuleFile {
    let mut rules = file(exceptions);
    rules.rules = ids.iter().map(|id| Rule::new(id, Engine::Clippy, "lint")).collect();
    rules
}

fn rule_count(rule: &str, count: usize) -> RuleCount {
    RuleCount { rule: rule.to_string(), count }
}

fn of(findings: Vec<Finding>) -> Vec<(String, RuleOutcome)> {
    vec![("ignored".to_string(), RuleOutcome::Findings(findings))]
}

fn places(findings: &[Finding]) -> Vec<(&str, Option<u32>, Option<u32>, &str)> {
    findings.iter().map(|f| (f.path.as_str(), f.line(), f.col, f.rule.as_str())).collect()
}

#[test]
fn findings_come_out_sorted_by_path_line_col_rule() {
    let shuffled = vec![
        finding("b", "src/b.rs", 1, None),
        finding("z", "src/a.rs", 2, Some(1)),
        finding("a", "src/a.rs", 2, Some(1)),
        finding("a", "src/a.rs", 2, None),
        finding("a", "src/a.rs", 10, None),
        finding("a", "src/a.rs", 2, Some(0)),
    ];
    let report = assemble_report(&file(vec![]), of(shuffled), None, day(), date(), &[], |f| f);
    assert_eq!(
        places(&report.findings),
        vec![
            ("src/a.rs", Some(2), None, "a"),
            ("src/a.rs", Some(2), Some(0), "a"),
            ("src/a.rs", Some(2), Some(1), "a"),
            ("src/a.rs", Some(2), Some(1), "z"),
            ("src/a.rs", Some(10), None, "a"),
            ("src/b.rs", Some(1), None, "b"),
        ]
    );
}

#[test]
fn a_finding_keep_new_drops_is_neither_silenced_nor_counted() {
    let rules = file(vec![exception("old", "a.rs"), exception("new", "a.rs")]);
    let findings = vec![finding("old", "a.rs", 1, None), finding("new", "a.rs", 2, None), finding("x", "a.rs", 3, None)];
    let keep_new = |found: Vec<Finding>| found.into_iter().filter(|f| f.line() != Some(1)).collect();
    let report = assemble_report(&rules, of(findings), None, day(), date(), &[], keep_new);
    assert_eq!(places(&report.findings), vec![("a.rs", Some(3), None, "x")]);
    let silenced: Vec<_> = report.silenced.iter().map(|s| (s.finding.rule.as_str(), s.exception.rule.as_str())).collect();
    assert_eq!(silenced, vec![("new", "new")]);
    assert_eq!((report.counts.findings, report.counts.silenced), (1, 1));
}

#[test]
fn silenced_findings_are_sorted_by_their_finding() {
    let rules = file(vec![exception("r", "a.rs"), exception("r", "b.rs")]);
    let findings = vec![finding("r", "b.rs", 1, None), finding("r", "a.rs", 9, None), finding("r", "a.rs", 4, None)];
    let report = assemble_report(&rules, of(findings), None, day(), date(), &[], |f| f);
    let lines: Vec<_> = report.silenced.iter().map(|s| (s.finding.path.as_str(), s.finding.line())).collect();
    assert_eq!(lines, vec![("a.rs", Some(4)), ("a.rs", Some(9)), ("b.rs", Some(1))]);
}

#[test]
fn counts_are_by_severity_and_kind() {
    let findings = vec![
        graded(Kind::Security, Severity::Warning, 1),
        graded(Kind::Quality, Severity::Error, 2),
        graded(Kind::Security, Severity::Error, 3),
    ];
    let counts = assemble_report(&file(vec![]), of(findings), None, day(), date(), &[], |f| f).counts;
    assert_eq!((counts.findings, counts.errors, counts.warnings), (3, 2, 1));
    assert_eq!((counts.quality, counts.security), (1, 2));
}

#[test]
fn exceptions_in_force_is_carried_with_nothing_silenced() {
    let rules = file(vec![exception("a", "x.rs"), exception("b", "y.rs")]);
    let report = assemble_report(&rules, of(vec![finding("c", "z.rs", 1, None)]), None, day(), date(), &[], |f| f);
    assert_eq!(report.exceptions_in_force, 2);
    assert!(report.silenced.is_empty());
    assert_eq!(report.counts.silenced, 0);
}

#[test]
fn not_available_and_failed_are_carried_sorted_with_a_twenty_line_tail() {
    let stderr: String = (1..=25).map(|n| format!("e{n}\n")).collect();
    let outcomes = vec![
        ("x".to_string(), RuleOutcome::Failed { rule: "lint".to_string(), engine: Engine::Clippy, code: None, stderr }),
        ("x".to_string(), RuleOutcome::NotAvailable { rule: "sec".to_string(), engine: Engine::Tessera, why: "absent".to_string() }),
        ("x".to_string(), RuleOutcome::NotAvailable { rule: "fmt".to_string(), engine: Engine::Ruff, why: "gone".to_string() }),
        ("x".to_string(), RuleOutcome::Failed { rule: "ast".to_string(), engine: Engine::AstGrep, code: Some(2), stderr: "boom\n".to_string() }),
    ];
    let report = assemble_report(&file(vec![]), outcomes, None, day(), date(), &[], |f| f);
    let not_available: Vec<_> = report.not_available.iter().map(|n| (n.rule.as_str(), n.engine, n.why.as_str())).collect();
    assert_eq!(not_available, vec![("fmt", Engine::Ruff, "gone"), ("sec", Engine::Tessera, "absent")]);
    let failed: Vec<_> = report.failed.iter().map(|f| (f.rule.as_str(), f.code)).collect();
    assert_eq!(failed, vec![("ast", Some(2)), ("lint", None)]);
    assert_eq!(report.failed[0].stderr_tail, "boom");
    let tail: Vec<&str> = report.failed[1].stderr_tail.lines().collect();
    assert_eq!((tail.len(), tail[0], tail[19]), (20, "e6", "e25"));
    assert_eq!((report.counts.not_available, report.counts.failed, report.counts.findings), (2, 2, 0));
}

#[test]
fn the_base_is_carried() {
    let oid = Oid::parse("0123456789abcdef0123456789abcdef01234567");
    assert_eq!(assemble_report(&file(vec![]), Vec::new(), oid.clone(), day(), date(), &[], |f| f).base, oid);
}

#[test]
fn the_same_input_gives_equal_reports() {
    let rules = file(vec![exception("r", "a.rs")]);
    let input = || of(vec![finding("r", "a.rs", 1, None), finding("q", "b.rs", 2, Some(3)), finding("q", "a.rs", 7, None)]);
    let report = || assemble_report(&rules, input(), None, day(), date(), &[], |f| f);
    assert_eq!(report(), report());
}

#[test]
fn a_finding_with_no_line_sorts_before_line_one_of_its_path() {
    let findings = vec![finding("r", "a.rs", 1, None), finding("r", "a.rs", None, None), finding("r", "0.rs", 5, None)];
    let report = assemble_report(&file(vec![]), of(findings), None, day(), date(), &[], |f| f);
    assert_eq!(
        places(&report.findings),
        vec![("0.rs", Some(5), None, "r"), ("a.rs", None, None, "r"), ("a.rs", Some(1), None, "r")]
    );
}

#[test]
fn a_silenced_finding_with_no_line_sorts_before_line_one_of_its_path() {
    let findings = vec![finding("r", "a.rs", 1, None), finding("r", "a.rs", None, None)];
    let report = assemble_report(&file(vec![exception("r", "a.rs")]), of(findings), None, day(), date(), &[], |f| f);
    let lines: Vec<_> = report.silenced.iter().map(|s| s.finding.line()).collect();
    assert_eq!(lines, vec![None, Some(1)]);
}

#[test]
fn the_date_is_carried_exactly() {
    let report = assemble_report(&file(vec![]), Vec::new(), None, day(), "1999-12-31".to_string(), &[], |f| f);
    assert_eq!(report.date, "1999-12-31");
}

#[test]
fn silenced_findings_are_counted_by_rule_in_rule_file_order() {
    let rules = with_rules(&["zeta", "alpha"], vec![exception("alpha", "a.rs"), exception("zeta", "a.rs")]);
    let findings =
        vec![finding("alpha", "a.rs", 1, None), finding("zeta", "a.rs", 2, None), finding("alpha", "a.rs", 3, None)];
    let report = assemble_report(&rules, of(findings), None, day(), date(), &[], |f| f);
    assert_eq!(report.silenced_by_rule, vec![rule_count("zeta", 1), rule_count("alpha", 2)]);
}

#[test]
fn a_silenced_finding_keep_new_drops_is_not_counted_by_rule() {
    let rules = with_rules(&["old", "new"], vec![exception("old", "a.rs"), exception("new", "a.rs")]);
    let findings =
        vec![finding("old", "a.rs", 1, None), finding("new", "a.rs", 2, None), finding("new", "a.rs", 3, None)];
    let keep_new = |found: Vec<Finding>| found.into_iter().filter(|f| f.line() != Some(1)).collect();
    let report = assemble_report(&rules, of(findings), None, day(), date(), &[], keep_new);
    assert_eq!(report.silenced_by_rule, vec![rule_count("new", 2)]);
}

#[test]
fn the_counts_by_rule_add_up_to_the_silenced_count() {
    let rules = with_rules(&["b", "a"], vec![exception("a", "a.rs"), exception("b", "a.rs"), exception("c", "a.rs")]);
    let findings = vec![
        finding("a", "a.rs", 1, None),
        finding("c", "a.rs", 2, None),
        finding("b", "a.rs", 3, None),
        finding("a", "a.rs", 4, None),
        finding("d", "a.rs", 5, None),
    ];
    let report = assemble_report(&rules, of(findings), None, day(), date(), &[], |f| f);
    assert_eq!(report.counts.silenced, 4);
    assert_eq!(report.silenced_by_rule.iter().map(|counted| counted.count).sum::<usize>(), 4);
    assert_eq!(report.silenced_by_rule, vec![rule_count("b", 1), rule_count("a", 2), rule_count("c", 1)]);
}

#[test]
fn nothing_silenced_gives_an_empty_silenced_by_rule() {
    let rules = with_rules(&["a"], vec![exception("a", "x.rs")]);
    let report = assemble_report(&rules, of(vec![finding("a", "z.rs", 1, None)]), None, day(), date(), &[], |f| f);
    assert_eq!(report.silenced_by_rule, Vec::new());
}

#[test]
fn the_engines_list_is_left_empty_for_the_caller() {
    let absent = RuleOutcome::NotAvailable { rule: "fmt".to_string(), engine: Engine::Ruff, why: "gone".to_string() };
    let outcomes = vec![
        ("x".to_string(), RuleOutcome::Findings(vec![finding("r", "a.rs", 1, None)])),
        ("y".to_string(), absent),
    ];
    let report = assemble_report(&file(vec![]), outcomes, None, day(), date(), &[], |f| f);
    assert!(report.engines.is_empty());
}

const NEEDS_BASE: &str = "reports only what a change adds; run with --base";

fn not_applicable_rule(id: &str, engine: Engine, why: Option<&str>) -> Rule {
    let mut rule = Rule::new(id, engine, "lint");
    rule.not_applicable = why.map(str::to_string);
    rule
}

fn base_only(id: &str, engine: Engine) -> Rule {
    let mut rule = Rule::new(id, engine, "lint");
    rule.only_with_base = true;
    rule
}

fn listed(report: &crate::Report) -> Vec<(&str, Engine, &str)> {
    report.not_applicable.iter().map(|n| (n.rule.as_str(), n.engine, n.why.as_str())).collect()
}

#[test]
fn a_not_applicable_rule_is_listed_with_its_reason_or_no_reason_given() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![
        not_applicable_rule("swift-x", Engine::SwiftLint, Some("param max not given")),
        not_applicable_rule("py-y", Engine::Ruff, None),
    ];
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(
        listed(&report),
        vec![("swift-x", Engine::SwiftLint, "param max not given"), ("py-y", Engine::Ruff, "no reason given")]
    );
    assert_eq!(report.counts.not_applicable, 2);
}

#[test]
fn with_no_base_a_base_only_rule_is_listed_and_its_outcomes_give_nothing() {
    let mut rules = file(vec![exception("added", "s.rs")]);
    rules.rules = vec![base_only("added", Engine::AstGrep), Rule::new("other", Engine::Clippy, "lint")];
    let outcomes = vec![
        ("x".to_string(), RuleOutcome::Findings(vec![finding("added", "a.rs", 1, None), finding("added", "s.rs", 2, None)])),
        ("x".to_string(), RuleOutcome::Findings(vec![finding("other", "b.rs", 3, None)])),
        ("x".to_string(), RuleOutcome::NotAvailable { rule: "added".to_string(), engine: Engine::AstGrep, why: "gone".to_string() }),
        ("x".to_string(), RuleOutcome::Failed { rule: "added".to_string(), engine: Engine::AstGrep, code: Some(1), stderr: String::new() }),
    ];
    let report = assemble_report(&rules, outcomes, None, day(), date(), &[], |f| f);
    assert_eq!(listed(&report), vec![("added", Engine::AstGrep, NEEDS_BASE)]);
    assert_eq!(places(&report.findings), vec![("b.rs", Some(3), None, "other")]);
    assert!(report.silenced.is_empty());
    assert!(report.silenced_by_rule.is_empty());
    assert!(report.not_available.is_empty());
    assert!(report.failed.is_empty());
    let counts = report.counts;
    assert_eq!((counts.findings, counts.errors, counts.quality, counts.silenced), (1, 1, 1, 0));
    assert_eq!((counts.not_available, counts.failed, counts.not_applicable), (0, 0, 1));
}

#[test]
fn the_tuple_name_does_not_decide_the_drop() {
    let mut rules = file(vec![]);
    rules.rules = vec![base_only("added", Engine::Clippy)];
    let outcomes = vec![("added".to_string(), RuleOutcome::Findings(vec![finding("kept", "a.rs", 1, None)]))];
    let report = assemble_report(&rules, outcomes, None, day(), date(), &[], |f| f);
    assert_eq!(places(&report.findings), vec![("a.rs", Some(1), None, "kept")]);
}

#[test]
fn with_a_base_a_base_only_rule_is_not_listed_and_its_findings_are_kept() {
    let mut rules = file(vec![]);
    rules.rules = vec![base_only("added", Engine::Clippy)];
    let oid = Oid::parse("0123456789abcdef0123456789abcdef01234567");
    let report = assemble_report(&rules, of(vec![finding("added", "a.rs", 4, None)]), oid, day(), date(), &[], |f| f);
    assert_eq!(listed(&report), Vec::new());
    assert_eq!(places(&report.findings), vec![("a.rs", Some(4), None, "added")]);
    assert_eq!((report.counts.findings, report.counts.not_applicable), (1, 0));
}

#[test]
fn with_every_position_equal_the_list_is_the_base_only_rules_then_the_not_applicable_rules_each_in_list_order() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![
        not_applicable_rule("na-z", Engine::Ruff, Some("why z")),
        not_applicable_rule("na-a", Engine::SwiftLint, Some("why a")),
    ];
    rules.rules = vec![
        base_only("b-z", Engine::Clippy),
        Rule::new("plain", Engine::Clippy, "lint"),
        base_only("a-m", Engine::AstGrep),
    ];
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(
        listed(&report),
        vec![
            ("b-z", Engine::Clippy, NEEDS_BASE),
            ("a-m", Engine::AstGrep, NEEDS_BASE),
            ("na-z", Engine::Ruff, "why z"),
            ("na-a", Engine::SwiftLint, "why a"),
        ]
    );
    assert_eq!(report.counts.not_applicable, 4);
}

fn at(position: usize, rule: Rule) -> Rule {
    Rule { position, ..rule }
}

#[test]
fn with_no_base_the_two_kinds_interleave_in_rule_file_order() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![
        at(4, not_applicable_rule("bee", Engine::Ruff, Some("why bee"))),
        at(2, not_applicable_rule("kay", Engine::SwiftLint, None)),
    ];
    rules.rules = vec![
        at(3, base_only("alp", Engine::AstGrep)),
        at(0, Rule::new("plain", Engine::Clippy, "lint")),
        at(1, base_only("zed", Engine::Clippy)),
    ];
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(
        listed(&report),
        vec![
            ("zed", Engine::Clippy, NEEDS_BASE),
            ("kay", Engine::SwiftLint, "no reason given"),
            ("alp", Engine::AstGrep, NEEDS_BASE),
            ("bee", Engine::Ruff, "why bee"),
        ]
    );
    assert_eq!(report.counts.not_applicable, 4);
}

#[test]
fn with_a_base_only_the_not_applicable_rules_are_listed_in_rule_file_order() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![
        at(3, not_applicable_rule("na-m", Engine::Ruff, Some("why m"))),
        at(5, not_applicable_rule("na-a", Engine::SwiftLint, Some("why a"))),
        at(1, not_applicable_rule("na-z", Engine::AstGrep, None)),
    ];
    rules.rules = vec![at(2, base_only("added", Engine::Clippy))];
    let oid = Oid::parse("0123456789abcdef0123456789abcdef01234567");
    let report = assemble_report(&rules, Vec::new(), oid, day(), date(), &[], |f| f);
    assert_eq!(
        listed(&report),
        vec![
            ("na-z", Engine::AstGrep, "no reason given"),
            ("na-m", Engine::Ruff, "why m"),
            ("na-a", Engine::SwiftLint, "why a"),
        ]
    );
    assert_eq!(report.counts.not_applicable, 3);
}

#[test]
fn a_not_applicable_rule_with_only_with_base_is_listed_once_with_its_own_why() {
    let mut rules = file(vec![]);
    let mut given = at(1, not_applicable_rule("given", Engine::Ruff, Some("param max not given")));
    given.only_with_base = true;
    let mut none = at(2, not_applicable_rule("none", Engine::SwiftLint, None));
    none.only_with_base = true;
    rules.not_applicable_rules = vec![given, none];
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(
        listed(&report),
        vec![("given", Engine::Ruff, "param max not given"), ("none", Engine::SwiftLint, "no reason given")]
    );
    assert_eq!(report.counts.not_applicable, 2);
}

#[test]
fn at_a_shared_position_the_base_only_rule_comes_before_the_not_applicable_rule() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![at(2, not_applicable_rule("aaa", Engine::Ruff, Some("why")))];
    rules.rules = vec![at(2, base_only("zzz", Engine::Clippy))];
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(listed(&report), vec![("zzz", Engine::Clippy, NEEDS_BASE), ("aaa", Engine::Ruff, "why")]);
}

#[test]
fn no_rule_out_of_play_gives_an_empty_list_and_a_zero_count() {
    let rules = with_rules(&["a", "b"], vec![]);
    let report = assemble_report(&rules, of(vec![finding("a", "a.rs", 1, None)]), None, day(), date(), &[], |f| f);
    assert_eq!(listed(&report), Vec::new());
    assert_eq!(report.counts.not_applicable, 0);
}

const NO_FILE_MATCH: &str = "its paths match no file in the rule file's folder";
const NO_ENGINE_FILE: &str = "no file its engine checks";

fn flags(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs.iter().map(|(id, why)| (id.to_string(), why.to_string())).collect()
}

fn covering(report: &crate::Report) -> Vec<(&str, Engine)> {
    report.covers_nothing.iter().map(|c| (c.rule.as_str(), c.engine)).collect()
}

fn rule_ids(findings: &[Finding]) -> Vec<&str> {
    findings.iter().map(|f| f.rule.as_str()).collect()
}

#[test]
fn a_flagged_own_rule_covers_nothing_and_its_outcomes_give_nothing() {
    for why in [NO_FILE_MATCH, NO_ENGINE_FILE] {
        let mut rules = file(vec![exception("own-x", "s.rs")]);
        rules.rules = vec![Rule::new("own-x", Engine::SwiftLint, "lint"), Rule::new("other", Engine::Clippy, "lint")];
        let outcomes = vec![
            ("x".to_string(), RuleOutcome::Findings(Vec::new())),
            ("x".to_string(), RuleOutcome::Findings(vec![finding("own-x", "a.rs", 1, None), finding("own-x", "s.rs", 2, None)])),
            ("x".to_string(), RuleOutcome::Findings(vec![finding("other", "b.rs", 3, None)])),
            ("x".to_string(), RuleOutcome::NotAvailable { rule: "own-x".to_string(), engine: Engine::SwiftLint, why: "gone".to_string() }),
            ("x".to_string(), RuleOutcome::Failed { rule: "own-x".to_string(), engine: Engine::SwiftLint, code: Some(1), stderr: String::new() }),
        ];
        let report = assemble_report(&rules, outcomes, None, day(), date(), &flags(&[("own-x", why)]), |f| f);
        assert_eq!(covering(&report), vec![("own-x", Engine::SwiftLint)], "{why}");
        assert_eq!(listed(&report), Vec::new(), "{why}");
        assert_eq!(places(&report.findings), vec![("b.rs", Some(3), None, "other")], "{why}");
        assert!(report.silenced.is_empty() && report.silenced_by_rule.is_empty(), "{why}");
        assert!(report.not_available.is_empty() && report.failed.is_empty(), "{why}");
        let counts = report.counts;
        assert_eq!((counts.covers_nothing, counts.not_applicable, counts.findings, counts.silenced), (1, 0, 1, 0), "{why}");
        assert_eq!((counts.not_available, counts.failed), (0, 0), "{why}");
    }
}

#[test]
fn a_flagged_baseline_rule_is_not_applicable_with_the_flag_why() {
    let rows = [
        (vec![("tm-y", NO_FILE_MATCH)], NO_FILE_MATCH),
        (vec![("tm-y", NO_ENGINE_FILE)], NO_ENGINE_FILE),
        (vec![("tm-y", NO_ENGINE_FILE), ("tm-y", NO_FILE_MATCH)], NO_ENGINE_FILE),
    ];
    for (pairs, why) in rows {
        let mut rules = file(vec![]);
        rules.rules = vec![Rule::new("tm-y", Engine::Ruff, "lint")];
        let outcomes = vec![
            ("x".to_string(), RuleOutcome::Findings(vec![finding("tm-y", "a.rs", 1, None)])),
            ("x".to_string(), RuleOutcome::Failed { rule: "tm-y".to_string(), engine: Engine::Ruff, code: None, stderr: String::new() }),
        ];
        let report = assemble_report(&rules, outcomes, None, day(), date(), &flags(&pairs), |f| f);
        assert_eq!(listed(&report), vec![("tm-y", Engine::Ruff, why)], "{pairs:?}");
        assert_eq!(covering(&report), Vec::new(), "{pairs:?}");
        assert!(report.findings.is_empty() && report.failed.is_empty(), "{pairs:?}");
        assert_eq!((report.counts.covers_nothing, report.counts.not_applicable, report.counts.failed), (0, 1, 0));
    }
}

#[test]
fn flagged_own_and_baseline_rules_are_listed_in_rule_file_order() {
    let mut rules = file(vec![]);
    rules.not_applicable_rules = vec![at(3, not_applicable_rule("na-m", Engine::AstGrep, Some("why m")))];
    rules.rules = vec![
        at(4, Rule::new("tm-b", Engine::Ruff, "lint")),
        at(0, Rule::new("plain", Engine::Clippy, "lint")),
        at(5, Rule::new("own-a", Engine::Clippy, "lint")),
        at(1, Rule::new("own-z", Engine::SwiftLint, "lint")),
        at(6, Rule::new("plain-2", Engine::Ruff, "lint")),
        at(2, Rule::new("tm-a", Engine::Tessera, "lint")),
    ];
    let no_files =
        flags(&[("own-a", NO_FILE_MATCH), ("tm-b", NO_FILE_MATCH), ("own-z", NO_ENGINE_FILE), ("tm-a", NO_ENGINE_FILE)]);
    let report = assemble_report(&rules, Vec::new(), None, day(), date(), &no_files, |f| f);
    assert_eq!(covering(&report), vec![("own-z", Engine::SwiftLint), ("own-a", Engine::Clippy)]);
    assert_eq!(
        listed(&report),
        vec![
            ("tm-a", Engine::Tessera, NO_ENGINE_FILE),
            ("na-m", Engine::AstGrep, "why m"),
            ("tm-b", Engine::Ruff, NO_FILE_MATCH),
        ]
    );
    assert_eq!((report.counts.covers_nothing, report.counts.not_applicable), (2, 3));
}

#[test]
fn a_flag_gives_way_to_a_not_applicable_rule_and_to_a_missing_base() {
    let oid = Oid::parse("0123456789abcdef0123456789abcdef01234567");
    let base_rules = || vec![at(0, base_only("own-b", Engine::Clippy)), at(1, base_only("tm-b", Engine::AstGrep))];
    let rows = [
        (
            "flagged and not applicable",
            vec![
                at(0, not_applicable_rule("own-n", Engine::Ruff, Some("param max not given"))),
                at(1, not_applicable_rule("tm-n", Engine::SwiftLint, None)),
            ],
            Vec::new(),
            None,
            vec![("own-n", Engine::Ruff, "param max not given"), ("tm-n", Engine::SwiftLint, "no reason given")],
            vec![],
            vec!["ghost", "tm-ghost", "plain", "own-b", "tm-b", "own-n", "tm-n"],
        ),
        (
            "flagged, only with a base, no base",
            Vec::new(),
            base_rules(),
            None,
            vec![("own-b", Engine::Clippy, NEEDS_BASE), ("tm-b", Engine::AstGrep, NEEDS_BASE)],
            vec![],
            vec!["ghost", "tm-ghost", "plain", "own-n", "tm-n"],
        ),
        (
            "flagged, only with a base, a base",
            Vec::new(),
            base_rules(),
            oid,
            vec![("tm-b", Engine::AstGrep, NO_FILE_MATCH)],
            vec![("own-b", Engine::Clippy)],
            vec!["ghost", "tm-ghost", "plain", "own-n", "tm-n"],
        ),
        (
            "flagged ids naming no rule",
            Vec::new(),
            vec![Rule::new("plain", Engine::Clippy, "lint")],
            None,
            vec![],
            vec![],
            vec!["ghost", "tm-ghost", "plain", "own-b", "tm-b", "own-n", "tm-n"],
        ),
    ];
    let no_files = flags(&[
        ("own-n", NO_FILE_MATCH),
        ("tm-n", NO_FILE_MATCH),
        ("own-b", NO_FILE_MATCH),
        ("tm-b", NO_FILE_MATCH),
        ("ghost", NO_FILE_MATCH),
        ("tm-ghost", NO_ENGINE_FILE),
    ]);
    for (case, set_aside, applied, base, not_applicable, covers_nothing, kept) in rows {
        let mut rules = file(vec![]);
        rules.not_applicable_rules = set_aside;
        rules.rules = applied;
        let found = ["ghost", "tm-ghost", "plain", "own-b", "tm-b", "own-n", "tm-n"];
        let findings = found.iter().zip(1..).map(|(rule, line)| finding(rule, "a.rs", line, None)).collect();
        let report = assemble_report(&rules, of(findings), base, day(), date(), &no_files, |f| f);
        assert_eq!(listed(&report), not_applicable, "{case}");
        assert_eq!(covering(&report), covers_nothing, "{case}");
        assert_eq!(rule_ids(&report.findings), kept, "{case}");
    }
}

#[test]
fn the_covers_nothing_and_not_applicable_counts_are_the_lists_lengths() {
    let rows = [
        (vec![], vec![], (0, 0)),
        (
            vec![("own-a", NO_FILE_MATCH)],
            vec![not_applicable_rule("na", Engine::Ruff, None), not_applicable_rule("na-2", Engine::Ruff, None)],
            (1, 2),
        ),
        (
            vec![("own-a", NO_FILE_MATCH), ("own-b", NO_ENGINE_FILE), ("tm-c", NO_FILE_MATCH)],
            vec![not_applicable_rule("na", Engine::Ruff, None), not_applicable_rule("na-2", Engine::Ruff, None)],
            (2, 3),
        ),
    ];
    for (pairs, set_aside, (covering_count, not_applicable_count)) in rows {
        let mut rules = with_rules(&["own-a", "own-b", "tm-c"], vec![]);
        rules.not_applicable_rules = set_aside;
        let report = assemble_report(&rules, Vec::new(), None, day(), date(), &flags(&pairs), |f| f);
        let counts = (report.counts.covers_nothing, report.counts.not_applicable);
        assert_eq!(counts, (covering_count, not_applicable_count), "{pairs:?}");
    }
}

fn expiring(rule: &str, path: &str, (year, month, last): (u16, u8, u8)) -> Exception {
    let mut granted = exception(rule, path);
    granted.expires = Some(Date::new(year, month, last).unwrap());
    granted
}

fn expired_rules(report: &crate::Report) -> Vec<&str> {
    report.expired.iter().map(|e| e.rule.as_str()).collect()
}

#[test]
fn an_exception_holds_through_its_expires_day_and_is_expired_the_day_after() {
    let mut granted = expiring("r", "a.rs", (2026, 11, 30));
    granted.what = Some("viol".to_string());
    let rules = file(vec![granted]);
    let gone = crate::Expired {
        rule: "r".to_string(),
        path: RelPath::new("a.rs"),
        what: Some("viol".to_string()),
        expires: "2026-11-30".to_string(),
        reason: "approved use".to_string(),
        approved: "maintainer, 2026-11-17".to_string(),
    };
    let rows = [
        ("the day before", (2026, 11, 29), vec![], 1, (0, 1)),
        ("the expires day", (2026, 11, 30), vec![], 1, (0, 1)),
        ("the day after", (2026, 12, 1), vec![gone], 0, (1, 0)),
    ];
    for (case, (year, month, last), expired, in_force, (kept, silenced)) in rows {
        let on = Date::new(year, month, last).unwrap();
        let report = assemble_report(&rules, of(vec![finding("r", "a.rs", 1, None)]), None, on, date(), &[], |f| f);
        assert_eq!(report.expired, expired, "{case}");
        assert_eq!(report.exceptions_in_force, in_force, "{case}");
        assert_eq!((report.findings.len(), report.silenced.len()), (kept, silenced), "{case}");
        assert_eq!((report.counts.findings, report.counts.silenced), (kept, silenced), "{case}");
    }
}

#[test]
fn an_exception_with_no_expires_is_never_expired_and_always_in_force() {
    let late = Date::new(9999, 12, 31).unwrap();
    let rules = file(vec![exception("r", "a.rs")]);
    let report = assemble_report(&rules, of(vec![finding("r", "a.rs", 1, None)]), None, late, date(), &[], |f| f);
    assert!(report.expired.is_empty());
    assert_eq!(report.exceptions_in_force, 1);
    assert_eq!((report.findings.len(), report.silenced.len()), (0, 1));
}

#[test]
fn expired_exceptions_keep_rule_file_order() {
    let exceptions = vec![expiring("z", "a.rs", (2026, 11, 1)), exception("m", "a.rs"), expiring("a", "a.rs", (2026, 10, 1))];
    let report = assemble_report(&file(exceptions), Vec::new(), None, day(), date(), &[], |f| f);
    assert_eq!(expired_rules(&report), vec!["z", "a"]);
}

#[test]
fn the_expired_count_and_exceptions_in_force_split_the_exceptions() {
    let rows = [
        (
            "two of three expired",
            vec![expiring("a", "a.rs", (2026, 11, 19)), exception("b", "b.rs"), expiring("c", "c.rs", (2025, 1, 1))],
            (2, 1),
        ),
        ("none expired", vec![exception("a", "a.rs"), expiring("b", "b.rs", (2026, 11, 20))], (0, 2)),
    ];
    for (case, exceptions, (expired, in_force)) in rows {
        let report = assemble_report(&file(exceptions), Vec::new(), None, day(), date(), &[], |f| f);
        assert_eq!((report.counts.expired, report.exceptions_in_force), (expired, in_force), "{case}");
        assert_eq!((report.expired.len(), report.counts.covers_nothing), (expired, 0), "{case}");
    }
}

//! ZUT for rule_loosenings.rs, on rule files built here: a row for each kind, each severity move, and a rewritten
//! pattern.

use super::rule_loosenings::rule_loosenings;
use super::{Loosening, LooseningKind};
use crate::{Engine, Rule, RuleFile, Severity};
use std::path::PathBuf;

fn rule() -> Rule {
    Rule::new("r1", Engine::Ruff, "E1")
}

fn strings(patterns: &[&str]) -> Vec<String> {
    patterns.iter().map(|pattern| pattern.to_string()).collect()
}

fn file(rules: Vec<Rule>, not_applicable_rules: Vec<Rule>) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = rules;
    file.not_applicable_rules = not_applicable_rules;
    file
}

fn found(kind: LooseningKind, before: Option<&str>, after: Option<&str>) -> Loosening {
    Loosening { kind, about: "r1".to_string(), before: before.map(String::from), after: after.map(String::from) }
}

#[test]
fn each_way_a_rule_loosens_is_found() {
    let not_applicable = Rule { not_applicable: Some("param max not given".to_string()), ..rule() };
    let rows = [
        ("rule gone", rule(), file(vec![], vec![]), vec![found(LooseningKind::RuleGone, None, None)]),
        (
            "rule now not applicable",
            rule(),
            file(vec![], vec![not_applicable]),
            vec![found(LooseningKind::RuleNotApplicable, None, Some("param max not given"))],
        ),
        (
            "severity lowered",
            rule(),
            file(vec![Rule { severity: Severity::Warning, ..rule() }], vec![]),
            vec![found(LooseningKind::SeverityLowered, Some("error"), Some("warning"))],
        ),
        (
            "error lowered to watch",
            rule(),
            file(vec![Rule { severity: Severity::Watch, ..rule() }], vec![]),
            vec![found(LooseningKind::SeverityLowered, Some("error"), Some("watch"))],
        ),
        (
            "warning lowered to watch",
            Rule { severity: Severity::Warning, ..rule() },
            file(vec![Rule { severity: Severity::Watch, ..rule() }], vec![]),
            vec![found(LooseningKind::SeverityLowered, Some("warning"), Some("watch"))],
        ),
        (
            "watch raised to warning",
            Rule { severity: Severity::Watch, ..rule() },
            file(vec![Rule { severity: Severity::Warning, ..rule() }], vec![]),
            vec![],
        ),
        (
            "path removed",
            Rule { paths: strings(&["src/**", "lib/**"]), ..rule() },
            file(vec![Rule { paths: strings(&["src/**"]), ..rule() }], vec![]),
            vec![found(LooseningKind::PathRemoved, Some("lib/**"), None)],
        ),
        (
            "paths limited",
            rule(),
            file(vec![Rule { paths: strings(&["src/**", "lib/**"]), ..rule() }], vec![]),
            vec![found(LooseningKind::PathsLimited, None, Some("src/**, lib/**"))],
        ),
        (
            "exclude added",
            Rule { exclude: strings(&["gen/**"]), ..rule() },
            file(vec![Rule { exclude: strings(&["gen/**", "vendor/**"]), ..rule() }], vec![]),
            vec![found(LooseningKind::ExcludeAdded, None, Some("vendor/**"))],
        ),
        (
            "rewritten pattern",
            Rule { paths: strings(&["a/**"]), ..rule() },
            file(vec![Rule { paths: strings(&["b/**"]), ..rule() }], vec![]),
            vec![found(LooseningKind::PathRemoved, Some("a/**"), None)],
        ),
        (
            "kit removed",
            Rule { kits: strings(&["a", "b"]), ..rule() },
            file(vec![Rule { kits: strings(&["a"]), ..rule() }], vec![]),
            vec![found(LooseningKind::KitRemoved, Some("b"), None)],
        ),
        (
            "kits limited",
            rule(),
            file(vec![Rule { kits: strings(&["a", "b"]), ..rule() }], vec![]),
            vec![found(LooseningKind::KitsLimited, None, Some("a, b"))],
        ),
        ("kits emptied", Rule { kits: strings(&["a"]), ..rule() }, file(vec![rule()], vec![]), vec![]),
        (
            "kit added to kits",
            Rule { kits: strings(&["a"]), ..rule() },
            file(vec![Rule { kits: strings(&["a", "b"]), ..rule() }], vec![]),
            vec![],
        ),
        (
            "outside kit added",
            rule(),
            file(vec![Rule { outside_kits: strings(&["x"]), ..rule() }], vec![]),
            vec![found(LooseningKind::OutsideKitAdded, None, Some("x"))],
        ),
        ("outside kit removed", Rule { outside_kits: strings(&["x"]), ..rule() }, file(vec![rule()], vec![]), vec![]),
    ];
    for (case, base_rule, now, expected) in rows {
        assert_eq!(rule_loosenings(&file(vec![base_rule], vec![]), &now), expected, "{case}");
    }
}

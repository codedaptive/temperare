//! Unit tests for app/rules_not_applicable.rs: which rules `tm rules` lists as not applicable, with which why, in what order,
//! from rule files built in code and no-file flags as literals.

use super::rules_not_applicable::rules_not_applicable;
use engines::NoFileRule;
use report::NotApplicable;
use rules::{Engine, Rule, RuleFile};
use std::path::PathBuf;

fn rule(id: &str, engine: Engine, position: usize, reason: Option<&str>) -> Rule {
    Rule { position, not_applicable: reason.map(str::to_string), ..Rule::new(id, engine, "check") }
}

fn file(rules: Vec<Rule>, set_aside: Vec<Rule>) -> RuleFile {
    RuleFile { rules, not_applicable_rules: set_aside, ..RuleFile::new(PathBuf::from(".")) }
}

/// A flag with an engine no rule below has, so an entry that took the flag's engine would show it.
fn flag(id: &str, why: &str) -> NoFileRule {
    NoFileRule { rule: id.to_string(), engine: Engine::Clippy, why: why.to_string() }
}

fn listed(id: &str, engine: Engine, why: &str) -> NotApplicable {
    NotApplicable { rule: id.to_string(), engine, why: why.to_string() }
}

#[test]
fn rules_not_applicable_lists_set_aside_then_flagged_baseline_rules_in_rule_file_order() {
    let rows: Vec<(&str, RuleFile, Vec<NoFileRule>, Vec<NotApplicable>)> = vec![
        (
            "nothing listed: own rule, unflagged tm- rule, a reason outside the set-aside list, a flag naming no rule",
            file(
                vec![
                    rule("lint-all", Engine::Ruff, 0, None),
                    rule("tm-applies", Engine::Ruff, 1, None),
                    rule("tm-reasoned", Engine::Ruff, 2, Some("stale reason")),
                ],
                vec![],
            ),
            vec![flag("tm-ghost", "no file of the folder matches its paths")],
            vec![],
        ),
        (
            "several, both kinds, in position order",
            file(
                vec![rule("tm-first", Engine::Ruff, 0, None), rule("tm-third", Engine::Ruff, 2, None)],
                vec![rule("tm-second", Engine::AstGrep, 1, Some("required params not given: entity_types"))],
            ),
            vec![flag("tm-third", "why third"), flag("tm-first", "why first")],
            vec![
                listed("tm-first", Engine::Ruff, "why first"),
                listed("tm-second", Engine::AstGrep, "required params not given: entity_types"),
                listed("tm-third", Engine::Ruff, "why third"),
            ],
        ),
        (
            "an own no-file rule is left out",
            file(vec![rule("no-print", Engine::Ruff, 0, None), rule("tm-flagged", Engine::Ruff, 1, None)], vec![]),
            vec![flag("no-print", "own why"), flag("tm-flagged", "baseline why")],
            vec![listed("tm-flagged", Engine::Ruff, "baseline why")],
        ),
        (
            "a flagged set-aside rule keeps its own reason",
            file(vec![], vec![rule("tm-set-aside", Engine::AstGrep, 0, Some("its own reason"))]),
            vec![flag("tm-set-aside", "flag why")],
            vec![listed("tm-set-aside", Engine::AstGrep, "its own reason")],
        ),
        (
            "the first flag for an id counts",
            file(vec![rule("tm-twice", Engine::Ruff, 0, None)], vec![]),
            vec![flag("tm-twice", "first why"), flag("tm-twice", "second why")],
            vec![listed("tm-twice", Engine::Ruff, "first why")],
        ),
        (
            "a set-aside rule with no reason, its id not tm-",
            file(vec![], vec![rule("own-set-aside", Engine::Ruff, 0, None)]),
            vec![],
            vec![listed("own-set-aside", Engine::Ruff, "no reason given")],
        ),
    ];
    let failures: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, rf, no_files, want)| {
            let got = rules_not_applicable(&rf, &no_files);
            (got != want).then(|| format!("{name}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

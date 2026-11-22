//! Unit tests for out_of_play.rs: set-aside rules with their reason or "no reason given", base-only rules with and
//! without a base, flagged baseline and own rules, unflagged and unknown ids, the first flag for an id, and the walk's
//! rule-file order with the set-aside rules left out of the third list.

use crate::{out_of_play, CoversNothing, NotApplicable};
use rules::{Engine, Rule, RuleFile};
use std::path::PathBuf;

const NEEDS_BASE: &str = "reports only what a change adds; run with --base";

fn at(position: usize, id: &str) -> Rule {
    Rule { position, ..Rule::new(id, Engine::Clippy, "lint") }
}

fn set_aside(position: usize, id: &str, why: Option<&str>) -> Rule {
    Rule { not_applicable: why.map(str::to_string), ..at(position, id) }
}

fn base_only(position: usize, id: &str) -> Rule {
    Rule { only_with_base: true, ..at(position, id) }
}

fn file(rules: Vec<Rule>, not_applicable_rules: Vec<Rule>) -> RuleFile {
    let mut file = RuleFile::new(PathBuf::from("."));
    file.rules = rules;
    file.not_applicable_rules = not_applicable_rules;
    file
}

fn flags(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs.iter().map(|(id, why)| (id.to_string(), why.to_string())).collect()
}

fn na(rule: &str, why: &str) -> NotApplicable {
    NotApplicable { rule: rule.to_string(), engine: Engine::Clippy, why: why.to_string() }
}

fn cn(rule: &str) -> CoversNothing {
    CoversNothing { rule: rule.to_string(), engine: Engine::Clippy }
}

#[test]
fn a_set_aside_rule_is_not_applicable_with_its_reason_and_not_dropped() {
    let rules = file(vec![], vec![set_aside(0, "swift-x", Some("param max not given"))]);
    assert_eq!(out_of_play(&rules, false, &[]), (vec![], vec![na("swift-x", "param max not given")], vec![]));
}

#[test]
fn a_set_aside_rule_with_no_reason_is_not_applicable_with_no_reason_given() {
    let rules = file(vec![], vec![set_aside(0, "py-y", None)]);
    assert_eq!(out_of_play(&rules, false, &[]), (vec![], vec![na("py-y", "no reason given")], vec![]));
}

#[test]
fn a_base_only_rule_is_listed_and_dropped_with_no_base_and_nowhere_with_one() {
    let rules = file(vec![base_only(0, "diff-only")], vec![]);
    assert_eq!(out_of_play(&rules, true, &[]), (vec![], vec![na("diff-only", NEEDS_BASE)], vec!["diff-only"]));
    assert_eq!(out_of_play(&rules, false, &[]), (vec![], vec![], vec![]));
}

#[test]
fn a_flagged_baseline_rule_is_not_applicable_with_the_flag_why_and_dropped() {
    let rules = file(vec![at(0, "tm-swift-1")], vec![]);
    let flagged = flags(&[("tm-swift-1", "no Swift file")]);
    let listed = vec![na("tm-swift-1", "no Swift file")];
    assert_eq!(out_of_play(&rules, false, &flagged), (vec![], listed, vec!["tm-swift-1"]));
}

#[test]
fn a_flagged_own_rule_covers_nothing_and_is_dropped() {
    let rules = file(vec![at(0, "house-x")], vec![]);
    let flagged = flags(&[("house-x", "no file matched")]);
    assert_eq!(out_of_play(&rules, false, &flagged), (vec![cn("house-x")], vec![], vec!["house-x"]));
}

#[test]
fn an_unflagged_rule_is_in_no_list() {
    let rules = file(vec![at(0, "house-x")], vec![]);
    assert_eq!(out_of_play(&rules, true, &[]), (vec![], vec![], vec![]));
}

#[test]
fn a_flagged_id_naming_no_rule_is_ignored() {
    let rules = file(vec![at(0, "house-x")], vec![]);
    let flagged = flags(&[("ghost", "no file matched"), ("tm-ghost", "no Swift file")]);
    assert_eq!(out_of_play(&rules, false, &flagged), (vec![], vec![], vec![]));
}

#[test]
fn the_first_flag_for_an_id_gives_the_why() {
    let rules = file(vec![at(0, "tm-a")], vec![]);
    let flagged = flags(&[("tm-a", "first why"), ("tm-a", "second why")]);
    assert_eq!(out_of_play(&rules, false, &flagged), (vec![], vec![na("tm-a", "first why")], vec!["tm-a"]));
}

#[test]
fn a_mix_is_listed_in_rule_file_order_with_set_aside_rules_never_dropped() {
    let rules = file(
        vec![at(3, "house-own"), base_only(1, "diff-only"), at(4, "tm-base"), at(5, "quiet")],
        vec![set_aside(4, "set-x", Some("why x")), set_aside(0, "set-flagged", None)],
    );
    let flagged = flags(&[("house-own", "no file matched"), ("tm-base", "no Swift file"), ("set-flagged", "no file")]);
    assert_eq!(
        out_of_play(&rules, true, &flagged),
        (
            vec![cn("house-own")],
            vec![
                na("set-flagged", "no reason given"),
                na("diff-only", NEEDS_BASE),
                na("tm-base", "no Swift file"),
                na("set-x", "why x"),
            ],
            vec!["diff-only", "house-own", "tm-base"],
        )
    );
}

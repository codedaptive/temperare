//! `run_baseline_test`: `tm baseline test`, every rule of the sets run on its own failing and passing cases.

use super::run_case::run_case;
use super::run_change_case::run_change_case;
use super::write_failure::write_failure;
use super::BaselineWorld;
use crate::output::{baseline_exit, baseline_json, baseline_next, baseline_text, BaselineRuleResult, CaseVerdict};
use crate::{BaselineArgs, UsageError};
use rules::{Problem, Rule};
use serde_json::Value;
use std::io::Write;

/// Tests the set `--set` names whatever its ready mark, or else each of the world's default sets marked ready, in
/// their order: every set's rules are read before any case runs, and a set that cannot be read is a usage error from
/// its first problem's what and fix (exit 2: on `err`, and under `--json` also the `{"error", "exit", "next"}` object
/// on `out`). Each rule, in its set's order, runs on its failing case and then its passing case (`run_case`; an
/// only_with_base rule's cases run as a change, `run_change_case`); a rule its set carries no cases for is broken on
/// both, and a rule rules set aside (`rule.not_applicable`) is not applicable on both with rules' why and is not run:
/// no case folder is made for it.
/// Prints the results as text with a `next:` line, or as one JSON line carrying `exit` and `next`, and returns the
/// exit code: 1 when a rule is broken, else 3 when one is not applicable or not available, else 0. Write errors are
/// ignored: a closed pipe is not tm's failure.
pub fn run_baseline_test(args: &BaselineArgs, world: &BaselineWorld, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    let sets: Vec<&str> = match &args.set {
        Some(set) => vec![set.as_str()],
        None => world.default_sets.iter().filter(|(_, ready)| *ready).map(|(set, _)| set.as_str()).collect(),
    };
    let mut taken = Vec::new();
    for set in sets {
        match (world.rules_of)(set) {
            Ok(rules) => taken.push((set, rules)),
            Err(problems) => return write_failure(&unreadable(set, problems), args.json, out, err),
        }
    }
    let mut results = Vec::new();
    for (set, rules) in &taken {
        for rule in rules {
            let (failing, passing) = case_verdicts(rule, world, err);
            results.push(BaselineRuleResult {
                set: set.to_string(),
                rule: rule.id.clone(),
                engine: rule.engine,
                failing,
                passing,
            });
        }
    }
    let exit = baseline_exit(&results);
    let next = baseline_next(exit, &results);
    if args.json {
        let mut object = baseline_json(&results);
        if let Value::Object(map) = &mut object {
            map.insert("exit".to_string(), Value::from(exit));
            map.insert("next".to_string(), Value::from(next));
        }
        let _ = writeln!(out, "{object}");
    } else {
        let _ = writeln!(out, "{}next: {next}", baseline_text(&results));
    }
    exit
}

/// The verdicts of `rule`'s failing and passing cases: not applicable on both when rules set it aside, without
/// running it; else broken on both when its set carries no cases for it; else each case run.
fn case_verdicts(rule: &Rule, world: &BaselineWorld, err: &mut dyn Write) -> (CaseVerdict, CaseVerdict) {
    if let Some(why) = &rule.not_applicable {
        let aside = CaseVerdict::NotApplicable { why: why.clone() };
        return (aside.clone(), aside);
    }
    match rule.cases() {
        Some(c) if rule.only_with_base => {
            (run_change_case(rule, true, c, world, err), run_change_case(rule, false, c, world, err))
        }
        Some(c) => (
            run_case(rule, true, &c.failing, &c.expected, world, err),
            run_case(rule, false, &c.passing, &c.expected, world, err),
        ),
        None => (no_cases(), no_cases()),
    }
}

/// The verdict of each case of a rule its set carries no cases for.
fn no_cases() -> CaseVerdict {
    CaseVerdict::Broken { why: "the set carries no cases for this rule".to_string() }
}

/// The usage error of a set that cannot be read: its first problem's what and fix. The problem's line and column index
/// the rule file rules made for the set, not one the user wrote, so they are not shown.
fn unreadable(set: &str, problems: Vec<Problem>) -> UsageError {
    problems.into_iter().next().map_or_else(
        || UsageError {
            what: format!("cannot read baseline set \"{set}\""),
            fix: "Run tm help to see the commands.".to_string(),
        },
        |first| UsageError { what: first.what, fix: first.fix },
    )
}

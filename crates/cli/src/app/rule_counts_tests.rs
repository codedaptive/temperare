use super::rule_counts::rule_counts;
use crate::RuleCount;
use engines::RuleRun;
use rules::{Engine, RuleFile, RuleOutcome};
use std::path::Path;

const NO_RULES: &str = "version = 1\n";
const ERROR_AND_WATCH: &str = "version = 1\n\
[[rule]]\nid = \"e\"\nengine = \"clippy\"\ncheck = \"clippy::all\"\nfix = \"Fix lint.\"\n\
[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F841\"\nseverity = \"watch\"\nfix = \"Remove variable.\"\n";
const WATCH_FIRST: &str = "version = 1\n\
[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F841\"\nseverity = \"watch\"\nfix = \"Remove variable.\"\n\
[[rule]]\nid = \"e\"\nengine = \"ruff\"\ncheck = \"F401\"\nfix = \"Remove import.\"\n";
const WATCH_ONLY: &str = "version = 1\n\
[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F841\"\nseverity = \"watch\"\nfix = \"Remove variable.\"\n\
[[rule]]\nid = \"v\"\nengine = \"ruff\"\ncheck = \"F401\"\nseverity = \"watch\"\nfix = \"Remove import.\"\n";

fn run(id: &str, engine: Engine, chars: Option<usize>) -> RuleRun {
    RuleRun::new(id.into(), engine, RuleOutcome::Findings(vec![]), chars)
}

fn count(rule: &str, engine: &str, chars: u64, ran: bool) -> RuleCount {
    RuleCount { rule: rule.into(), engine: engine.into(), chars, ran }
}

#[test]
fn counts_each_run_in_order_except_a_watch_rules() {
    let rows = [
        (
            "order, engine names, absent and empty runs",
            NO_RULES,
            vec![run("missing", Engine::Ruff, None), run("empty", Engine::AstGrep, Some(0)), run("output", Engine::Ruff, Some(123))],
            vec![count("missing", "ruff", 0, false), count("empty", "ast-grep", 0, true), count("output", "ruff", 123, true)],
        ),
        (
            "a watch rule's run has no count",
            ERROR_AND_WATCH,
            vec![run("e", Engine::Clippy, Some(40)), run("w", Engine::Ruff, Some(90))],
            vec![count("e", "clippy", 40, true)],
        ),
        (
            "shared run with a watch rule first",
            WATCH_FIRST,
            vec![run("w", Engine::Ruff, None), run("e", Engine::Ruff, Some(90))],
            vec![count("e", "ruff", 90, true)],
        ),
        (
            "shared run of watch rules only",
            WATCH_ONLY,
            vec![run("w", Engine::Ruff, None), run("v", Engine::Ruff, None)],
            vec![],
        ),
    ];
    for (name, text, runs, want) in rows {
        let rf = RuleFile::parse(text, Path::new("/repo")).unwrap();
        assert_eq!(rule_counts(&runs, &rf), want, "{name}");
    }
}

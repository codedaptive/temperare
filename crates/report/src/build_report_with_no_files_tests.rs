//! Unit tests for build_report_with_no_files.rs, with no base: a flagged rule of the repository's own covers nothing, a
//! flagged baseline rule is not applicable with the engines' why.

use crate::{build_report_with_no_files, CoversNothing, NotApplicable};
use rules::{Engine, Rule, RuleFile, RuleOutcome};
use std::path::PathBuf;

#[test]
fn a_flagged_rule_is_listed_by_whose_it_is() {
    let rows = [
        (
            "own-x",
            Engine::SwiftLint,
            vec![CoversNothing { rule: "own-x".to_string(), engine: Engine::SwiftLint }],
            vec![],
            (1, 0),
        ),
        (
            "tm-y",
            Engine::Ruff,
            vec![],
            vec![NotApplicable {
                rule: "tm-y".to_string(),
                engine: Engine::Ruff,
                why: "its paths match no file in the rule file's folder".to_string(),
            }],
            (0, 1),
        ),
    ];
    for (id, engine, covers_nothing, not_applicable, (covering, applying)) in rows {
        let mut rules = RuleFile::new(PathBuf::from("."));
        rules.rules = vec![Rule::new(id, engine, "lint")];
        let outcomes = vec![(id.to_string(), RuleOutcome::Findings(Vec::new()))];
        let no_files = vec![(id.to_string(), "its paths match no file in the rule file's folder".to_string())];
        let report = build_report_with_no_files(&rules, outcomes, None, no_files).unwrap();
        assert_eq!(report.covers_nothing, covers_nothing, "{id}");
        assert_eq!(report.not_applicable, not_applicable, "{id}");
        let counts = (report.counts.covers_nothing, report.counts.not_applicable, report.counts.findings);
        assert_eq!(counts, (covering, applying, 0), "{id}");
    }
}

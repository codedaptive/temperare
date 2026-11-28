//! ZUT for rule_file_parse.rs: literal and invalid globs in each rule-file scope.

use super::baseline_set::BaselineSet;
use super::rule_file_parse::parse_with_sets;
use crate::{RelPath, RuleFile};
use std::path::Path;

const RULE: &str = "[[rule]]\nid = \"r\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
const SETS: &[BaselineSet] = &[BaselineSet {
    name: "standard",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-s1\"\nengine = \"clippy\"\ncheck = \"s1\"\nfix = \"f\"\n\
           paths = [\"**\"]\nfailing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\n\
           passing = { files = [\"a.rs\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-s1/failing/a.rs", "x\n"), ("cases/tm-s1/passing/a.rs", "x\n")],
}];

fn cases(glob: &str) -> Vec<(&'static str, String, bool, (usize, usize), &'static str)> {
    vec![
        ("rule paths", format!("version = 1\n{RULE}paths = [\"{glob}\"]\n"), false, (7, 10), "rule \"r\": "),
        (
            "rule exclude",
            format!("version = 1\n{RULE}paths = [\"**\"]\nexclude = [\"{glob}\"]\n"),
            false,
            (8, 12),
            "rule \"r\": ",
        ),
        (
            "override exclude",
            format!(
                "version = 1\nbaseline = [\"standard@1\"]\n[[override]]\nrule = \"tm-s1\"\nexclude = [\"{glob}\"]\n"
            ),
            true,
            (5, 12),
            "[[override]] for \"tm-s1\": ",
        ),
        (
            "kit paths",
            format!("version = 1\n[kits.core]\npaths = [\"{glob}\"]\n{RULE}kits = [\"core\"]\n"),
            false,
            (3, 10),
            "kit \"core\": ",
        ),
    ]
}

#[test]
fn literal_metacharacter_matches_only_literal_paths() {
    let mut failures = Vec::new();
    for (label, text, baseline, _, _) in cases("**/*$") {
        let parsed = if baseline {
            parse_with_sets(&text, Path::new("."), SETS)
        } else {
            RuleFile::parse(&text, Path::new("."))
        };
        match parsed {
            Ok(file) => {
                let rule = &file.rules[0];
                let actual = (rule.applies(&RelPath::new("src/a$")), rule.applies(&RelPath::new("src/a.rs")));
                let expected = if label.contains("exclude") { (false, true) } else { (true, false) };
                if actual != expected {
                    failures.push(format!("{label}: expected {expected:?}, got {actual:?}"));
                }
            }
            Err(problems) => failures.push(format!("{label}: expected a loaded file, got {problems:?}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn invalid_glob_reports_its_location_and_owner() {
    let mut failures = Vec::new();
    for (label, text, baseline, position, owner) in cases("src/[a.rs") {
        let parsed = if baseline {
            parse_with_sets(&text, Path::new("."), SETS)
        } else {
            RuleFile::parse(&text, Path::new("."))
        };
        match parsed {
            Err(problems) if problems.len() == 1 => {
                let problem = &problems[0];
                let prefix = format!("{owner}bad glob \"src/[a.rs\": ");
                if (problem.line, problem.col) != position || !problem.what.starts_with(&prefix) {
                    failures.push(format!("{label}: expected {position:?} and prefix {prefix:?}, got {problem:?}"));
                }
            }
            result => failures.push(format!("{label}: expected exactly one problem, got {result:?}")),
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

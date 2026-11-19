//! ZUT for run_check.rs's exit-3 next line, using closure worlds without disk, git or PATH.

use super::{run_check, CheckWorld};
use crate::CheckArgs;
use procio::Oid;
use report::{Counts, NotAvailable, Report};
use rules::{Engine, RuleFile, RuleOutcome};
use std::path::{Path, PathBuf};

const TEXT: &str = "version = 1
owner = \"core\"
[[rule]]
id = \"q\"
engine = \"ruff\"
check = \"F401\"
fix = \"Remove the import.\"
";
const CARGO_WHY: &str = "no Cargo.toml at or above the rule file's folder";

type Outcomes = Vec<(String, RuleOutcome)>;

fn unavailable(rule: &str, engine: Engine, why: &str) -> NotAvailable {
    NotAvailable { rule: rule.to_string(), engine, why: why.to_string() }
}

fn check(not_available: Vec<NotAvailable>, json: bool) -> (u8, String, String) {
    let build_report = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>| {
        Ok(Report {
            findings: vec![],
            silenced: vec![],
            exceptions_in_force: 0,
            not_available: not_available.clone(),
            failed: vec![],
            base: None,
            counts: Counts {
                findings: 0,
                errors: 0,
                warnings: 0,
                quality: 0,
                security: 0,
                silenced: 0,
                not_available: 0,
                failed: 0,
            },
        })
    };
    let world = CheckWorld {
        toplevel: &|_| Ok(PathBuf::from("/repo")),
        exists: &|_| true,
        read: &|path| {
            assert_eq!(path, Path::new("/repo/temperare.toml"));
            Ok(RuleFile::parse(TEXT, Path::new("/repo")).unwrap())
        },
        rev_parse: &|_, _| unreachable!("rev_parse"),
        run_rules: &|_, _, _| vec![],
        build_report: &build_report,
    };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run_check(
        &CheckArgs { json, ..CheckArgs::default() },
        Path::new("/repo"),
        &world,
        &mut out,
        &mut err,
    );
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

#[test]
fn clippy_without_a_cargo_project_explains_where_to_put_the_rule_file() {
    let (code, out, err) = check(vec![unavailable("c", Engine::Clippy, CARGO_WHY)], false);
    assert_eq!((code, err.as_str()), (3, ""));
    assert_eq!(
        out.split_inclusive('\n').last().unwrap(),
        "next: clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)\n"
    );
}

#[test]
fn mixed_unavailable_entries_each_get_their_own_next_step() {
    let (code, out, err) = check(
        vec![
            unavailable("s", Engine::SwiftLint, "swiftlint not found on PATH"),
            unavailable("c", Engine::Clippy, CARGO_WHY),
            unavailable("t", Engine::Tessera, "Tessera's slop records are not defined yet"),
        ],
        false,
    );
    assert_eq!((code, err.as_str()), (3, ""));
    assert_eq!(
        out.split_inclusive('\n').last().unwrap(),
        "next: install swiftlint; clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks; Tessera's checks are not available yet (or run without those rules)\n"
    );
}

#[test]
fn json_next_explains_clippy_without_a_cargo_project() {
    let (code, out, err) = check(vec![unavailable("c", Engine::Clippy, CARGO_WHY)], true);
    assert_eq!((code, err.as_str()), (3, ""));
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        value["next"],
        "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)"
    );
}

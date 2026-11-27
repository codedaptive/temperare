//! ZUT for run_check.rs's exit-3 next line, using closure worlds without disk, git or PATH.

use super::{run_check, CheckWorld};
use crate::CheckArgs;
use report::{NotAvailable, Oid, Report};
use engines::{CheckedRules, NotAvailableCause, RuleRun};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};
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

/// A not-available report entry and the cause its rule's run carries.
fn unavailable(rule: &str, engine: Engine, why: &str, cause: NotAvailableCause) -> (NotAvailable, NotAvailableCause) {
    (NotAvailable { rule: rule.to_string(), engine, why: why.to_string() }, cause)
}

/// `cctao check` whose run gives each entry's rule not available with its cause, and whose report lists the entries.
fn check(entries: Vec<(NotAvailable, NotAvailableCause)>, json: bool) -> (u8, String, String) {
    let run_rules = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
        let runs = entries
            .iter()
            .map(|(na, cause)| {
                let outcome = RuleOutcome::NotAvailable { rule: na.rule.clone(), engine: na.engine, why: na.why.clone() };
                RuleRun::new(na.rule.clone(), na.engine, outcome, None).with_cause(*cause)
            })
            .collect();
        CheckedRules { runs, engines: vec![] }
    };
    let build_report = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>, _: Vec<(String, String)>| {
        Ok(Report { not_available: entries.iter().map(|(na, _)| na.clone()).collect(), ..Default::default() })
    };
    let world = CheckWorld {
        toplevel: &|_| Ok(PathBuf::from("/repo")),
        exists: &|_| true,
        read: &|path| {
            assert_eq!(path, Path::new("/repo/tao.toml"));
            Ok(RuleFile::parse(TEXT, Path::new("/repo")).unwrap())
        },
        rev_parse: &|_, _| unreachable!("rev_parse"),
        run_rules: &run_rules,
        availability: &|_, _| engines::Availability::Available,
        build_report: &build_report,
        home: &|| None,
        now: &|| 0,
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
    let (code, out, err) = check(vec![unavailable("c", Engine::Clippy, CARGO_WHY, NotAvailableCause::NoCargoProject)], false);
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
            unavailable("s", Engine::SwiftLint, "swiftlint not found on PATH", NotAvailableCause::EngineNotOnPath),
            unavailable("c", Engine::Clippy, CARGO_WHY, NotAvailableCause::NoCargoProject),
            unavailable("t", Engine::Tessera, "Tessera's slop records are not defined yet", NotAvailableCause::Other),
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
    let (code, out, err) = check(vec![unavailable("c", Engine::Clippy, CARGO_WHY, NotAvailableCause::NoCargoProject)], true);
    assert_eq!((code, err.as_str()), (3, ""));
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        value["next"],
        "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)"
    );
}

//! ZUT for run_rules_counted_with.rs's `stopped`: a check the engine said it does not know becomes `NotAvailable`
//! with a why naming the engine, its version from the versions given (`(version unknown)` without one) and the check;
//! a check not in temperare's list of its engine's codes becomes `NotAvailable` with the engine and its version before
//! what it said; `NotOnPath` and `NotAvailable` become `NotAvailable` with their why; `NoFiles` is no findings and
//! `Failed` keeps its kind. No engine runs in these. The unknown-check path is that of clippy, swiftlint and ruff, the
//! engines that can say a check is unknown: one test reaches it through `engine_hits` with swiftlint's output on a
//! fake host, the others hand `stopped` a made-up stop for ruff and ast-grep. ast-grep has no such path itself: its
//! check is the repository's YAML, and a YAML ast-grep rejects stays `Failed`.

use crate::engine_hits::engine_hits;
use crate::host::Host;
use crate::run_rules_counted_with::stopped;
use crate::types::{EngineVersion, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Rule, RuleFile, RuleOutcome};
use std::cell::Cell;
use std::path::Path;

const FOLDER: &str = "/no/such/engines-zut-folder";

fn rule(engine: &str, check: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n");
    RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

#[test]
fn a_swiftlint_rule_swiftlint_does_not_know_is_not_available_naming_engine_and_check() {
    let rule = rule("swiftlint", "no_such_rule");
    let run = |_: &Cmd| -> Result<Output, ProcError> {
        Ok(Output {
            code: Some(0),
            stdout: b"[\n\n]".to_vec(),
            stderr: b"warning: 'no_such_rule' is not a valid rule identifier\nValid rule identifiers:\nforce_cast\n".to_vec(),
        })
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None), base: None };
    let stop = engine_hits(&rule, Ok(Path::new(FOLDER)), &host).unwrap_err();
    assert_eq!(stopped(&rule, stop, &[]), RuleOutcome::NotAvailable {
        rule: "r".to_string(),
        engine: Engine::SwiftLint,
        why: "swiftlint (version unknown) does not know the check `no_such_rule`: warning: 'no_such_rule' is not a valid rule identifier".to_string(),
    });
}

// The stop is made by hand: no ast-grep run gives `UnknownCheck`. It shows only that `stopped` names an engine by
// its rule file name ("ast-grep"), not its enum name.
#[test]
fn an_unknown_check_names_the_engine_by_its_rule_file_name() {
    let rule = rule("ast-grep", "rules/x.yml");
    let got = stopped(&rule, Stop::UnknownCheck { said: "no such rule".to_string() }, &[]);
    assert_eq!(got, RuleOutcome::NotAvailable {
        rule: "r".to_string(),
        engine: Engine::AstGrep,
        why: "ast-grep (version unknown) does not know the check `rules/x.yml`: no such rule".to_string(),
    });
}

#[test]
fn not_available_and_failed_keep_their_kind() {
    let rule = rule("ruff", "E711");
    let not_on_path = stopped(&rule, Stop::NotOnPath { program: "ruff".to_string() }, &[]);
    let want = RuleOutcome::NotAvailable { rule: "r".into(), engine: Engine::Ruff, why: "ruff not found on PATH".into() };
    assert_eq!(not_on_path, want);
    let not_available = stopped(&rule, Stop::NotAvailable { why: "no Cargo.toml".to_string() }, &[]);
    let want = RuleOutcome::NotAvailable { rule: "r".into(), engine: Engine::Ruff, why: "no Cargo.toml".into() };
    assert_eq!(not_available, want);
    let failed = stopped(&rule, Stop::Failed { code: Some(2), stderr: "boom".to_string() }, &[]);
    assert_eq!(failed, RuleOutcome::Failed { rule: "r".into(), engine: Engine::Ruff, code: Some(2), stderr: "boom".into() });
}

#[test]
fn an_unknown_check_names_its_engines_version_and_no_other_engines() {
    let rule = rule("ruff", "ZZZ999");
    let versions = [
        EngineVersion { engine: Engine::SwiftLint, version: Some("0.59.1".to_string()) },
        EngineVersion { engine: Engine::Ruff, version: Some("1.2.3".to_string()) },
    ];
    let got = stopped(&rule, Stop::UnknownCheck { said: "Unknown rule selector".to_string() }, &versions);
    assert_eq!(got, RuleOutcome::NotAvailable {
        rule: "r".to_string(),
        engine: Engine::Ruff,
        why: "ruff 1.2.3 does not know the check `ZZZ999`: Unknown rule selector".to_string(),
    });
}

#[test]
fn an_engine_listed_with_no_version_is_version_unknown() {
    let rule = rule("ruff", "ZZZ999");
    let versions = [EngineVersion { engine: Engine::Ruff, version: None }];
    let got = stopped(&rule, Stop::UnknownCheck { said: "no".to_string() }, &versions);
    assert_eq!(got, RuleOutcome::NotAvailable {
        rule: "r".to_string(),
        engine: Engine::Ruff,
        why: "ruff (version unknown) does not know the check `ZZZ999`: no".to_string(),
    });
}

#[test]
fn no_files_is_no_findings() {
    let rule = rule("shellcheck", "SC2086");
    let got = stopped(&rule, Stop::NoFiles { why: "no file its engine checks".to_string() }, &[]);
    assert_eq!(got, RuleOutcome::Findings(vec![]));
}

fn not_listed() -> Stop {
    Stop::NotListed { said: "SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)".to_string() }
}

fn not_available(why: &str) -> RuleOutcome {
    RuleOutcome::NotAvailable { rule: "r".to_string(), engine: Engine::ShellCheck, why: why.to_string() }
}

#[test]
fn a_check_not_listed_names_its_engines_version_before_what_it_said() {
    let versions = [EngineVersion { engine: Engine::ShellCheck, version: Some("0.11.0".to_string()) }];
    let got = stopped(&rule("shellcheck", "SC9999"), not_listed(), &versions);
    let why = "shellcheck 0.11.0: SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, not_available(why));
}

#[test]
fn a_check_not_listed_with_no_version_read_is_version_unknown() {
    let versions = [EngineVersion { engine: Engine::ShellCheck, version: None }];
    let got = stopped(&rule("shellcheck", "SC9999"), not_listed(), &versions);
    let why = "shellcheck (version unknown): SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, not_available(why));
}

#[test]
fn a_check_not_listed_never_takes_another_engines_version() {
    let versions = [EngineVersion { engine: Engine::Ruff, version: Some("1.2.3".to_string()) }];
    let got = stopped(&rule("shellcheck", "SC9999"), not_listed(), &versions);
    let why = "shellcheck (version unknown): SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, not_available(why));
}

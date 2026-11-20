//! ZUT for availability.rs: engine program lookups, including both Clippy programs and ShellCheck, exact absence
//! reasons, Tessera never available (settled S-20), and an engine with no adapter in this build (Pylint) not
//! available whatever is on PATH, both from `availability_from` and as a rule's outcome from `run_rules_versioned`.

use crate::availability::availability_from;
use crate::types::{Availability, RuleRun};
use rules::{Engine, RuleFile, RuleOutcome};
use std::cell::RefCell;
use std::path::Path;

fn not_available(why: &str) -> Availability {
    Availability::NotAvailable { why: why.to_string() }
}

#[test]
fn each_absent_engine_says_why() {
    assert_eq!(availability_from(Engine::Clippy, |_| false), not_available("cargo-clippy not found on PATH"));
    assert_eq!(availability_from(Engine::SwiftLint, |_| false), not_available("swiftlint not found on PATH"));
    assert_eq!(availability_from(Engine::Ruff, |_| false), not_available("ruff not found on PATH"));
    assert_eq!(availability_from(Engine::AstGrep, |_| false), not_available("ast-grep not found on PATH"));
    assert_eq!(availability_from(Engine::ShellCheck, |_| false), not_available("shellcheck not found on PATH"));
}

#[test]
fn tessera_is_not_available_whatever_is_on_path() {
    let why = not_available("Tessera's slop records are not defined yet");
    assert_eq!(availability_from(Engine::Tessera, |_| false), why);
    assert_eq!(availability_from(Engine::Tessera, |_| true), why);
}

#[test]
fn each_present_engine_is_available() {
    for engine in [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep, Engine::ShellCheck] {
        assert_eq!(availability_from(engine, |_| true), Availability::Available, "{engine:?}");
    }
}

#[test]
fn each_engine_looks_up_its_program() {
    let cases = [
        (Engine::Clippy, "cargo-clippy"),
        (Engine::SwiftLint, "swiftlint"),
        (Engine::Ruff, "ruff"),
        (Engine::AstGrep, "ast-grep"),
        (Engine::ShellCheck, "shellcheck"),
    ];
    for (engine, program) in cases {
        assert_eq!(asked_for(engine), vec![program.to_string()], "{engine:?}");
    }
}

#[test]
fn tessera_looks_up_nothing() {
    assert_eq!(asked_for(Engine::Tessera), Vec::<String>::new());
}

/// The names `availability_from` looks up on PATH for `engine`, in order.
fn asked_for(engine: Engine) -> Vec<String> {
    let asked = RefCell::new(Vec::new());
    availability_from(engine, |name| {
        asked.borrow_mut().push(name.to_string());
        false
    });
    asked.into_inner()
}

#[test]
fn clippy_with_cargo_but_no_cargo_clippy_is_not_available() {
    let got = availability_from(Engine::Clippy, |name| name == "cargo");
    assert_eq!(got, not_available("cargo-clippy not found on PATH"));
}

#[test]
fn clippy_without_driver_is_not_available() {
    assert_eq!(
        availability_from(Engine::Clippy, |name| name == "cargo-clippy"),
        not_available("clippy-driver not found on PATH")
    );
}

#[test]
fn clippy_looks_up_both_programs_in_order_when_present() {
    let asked = RefCell::new(Vec::new());
    availability_from(Engine::Clippy, |name| {
        asked.borrow_mut().push(name.to_string());
        true
    });
    assert_eq!(asked.into_inner(), vec!["cargo-clippy", "clippy-driver"]);
}

#[test]
fn pylint_has_no_adapter_whatever_is_on_path_and_looks_up_nothing() {
    let why = not_available("no adapter for pylint in this build");
    assert_eq!(availability_from(Engine::Pylint, |_| true), why);
    assert_eq!(availability_from(Engine::Pylint, |_| false), why);
    assert_eq!(asked_for(Engine::Pylint), Vec::<String>::new());
}

#[test]
fn a_pylint_rule_run_is_not_available_with_no_adapter_and_counts_nothing() {
    // The repository does not exist: availability is decided whatever the rule file's folder gives.
    let repo = Path::new("/no/such/engines-zut-repo");
    let text = "version = 1\n[[rule]]\nid = \"doc\"\nengine = \"pylint\"\ncheck = \"C0114\"\nfix = \"f\"\n";
    let file = RuleFile::parse(text, repo).unwrap();
    let runs = crate::run_rules_versioned(&file, repo, &[]).runs;
    let want = RuleRun::new(
        "doc".to_string(),
        Engine::Pylint,
        RuleOutcome::NotAvailable {
            rule: "doc".to_string(),
            engine: Engine::Pylint,
            why: "no adapter for pylint in this build".to_string(),
        },
        None,
    );
    assert_eq!(runs, vec![want]);
}

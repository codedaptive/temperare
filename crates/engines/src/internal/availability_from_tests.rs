//! Unit test for availability_from.rs: engine program lookups, including both Clippy programs and ShellCheck, exact absence
//! reasons, the Node engines' `node`, and Tessera never available, from `availability_from`.

use crate::availability_from::availability_from;
use crate::types::Availability;
use rules::Engine;
use std::cell::RefCell;

fn not_available(why: &str) -> Availability {
    Availability::NotAvailable { why: why.to_string() }
}

#[test]
fn each_absent_engine_says_why() {
    assert_eq!(availability_from(Engine::Clippy, None, |_| false), not_available("cargo-clippy not found on PATH"));
    assert_eq!(availability_from(Engine::SwiftLint, None, |_| false), not_available("swiftlint not found on PATH"));
    assert_eq!(availability_from(Engine::Ruff, None, |_| false), not_available("ruff not found on PATH"));
    assert_eq!(availability_from(Engine::AstGrep, None, |_| false), not_available("ast-grep not found on PATH"));
    assert_eq!(availability_from(Engine::ShellCheck, None, |_| false), not_available("shellcheck not found on PATH"));
}

#[test]
fn node_engines_name_node_or_their_program() {
    let no_node = |name: &str| name != "node";
    let cases: [(&str, Engine, &dyn Fn(&str) -> bool, &str); 6] = [
        ("eslint, no node", Engine::Eslint, &no_node, "node not found on PATH"),
        ("stylelint, no node", Engine::Stylelint, &no_node, "node not found on PATH"),
        ("markdownlint, no node", Engine::Markdownlint, &no_node, "node not found on PATH"),
        ("eslint, nothing", Engine::Eslint, &|_| false, "eslint not found on PATH"),
        ("stylelint, nothing", Engine::Stylelint, &|_| false, "stylelint not found on PATH"),
        ("markdownlint, nothing", Engine::Markdownlint, &|_| false, "markdownlint-cli2 not found on PATH"),
    ];
    for (name, engine, on_path, why) in cases {
        assert_eq!(availability_from(engine, None, on_path), not_available(why), "{name}");
    }
}

#[test]
fn tessera_is_not_available_whatever_is_on_path() {
    let why = not_available("Tessera's slop records are not defined yet");
    assert_eq!(availability_from(Engine::Tessera, None, |_| false), why);
    assert_eq!(availability_from(Engine::Tessera, None, |_| true), why);
}

#[test]
fn each_present_engine_is_available() {
    for engine in [Engine::Clippy, Engine::SwiftLint, Engine::Ruff, Engine::AstGrep, Engine::ShellCheck] {
        assert_eq!(availability_from(engine, None, |_| true), Availability::Available, "{engine:?}");
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

/// The names `availability_from` looks up on PATH for `engine`, in order.
fn asked_for(engine: Engine) -> Vec<String> {
    let asked = RefCell::new(Vec::new());
    availability_from(engine, None, |name| {
        asked.borrow_mut().push(name.to_string());
        false
    });
    asked.into_inner()
}

#[test]
fn clippy_with_cargo_but_no_cargo_clippy_is_not_available() {
    let got = availability_from(Engine::Clippy, None, |name| name == "cargo");
    assert_eq!(got, not_available("cargo-clippy not found on PATH"));
}

#[test]
fn clippy_without_driver_is_not_available() {
    assert_eq!(
        availability_from(Engine::Clippy, None, |name| name == "cargo-clippy"),
        not_available("clippy-driver not found on PATH")
    );
}

#[test]
fn clippy_looks_up_both_programs_in_order_when_present() {
    let asked = RefCell::new(Vec::new());
    availability_from(Engine::Clippy, None, |name| {
        asked.borrow_mut().push(name.to_string());
        true
    });
    assert_eq!(asked.into_inner(), vec!["cargo-clippy", "clippy-driver"]);
}

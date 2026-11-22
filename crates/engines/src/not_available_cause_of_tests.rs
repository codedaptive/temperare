//! ZUT for mapping a stop to its not-available cause.

use crate::an_engine_with_no_adapter::an_engine_with_no_adapter;
use crate::not_available_cause_of::not_available_cause_of;
use crate::types::{NotAvailableCause, Stop};
use rules::Engine;

#[test]
fn each_stop_maps_to_its_cause() {
    let not_on_path = |program: &str| Stop::NotOnPath { program: program.into() };
    let engine = Some(NotAvailableCause::EngineNotOnPath);
    let helper = Some(NotAvailableCause::HelperNotOnPath);
    let cases = [
        ("ruff absent", Engine::Ruff, not_on_path("ruff"), engine),
        ("cargo-clippy absent", Engine::Clippy, not_on_path("cargo-clippy"), engine),
        ("clippy-driver absent", Engine::Clippy, not_on_path("clippy-driver"), engine),
        ("cargo absent under clippy", Engine::Clippy, not_on_path("cargo"), helper),
        ("git absent for shellcheck", Engine::ShellCheck, not_on_path("git"), helper),
        ("git absent for git", Engine::Git, not_on_path("git"), engine),
        ("unknown check", Engine::Ruff, Stop::UnknownCheck { said: "unknown".into() }, Some(NotAvailableCause::UnknownCheck)),
        (
            "no Cargo.toml for clippy",
            Engine::Clippy,
            Stop::NoCargoProject { why: "no Cargo.toml at or above the rule file's folder".into() },
            Some(NotAvailableCause::NoCargoProject),
        ),
        ("not available", Engine::Ruff, Stop::NotAvailable { why: "unavailable".into() }, Some(NotAvailableCause::Other)),
        ("not listed", Engine::ShellCheck, Stop::NotListed { said: "unlisted".into() }, Some(NotAvailableCause::Other)),
        ("failed", Engine::Ruff, Stop::Failed { code: Some(2), stderr: "failed".into() }, None),
        ("no files", Engine::ShellCheck, Stop::NoFiles { why: "no files".into() }, None),
        ("a program run for an engine with no entry", an_engine_with_no_adapter(), not_on_path("x"), helper),
    ];
    for (name, engine, stop, expected) in cases {
        assert_eq!(not_available_cause_of(&stop, engine), expected, "{name}");
    }
}

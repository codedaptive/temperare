//! Unit test for unavailable.rs: the `Stop` of an engine that cannot be run, from a supplied PATH lookup.

use crate::types::Stop;
use crate::unavailable::unavailable;
use rules::Engine;
use std::cell::RefCell;

#[test]
fn unavailable_reports_engines_and_programs() {
    let cases = [
        ("never", Engine::Tessera, vec![], vec![], Some(Stop::NotAvailable { why: "Tessera's slop records are not defined yet".into() })),
        ("ruff absent", Engine::Ruff, vec![], vec!["ruff"], Some(Stop::NotOnPath { program: "ruff".into() })),
        ("clippy driver absent", Engine::Clippy, vec!["cargo-clippy"], vec!["cargo-clippy", "clippy-driver"], Some(Stop::NotOnPath { program: "clippy-driver".into() })),
        ("ruff present", Engine::Ruff, vec!["ruff"], vec!["ruff"], None),
    ];
    for (name, engine, present, expected_lookups, expected) in cases {
        let lookups = RefCell::new(Vec::new());
        let got = unavailable(engine, None, |program| {
            lookups.borrow_mut().push(program.to_owned());
            present.contains(&program)
        });
        assert_eq!(got, expected, "{name}");
        assert_eq!(lookups.into_inner(), expected_lookups, "{name}");
    }
}

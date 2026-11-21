//! Unit tests for hits_tessera.rs: every tessera rule is not available with the standard text, and nothing is run or looked up.

use super::{hits_tessera, TESSERA};
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::Cell;
use std::path::Path;

#[test]
fn a_tessera_rule_is_not_available_and_touches_no_program() {
    let text = "version = 1\n[[rule]]\nid = \"t\"\nengine = \"tessera\"\ncheck = \"slop\"\nfix = \"f\"\n";
    let file = RuleFile::parse(text, Path::new("/repo")).unwrap();
    let run = |_: &Cmd| -> Result<Output, ProcError> { panic!("hits_tessera runs nothing") };
    let on_path = |_: &str| -> bool { panic!("hits_tessera looks nothing up on PATH") };
    let got = hits_tessera(&TESSERA, &file.rules[0], Path::new("/repo"), &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None) });
    assert_eq!(got, Err(Stop::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() }));
}

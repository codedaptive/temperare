//! ZUT for cargo_audit_entry.rs: the fields of `CARGO_AUDIT`, its version probe read on cargo-audit 0.23.0's
//! `--version` output, and its refusal and reader wired to cargo-audit's. Nothing runs.

use super::CARGO_AUDIT;
use crate::entry::{Params, Pick};
use procio::Output;
use rules::Engine;
use std::path::Path;

fn out(code: i32, stdout: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() }
}

#[test]
fn cargo_audit_entry_fields() {
    assert_eq!((CARGO_AUDIT.name, CARGO_AUDIT.engine, CARGO_AUDIT.program), ("cargo-audit", Engine::CargoAudit, "cargo-audit"));
    assert_eq!(CARGO_AUDIT.needs, ["cargo-audit"]);
    assert_eq!(CARGO_AUDIT.never, None);
    assert_eq!(CARGO_AUDIT.args, ["audit", "--no-fetch", "--no-yanked", "--json"]);
    assert_eq!(CARGO_AUDIT.pick, Pick::None);
    assert!(matches!(CARGO_AUDIT.params, Params::Args(args) if args.is_empty()), "{:?}", CARGO_AUDIT.params);
    assert_eq!(CARGO_AUDIT.config, None);
    assert!(CARGO_AUDIT.after.is_empty());

    let probe = CARGO_AUDIT.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    assert_eq!((probe.read)(&out(0, b"cargo-audit 0.23.0\n")), Some("0.23.0".to_string()));

    let refusals: [(&str, bool); 2] = [("yanked", true), ("RUSTSEC-2021-0003", false)];
    for (check, refused) in refusals {
        assert_eq!((CARGO_AUDIT.refuse)(check).is_some(), refused, "{check}");
    }

    let clean = br#"{"settings": {"target_arch": [], "target_os": [], "severity": null, "ignore": [], "informational_warnings": ["unmaintained", "unsound", "notice"]}, "vulnerabilities": {"found": false, "count": 0, "list": []}, "warnings": {}}"#;
    assert_eq!((CARGO_AUDIT.read)(&out(0, clean), "all", Path::new("/w"), Path::new("/w")), Ok(vec![]));
}

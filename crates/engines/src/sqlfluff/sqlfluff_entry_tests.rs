//! Unit test for sqlfluff_entry.rs: the fields of `SQLFLUFF`, its version probe read on sqlfluff 4.5.0's `--version` output,
//! and its refusal, unknown-rule and reader wired to sqlfluff's. Nothing runs.

use super::SQLFLUFF;
use crate::entry::{Params, Pick};
use crate::types::Hit;
use procio::Output;
use rules::{Engine, RelPath};
use std::path::Path;

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn sqlfluff_entry_fields() {
    assert_eq!((SQLFLUFF.name, SQLFLUFF.engine, SQLFLUFF.program), ("sqlfluff", Engine::Sqlfluff, "sqlfluff"));
    assert_eq!(SQLFLUFF.needs, ["sqlfluff"]);
    assert_eq!(SQLFLUFF.never, None);
    assert_eq!(&SQLFLUFF.args[..3], ["lint", "--format", "json"]);
    for switch in ["--disable-noqa", "--disregard-sqlfluffignores", "--exclude-rules=", "--ignore="] {
        assert!(SQLFLUFF.args.contains(&switch), "{switch}");
    }
    assert_eq!(SQLFLUFF.pick, Pick::Arg("--rules="));
    assert!(matches!(SQLFLUFF.params, Params::Args(args) if args.is_empty()), "{:?}", SQLFLUFF.params);
    assert_eq!(SQLFLUFF.config, Some("--config="));
    assert!(SQLFLUFF.after.is_empty());

    let probe = SQLFLUFF.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    assert_eq!((probe.read)(&out(0, b"sqlfluff, version 4.5.0\n", b"")), Some("4.5.0".to_string()));

    assert!((SQLFLUFF.refuse)("LT01,LT02").is_some(), "a comma list is refused");

    let unknown = out(0, b"", include_bytes!("fixtures/unknown_rule.txt"));
    assert_eq!(
        (SQLFLUFF.unknown)(&unknown, "NOPE99").as_deref(),
        Some("WARNING    Tried to allowlist unknown rule references: ['NOPE99']")
    );

    let finding = out(0, include_bytes!("fixtures/warnings.json"), b"");
    let hit = Hit {
        path: RelPath::new("a.sql"),
        line: Some(1),
        col: Some(9),
        what: "LT01: Expected only single space before 'FROM' keyword. Found '  '.".to_string(),
    };
    assert_eq!((SQLFLUFF.read)(&finding, "LT01", Path::new("/w"), Path::new("/w")), Ok(vec![hit]));
}

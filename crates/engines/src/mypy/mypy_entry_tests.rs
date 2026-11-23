//! ZUT for mypy_entry.rs: the fields of `MYPY`, its version probe read on mypy 2.4.1's `--version` output, and its
//! refusal, unknown-code and reader wired to mypy's. Nothing runs.

use super::MYPY;
use crate::entry::{Params, Pick};
use crate::types::Hit;
use procio::Output;
use rules::{Engine, RelPath};
use std::path::Path;

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn mypy_entry_fields() {
    assert_eq!((MYPY.name, MYPY.engine, MYPY.program), ("mypy", Engine::Mypy, "mypy"));
    assert_eq!(MYPY.needs, ["mypy"]);
    assert_eq!(MYPY.never, None);
    assert_eq!(MYPY.args, ["--output=json", "--cache-dir=/dev/null"]);
    assert_eq!(MYPY.pick, Pick::Arg("--enable-error-code="));
    assert!(matches!(MYPY.params, Params::Args(args) if args.is_empty()), "{:?}", MYPY.params);
    assert_eq!(MYPY.config, Some("--config-file="));
    assert!(MYPY.after.is_empty());

    let probe = MYPY.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    assert_eq!((probe.read)(&out(0, b"mypy 2.4.1 (compiled: yes)\n", b"")), Some("2.4.1".to_string()));

    let refusals: [(&str, bool); 2] = [("arg-type,misc", true), ("arg-type", false)];
    for (check, refused) in refusals {
        assert_eq!((MYPY.refuse)(check).is_some(), refused, "{check}");
    }

    let unknown: [(&[u8], Option<&str>); 2] = [
        (b"mypy: error: Invalid error code(s): nonsense-code\n", Some("mypy: error: Invalid error code(s): nonsense-code")),
        (b"", None),
    ];
    for (stderr, said) in unknown {
        assert_eq!((MYPY.unknown)(&out(2, b"", stderr), "nonsense-code").as_deref(), said, "{stderr:?}");
    }

    let finding = out(1, include_bytes!("fixtures/arg_type.jsonl"), b"");
    let hit = Hit {
        path: RelPath::new("a.py"),
        line: Some(8),
        col: Some(3),
        what: "Argument 1 to \"f\" has incompatible type \"str\"; expected \"int\"".to_string(),
    };
    assert_eq!((MYPY.read)(&finding, "arg-type", Path::new("/w"), Path::new("/w")), Ok(vec![hit]));
}

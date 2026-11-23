//! ZUT for parse_dart.rs on dart 3.14.1's real machine output (fixtures/machine.txt, made as fixtures/commands.txt
//! says): the rule's code is kept, case aside, with escaped `|` and `\` read back; other exits and malformed lines fail.

use super::parse_dart;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const T: &str = "/private/var/folders/zc/zl4w__wx2xdfmd5lps9dp2000000gn/T/dartfix.0bPKGrAnVf";

#[test]
fn parse_dart_keeps_the_checks_diagnostics_and_rejects_invalid_output() {
    let hit = |path: &str, line, col, what: &str| Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() };
    let unused = hit("lib/a.dart", 2, 7, "The value of the local variable 'unused' isn't used.");
    let spare = hit("lib/pipe|name.dart", 4, 7, "The value of the local variable 'spare' isn't used.");
    let fixture = include_bytes!("fixtures/machine.txt");
    let info = b"INFO|LINT|PREFER_SINGLE_QUOTES|lib/a.dart|2|9|3|Unnecessary use of double quotes.\n";
    let rows: [(&str, Option<i32>, &[u8], &[u8], &str, String, Option<Vec<Hit>>); 16] = [
        ("warning", Some(3), fixture, b"", "unused_local_variable", T.into(), Some(vec![unused.clone(), spare])),
        ("lint", Some(3), fixture, b"", "prefer_single_quotes", T.into(), Some(vec![hit("lib/a.dart", 3, 9, "Unnecessary use of double quotes.")])),
        ("escapes", Some(3), fixture, b"", "uri_does_not_exist", T.into(), Some(vec![hit("lib/pipe|name.dart", 1, 8, "Target of URI doesn't exist: 'no|such\\file.dart'.")])),
        ("upper-case check", Some(3), fixture, b"", "UNUSED_LOCAL_VARIABLE", T.into(), Some(vec![unused, hit("lib/pipe|name.dart", 4, 7, "The value of the local variable 'spare' isn't used.")])),
        ("subfolder", Some(3), fixture, b"", "prefer_single_quotes", format!("{T}/lib"), Some(vec![hit("a.dart", 3, 9, "Unnecessary use of double quotes.")])),
        ("outside folder", Some(3), fixture, b"", "unused_local_variable", format!("{T}/bin"), Some(vec![])),
        ("code not printed", Some(3), fixture, b"", "dead_code", T.into(), Some(vec![])),
        ("clean", Some(0), b"", b"", "unused_local_variable", T.into(), Some(vec![])),
        ("infos only, relative path", Some(0), info, b"", "prefer_single_quotes", T.into(), Some(vec![hit("lib/a.dart", 2, 9, "Unnecessary use of double quotes.")])),
        ("exit 64", Some(64), b"", b"Could not find a command named \"analyse\".", "unused_local_variable", T.into(), None),
        ("exit 1", Some(1), info, b"", "prefer_single_quotes", T.into(), None),
        ("exit 4", Some(4), fixture, b"", "unused_local_variable", T.into(), None),
        ("signal", None, b"", b"", "unused_local_variable", T.into(), None),
        ("seven fields", Some(2), b"WARNING|STATIC_WARNING|UNUSED_LOCAL_VARIABLE|lib/a.dart|2|7|6\n", b"", "dead_code", T.into(), None),
        ("escaped separator", Some(2), b"WARNING|STATIC_WARNING|UNUSED_LOCAL_VARIABLE|lib/a.dart|2|7\\|6|msg\n", b"", "dead_code", T.into(), None),
        ("line not a number", Some(2), b"WARNING|STATIC_WARNING|UNUSED_LOCAL_VARIABLE|lib/a.dart|two|7|6|msg\n", b"", "dead_code", T.into(), None),
    ];
    let mut failed = Vec::new();
    for (name, code, stdout, stderr, check, folder, want) in rows {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            let got = parse_dart(&out, check, Path::new(T), Path::new(&folder));
            match want { Some(hits) => assert_eq!(got.unwrap(), hits, "{name}"), None => assert!(got.is_err(), "{name}: {got:?}") }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

//! Unit test for parse_yamllint.rs on yamllint 1.38.0's real parsable output (fixtures/findings.txt, syntax.txt,
//! no_such_rule.txt; fixtures/fixtures.txt says how each was made), and on lines and exits it must refuse.

use super::parse_yamllint;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

const TRACEBACK: &[u8] = b"Traceback (most recent call last):\nFileNotFoundError: [Errno 2] No such file or directory: '/work/y/missing.yaml'\n";

#[test]
fn parse_yamllint_selects_the_check_and_rejects_invalid_output() {
    let hit = |path: &str, line, col, what: &str| Hit { path: RelPath::new(path), line: Some(line), col: Some(col), what: what.to_string() };
    // A colon is not a Windows file-name character, so `c:3.yaml` is a drive-relative path there and falls outside the folder.
    let with_colon = |mut hits: Vec<Hit>, colon: Hit| { if cfg!(unix) { hits.push(colon); } hits };
    let truthy = "truthy value should be one of [false, true]";
    let start = "missing document start \"---\"";
    let findings: &[u8] = include_bytes!("fixtures/findings.txt");
    let syntax: &[u8] = include_bytes!("fixtures/syntax.txt");
    let no_such: &[u8] = include_bytes!("fixtures/no_such_rule.txt");
    let warning_only: &[u8] = b"b.yml:2:8: [warning] truthy value should be one of [false, true] (truthy)\n";
    let rows: [(&str, Option<i32>, &[u8], &[u8], &str, &str, &str, Result<Vec<Hit>, &[&str]>); 18] = [
        ("truthy, warnings", Some(1), findings, b"", "truthy", "/work/y", "/work/y",
            Ok(with_colon(vec![hit("a.yaml", 1, 6, truthy), hit("b.yml", 2, 8, truthy)], hit("c:3.yaml", 1, 7, truthy)))),
        ("document-start, a path holding a colon", Some(1), findings, b"", "document-start", "/work/y", "/work/y",
            Ok(with_colon(vec![hit("a.yaml", 1, 1, start)], hit("c:3.yaml", 1, 1, start)))),
        ("line-length, an error", Some(1), findings, b"", "line-length", "/work/y", "/work/y",
            Ok(vec![hit("a.yaml", 2, 81, "line too long (87 > 80 characters)")])),
        ("a check not in the output", Some(1), findings, b"", "indentation", "/work/y", "/work/y", Ok(vec![])),
        ("outside folder", Some(1), findings, b"", "truthy", "/work/y", "/work/y/sub", Ok(vec![])),
        ("parent folder", Some(1), b"y/a.yaml:1:6: [warning] truthy value should be one of [false, true] (truthy)\n", b"", "truthy", "/work", "/work/y",
            Ok(vec![hit("a.yaml", 1, 6, truthy)])),
        ("warnings only exit 0", Some(0), warning_only, b"", "truthy", "/work/y", "/work/y", Ok(vec![hit("b.yml", 2, 8, truthy)])),
        ("clean", Some(0), b"", b"", "truthy", "/work/y", "/work/y", Ok(vec![])),
        ("syntax, another check", Some(1), syntax, b"", "document-start", "/work/y", "/work/y", Err(&["bad.yaml", "line 2"])),
        ("syntax, its own check", Some(1), syntax, b"", "syntax", "/work/y", "/work/y", Err(&["bad.yaml", "line 2"])),
        ("no such rule, exit 255", Some(255), b"", no_such, "nosuch", "/work/y", "/work/y", Err(&["exited 255, not 0 or 1"])),
        ("exit 2", Some(2), b"", b"", "truthy", "/work/y", "/work/y", Err(&["exited 2, not 0 or 1"])),
        ("signal", None, b"", b"", "truthy", "/work/y", "/work/y", Err(&["signal"])),
        ("traceback, exit 1", Some(1), b"", TRACEBACK, "truthy", "/work/y", "/work/y", Err(&["missing.yaml"])),
        ("stderr beside findings", Some(1), findings, b"a warning\n", "truthy", "/work/y", "/work/y", Err(&["a warning"])),
        ("another level", Some(1), b"a.yaml:1:6: [info] truthy value (truthy)\n", b"", "truthy", "/work/y", "/work/y",
            Err(&["a.yaml:1:6: [info] truthy value (truthy)"])),
        ("no rule", Some(1), b"a.yaml:1:6: [warning] truthy value\n", b"", "truthy", "/work/y", "/work/y",
            Err(&["a.yaml:1:6: [warning] truthy value"])),
        ("not a finding", Some(1), b"something else\n", b"", "truthy", "/work/y", "/work/y", Err(&["something else"])),
    ];
    let mut failed = Vec::new();
    for (name, code, stdout, stderr, check, cwd, folder, want) in rows {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            let got = parse_yamllint(&out, check, Path::new(cwd), Path::new(folder));
            match want {
                Ok(hits) => assert_eq!(got.unwrap(), hits, "{name}"),
                Err(tokens) => {
                    let why = got.expect_err(name).why;
                    assert!(tokens.iter().all(|token| why.contains(token)), "{name}: {why}");
                }
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

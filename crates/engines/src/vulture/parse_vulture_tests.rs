use super::parse_vulture;
use crate::types::Hit;
use procio::Output;
use rules::RelPath;
use std::path::Path;

#[test]
fn parse_vulture_selects_kinds_and_rejects_invalid_output() {
    let hit = |line, what: &str| Hit { path: RelPath::new("example.py"), line: Some(line), col: None, what: what.to_string() };
    let fixture = include_bytes!("fixtures/every_kind.txt");
    let rows: [(&str, Option<i32>, &[u8], &[u8], &str, &str, Option<Vec<Hit>>); 16] = [
        ("attribute", Some(3), fixture, b"", "V101", "/work/py", Some(vec![hit(5, "unused attribute 'abandoned'")])),
        ("class", Some(3), fixture, b"", "V102", "/work/py", Some(vec![hit(3, "unused class 'Ghost'")])),
        ("function", Some(3), fixture, b"", "V103", "/work/py", Some(vec![hit(14, "unused function 'dead'")])),
        ("import", Some(3), fixture, b"", "V104", "/work/py", Some(vec![hit(1, "unused import 'math'")])),
        ("method", Some(3), fixture, b"", "V105", "/work/py", Some(vec![hit(7, "unused method 'dormant'")])),
        ("property", Some(3), fixture, b"", "V106", "/work/py", Some(vec![hit(10, "unused property 'forgotten'")])),
        ("variables", Some(3), fixture, b"", "V107", "/work/py", Some(vec![hit(14, "unused variable 'argument'"), hit(15, "unused variable 'unused_local'")])),
        ("unreachable", Some(3), fixture, b"", "V201", "/work/py", Some(vec![hit(17, "unreachable code after 'return'"), hit(19, "unsatisfiable 'if' condition")])),
        ("outside folder", Some(3), fixture, b"", "V103", "/work/py/sub", Some(vec![])),
        ("clean", Some(0), b"", b"", "V103", "/work/py", Some(vec![])),
        ("stderr", Some(3), fixture, b"syntax error", "V103", "/work/py", None),
        ("exit 1", Some(1), b"", b"", "V103", "/work/py", None),
        ("exit 2", Some(2), b"", b"", "V103", "/work/py", None),
        ("invalid syntax line", Some(3), b"example.py:1: invalid syntax at \"def f(:\"\n", b"", "V103", "/work/py", None),
        ("unknown message", Some(3), b"example.py:1: something unknown (90% confidence)\n", b"", "V103", "/work/py", None),
        ("parent folder", Some(3), b"py/example.py:14: unused function 'dead' (60% confidence)\n", b"", "V103", "/work/py", Some(vec![hit(14, "unused function 'dead'")])),
    ];
    let mut failed = Vec::new();
    for (name, code, stdout, stderr, check, folder, want) in rows {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            let cwd = if name == "parent folder" { "/work" } else { "/work/py" };
            let got = parse_vulture(&out, check, Path::new(cwd), Path::new(folder));
            match want { Some(hits) => assert_eq!(got.unwrap(), hits, "{name}"), None => assert!(got.is_err(), "{name}: {got:?}") }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

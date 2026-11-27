//! Unit test for no_dialect.rs: sqlfluff 4.5.0's no-dialect stderr (fixtures/no_dialect.txt, fixtures/fixtures.txt) gives
//! the why naming the missing setting; other stderr gives None.

use super::no_dialect;
use procio::Output;

#[test]
fn no_dialect_cases() {
    let why = "no dialect is set in the repository's sqlfluff config: set `dialect` under `[sqlfluff]` in its .sqlfluff";
    let rows: [(&str, i32, &[u8], Option<&str>); 3] = [
        ("the saved no-dialect run", 2, include_bytes!("fixtures/no_dialect.txt"), Some(why)),
        ("an unknown-rule warning", 0, include_bytes!("fixtures/unknown_rule.txt"), None),
        ("empty stderr", 1, b"", None),
    ];
    for (label, code, stderr, expected) in rows {
        let out = Output { code: Some(code), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(no_dialect(&out).as_deref(), expected, "{label}");
    }
}

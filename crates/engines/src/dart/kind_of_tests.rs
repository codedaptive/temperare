//! Unit test for kind_of.rs on dart 3.14.1's real probe output (fixtures/probe_*.txt, made as fixtures/commands.txt says):
//! an unknown code is UnknownCheck with dart's words, a code that is not a lint a Diagnostic, a lint a Lint, and an
//! exit other than 0 or 2 Failed.

use super::{kind_of, CheckKind};
use crate::types::Stop;
use procio::Output;

#[test]
fn kind_of_cases() {
    let unknown = Err(Stop::UnknownCheck { said: "'no_such_code_xyz' isn't a recognized diagnostic code.".into() });
    let failed_stop = Err(Stop::Failed { code: Some(64), stderr: "dart's probe of `x` failed: Could not find a command named \"analyze\".\n".into() });
    let rows: [(&str, Option<i32>, &str, &str, &str, Result<CheckKind, Stop>); 5] = [
        ("unknown code", Some(2), include_str!("fixtures/probe_unknown.txt"), "", "no_such_code_xyz", unknown),
        ("lint", Some(0), include_str!("fixtures/probe_lint.txt"), "", "prefer_single_quotes", Ok(CheckKind::Lint)),
        ("diagnostic", Some(2), include_str!("fixtures/probe_diagnostic.txt"), "", "unused_local_variable",
            Ok(CheckKind::Diagnostic)),
        ("other exit", Some(64), "", "Could not find a command named \"analyze\".\n", "x", failed_stop),
        ("killed", None, "", "", "x", Err(Stop::Failed { code: None, stderr: "dart's probe of `x` failed: ".into() })),
    ];
    let mut failed = Vec::new();
    for (name, code, stdout, stderr, check, want) in rows {
        let out = Output { code, stdout: stdout.as_bytes().to_vec(), stderr: stderr.as_bytes().to_vec() };
        let got = kind_of(&out, check);
        if got != want {
            eprintln!("{name}: got {got:?}");
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

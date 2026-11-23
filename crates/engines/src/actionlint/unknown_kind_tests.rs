//! ZUT for unknown_kind.rs on actionlint 1.7.12's real JSON (fixtures/fixtures.txt records each capture): a kind the
//! run defined is None; one it did not, `shellcheck` included when the run had ShellCheck off, names the kinds it did;
//! stdout that is not the JSON is None.

use super::unknown_kind;
use procio::Output;

const OFF: &str = "the kinds this run defined are action, credentials, deprecated-commands, env-var, events, expression, \
    glob, id, if-cond, job-needs, matrix, permissions, runner-label, shell-name, syntax-check, workflow-call";
const ON: &str = "the kinds this run defined are action, credentials, deprecated-commands, env-var, events, expression, \
    glob, id, if-cond, job-needs, matrix, permissions, runner-label, shell-name, shellcheck, syntax-check, workflow-call";

#[test]
fn unknown_kind_cases() {
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let shellcheck: &[u8] = include_bytes!("fixtures/shellcheck.json");
    let rows: [(&str, &[u8], &str, Option<&str>); 7] = [
        ("defined", findings, "expression", None),
        ("defined, last", findings, "workflow-call", None),
        ("misspelt", findings, "expresion", Some(OFF)),
        ("shellcheck off", include_bytes!("fixtures/clean.json"), "shellcheck", Some(OFF)),
        ("shellcheck on", shellcheck, "shellcheck", None),
        ("pyflakes off", shellcheck, "pyflakes", Some(ON)),
        ("not the JSON", b"", "expresion", None),
    ];
    for (label, stdout, check, expected) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(unknown_kind(&out, check).as_deref(), expected, "{label}");
    }
}

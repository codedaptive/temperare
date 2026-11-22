//! ZUT for golangci_lint_refusal.rs: which checks are refused and the reason given.

use super::golangci_lint_refusal;

#[test]
fn golangci_lint_refusal_refuses_what_is_not_a_linter_list() {
    let why = |check: &str, part: &str| {
        Some(format!(
            "golangci-lint check `{check}`: `{part}` is not a linter name; tm picks golangci-lint's linters by name \
             (`staticcheck`), not a linter's own checks (`SA1019`)"
        ))
    };
    let rows: Vec<(&str, Option<String>)> = vec![
        ("staticcheck", None),
        ("govet", None),
        ("err113", None),
        ("wsl_v5", None),
        ("staticcheck,govet", None),
        ("SA1019", why("SA1019", "SA1019")),
        ("Staticcheck", why("Staticcheck", "Staticcheck")),
        ("typecheck", why("typecheck", "typecheck")),
        ("staticcheck,typecheck", why("staticcheck,typecheck", "typecheck")),
        ("", why("", "")),
        ("staticcheck,", why("staticcheck,", "")),
        ("-x", why("-x", "-x")),
        ("staticcheck, govet", why("staticcheck, govet", " govet")),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(check, want)| {
            let got = golangci_lint_refusal(check);
            (got != want).then(|| format!("{check:?}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "{wrong:#?}");
}

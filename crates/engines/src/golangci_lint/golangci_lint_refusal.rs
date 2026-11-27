//! `golangci_lint_refusal`: why cctao cannot run a golangci-lint check as a list of linters, if it cannot.

/// The reason `check` is refused, naming its first comma part that is not a linter name (one or more ASCII lowercase
/// letters, digits or `_`: `staticcheck`, `err113`, `wsl_v5`) or is `typecheck`; None when every part is one. cctao
/// passes the check as `--enable-only=<check>` and matches issues back by their linter, so a linter's own check code
/// (`SA1019`), which golangci-lint takes only from its config file, cannot be tied back. `typecheck` is golangci-lint's
/// load step, whose issues the reader reports as a failed run, never as findings.
pub(crate) fn golangci_lint_refusal(check: &str) -> Option<String> {
    let part = check.split(',').find(|part| *part == "typecheck" || !is_linter_name(part))?;
    Some(format!(
        "golangci-lint check `{check}`: `{part}` is not a linter name; cctao picks golangci-lint's linters by name \
         (`staticcheck`), not a linter's own checks (`SA1019`)"
    ))
}

/// One or more ASCII lowercase letters, digits or `_`.
fn is_linter_name(part: &str) -> bool {
    !part.is_empty() && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

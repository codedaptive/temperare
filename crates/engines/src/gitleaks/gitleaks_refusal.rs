//! `gitleaks_refusal`: why tm will not run a gitleaks check, if it will not.

/// The reason `check` is refused; None unless a comma part of it is empty or is `ALL`. gitleaks fails on an empty
/// part ("Requested rule  not found in rules") and knows no rule named ALL, while the SARIF reader would read ALL as
/// every rule. Other ids are not shape-checked: a repo's own gitleaks config can name rules of its own.
pub(crate) fn gitleaks_refusal(check: &str) -> Option<String> {
    let part = check.split(',').find(|part| part.is_empty() || *part == "ALL")?;
    let named = if part.is_empty() { "an empty part".to_string() } else { format!("`{part}`") };
    Some(format!("gitleaks check `{check}`: {named} is not a gitleaks rule id"))
}

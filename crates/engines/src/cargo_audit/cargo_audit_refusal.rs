//! `cargo_audit_refusal`: why cctao cannot run a cargo-audit check, if it cannot.

/// The advisory kinds a check may name; `all` keeps every advisory.
const KINDS: [&str; 5] = ["all", "vulnerability", "unmaintained", "unsound", "notice"];

/// None when `check` is one of `KINDS` or an advisory id `RUSTSEC-<4 digits>-<4 digits>`. `yanked` is refused because
/// cargo-audit learns of a yanked crate only from the crates.io index, which cctao never fetches; anything else is
/// refused as neither an id nor a kind.
pub(crate) fn cargo_audit_refusal(check: &str) -> Option<String> {
    if KINDS.contains(&check) || is_advisory_id(check) {
        return None;
    }
    if check == "yanked" {
        return Some("cargo-audit check `yanked` needs the crates.io index, which cctao never fetches; name an advisory id or a kind: all, vulnerability, unmaintained, unsound or notice".to_string());
    }
    Some(format!(
        "cargo-audit check `{check}` is not an advisory id (RUSTSEC-YYYY-NNNN) or a kind: all, vulnerability, unmaintained, unsound or notice"
    ))
}

/// Whether `check` reads `RUSTSEC-`, four ASCII digits, `-`, four ASCII digits.
fn is_advisory_id(check: &str) -> bool {
    let Some((year, number)) = check.strip_prefix("RUSTSEC-").and_then(|rest| rest.split_once('-')) else {
        return false;
    };
    [year, number].iter().all(|part| part.len() == 4 && part.bytes().all(|b| b.is_ascii_digit()))
}

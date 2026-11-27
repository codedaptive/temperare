//! `vulture_refusal`: why cctao cannot match a vulture check's findings back to it, if it cannot.

use super::VULTURE_KINDS;

/// The reason `check` is refused; None for exactly one of the codes in `VULTURE_KINDS`. The check is refused before
/// running because vulture has no option that selects a kind: it reports every kind and never says it does not know
/// a check, so a check that is not one of its codes would read clean. F401 and F841 are flake8's codes, which
/// vulture's `# noqa` reads as V104 and V107; its findings never carry them, so the reason names the code to use.
pub(crate) fn vulture_refusal(check: &str) -> Option<String> {
    if VULTURE_KINDS.iter().any(|(code, _)| *code == check) {
        return None;
    }
    let flake8 = |code: &str| {
        format!("vulture check `{check}` is flake8's code, which vulture's noqa reads as {code}; its findings carry {code}: name {code}")
    };
    Some(match check {
        "F401" => flake8("V104"),
        "F841" => flake8("V107"),
        _ => {
            let codes: Vec<&str> = VULTURE_KINDS.iter().map(|(code, _)| *code).collect();
            format!(
                "vulture check `{check}` is not a vulture code (one of {}), so cctao cannot tell which findings it selects",
                codes.join(", ")
            )
        }
    })
}

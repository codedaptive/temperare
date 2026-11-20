//! `swiftlint_refusal`: why tm will not run a swiftlint check, if it will not.

/// The reason a `custom_rules` check is refused; `None` for any other check. Under `--only-rule` that name runs no
/// custom rule, so swiftlint reports nothing and the rule could match nothing (S-35).
pub(crate) fn swiftlint_refusal(check: &str) -> Option<String> {
    (check == "custom_rules").then(|| {
        "swiftlint check `custom_rules` names no rule swiftlint runs under --only-rule: name the custom rule's own \
         identifier"
            .to_string()
    })
}

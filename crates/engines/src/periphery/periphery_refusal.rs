//! `periphery_refusal`: why tm cannot run a Periphery check, if it cannot.

/// The result hints Periphery 3.8.0 prints, and `all`, which keeps every result.
const HINTS: [&str; 7] = [
    "all",
    "unused",
    "assignOnlyProperty",
    "redundantProtocol",
    "redundantConformance",
    "redundantPublicAccessibility",
    "superfluousIgnoreCommand",
];

/// None when `check` is one of `HINTS`, written as Periphery writes it; else the refusal naming them. Refused before
/// anything runs rather than left to the engine: Periphery is never given the check and never says it does not know
/// one, and `parse_periphery` keeps the results carrying the check's hint, so a check that is no hint would read clean.
pub(crate) fn periphery_refusal(check: &str) -> Option<String> {
    if HINTS.contains(&check) {
        return None;
    }
    Some(format!(
        "periphery check `{check}` is not a Periphery result hint: all, unused, assignOnlyProperty, redundantProtocol, redundantConformance, redundantPublicAccessibility or superfluousIgnoreCommand"
    ))
}

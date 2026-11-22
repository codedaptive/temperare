//! ZUT for pylint_refusal.rs: which checks are refused and the reason given, the cases written out from the contract
//! and pylint 4.1.4's old names.

use super::pylint_refusal;

fn not_an_id(check: &str) -> Option<String> {
    Some(format!(
        "pylint check `{check}` is not a message id (a letter C, R, W, E, F or I and four digits, as C0121), so tm cannot tell which findings it selects"
    ))
}

#[test]
fn pylint_refusal_refuses_what_is_not_a_current_message_id() {
    let rows = [
        ("a convention id", "C0121", None),
        ("an info id", "I0021", None),
        ("a fatal id", "F0001", None),
        ("a removed id, not renamed", "C0330", None),
        ("a symbol", "singleton-comparison", not_an_id("singleton-comparison")),
        ("a category", "C", not_an_id("C")),
        ("a checker name", "design", not_an_id("design")),
        ("a comma list", "C0121,C0123", not_an_id("C0121,C0123")),
        ("lowercase", "c0121", not_an_id("c0121")),
        ("three digits", "C012", not_an_id("C012")),
        ("five digits", "C01210", not_an_id("C01210")),
        ("another letter", "X0121", not_an_id("X0121")),
        ("empty", "", not_an_id("")),
        (
            "an old id of one",
            "E1103",
            Some("pylint check `E1103` is an old id of E1101 in pylint 4.1.4, whose findings carry E1101: name E1101".to_string()),
        ),
        (
            "an old id of two",
            "E0012",
            Some(
                "pylint check `E0012` is an old id of R0022 or W0012 in pylint 4.1.4, whose findings carry R0022 or W0012: name R0022 or W0012"
                    .to_string(),
            ),
        ),
    ];
    for (name, check, want) in rows {
        assert_eq!(pylint_refusal(check), want, "{name}");
    }
}

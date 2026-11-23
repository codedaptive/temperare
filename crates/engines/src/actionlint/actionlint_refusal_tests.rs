//! ZUT for actionlint_refusal.rs: lowercase words joined by single `-` pass; anything else is refused.

use super::actionlint_refusal;

#[test]
fn actionlint_refusal_cases() {
    let refused = |check: &str| {
        Some(format!(
            "actionlint check `{check}` is not an actionlint check kind (lowercase words joined by -, such as expression)"
        ))
    };
    let rows: [(&str, Option<String>); 13] = [
        ("expression", None),
        ("runner-label", None),
        ("deprecated-commands", None),
        ("expresion", None),
        ("", refused("")),
        ("Expression", refused("Expression")),
        ("syntax_check", refused("syntax_check")),
        ("-expression", refused("-expression")),
        ("expression-", refused("expression-")),
        ("if--cond", refused("if--cond")),
        ("SC2086", refused("SC2086")),
        (" expression", refused(" expression")),
        ("expression,action", refused("expression,action")),
    ];
    for (check, expected) in rows {
        assert_eq!(actionlint_refusal(check), expected, "{check:?}");
    }
}

//! `pylint_refusal`: why cctao cannot match a pylint check's findings back to it, if it cannot.

/// Each old message id pylint 4.1.4 still accepts, with the current id(s) its findings carry. Listed from pylint
/// 4.1.4's message store (every message's `old_names`, ids only); none of these is a current id in 4.1.4.
const OLD_IDS: &[(&str, &[&str])] = &[
    ("C0102", &["C0104"]),
    ("C0111", &["C0114", "C0115", "C0116"]),
    ("C0113", &["C0117"]),
    ("C0144", &["C2401"]),
    ("C1801", &["C1802"]),
    ("C1901", &["C1804"]),
    ("C2001", &["C1805"]),
    ("E0012", &["R0022", "W0012"]),
    ("E0234", &["E0301"]),
    ("E0235", &["E0302"]),
    ("E0632", &["W0632"]),
    ("E0703", &["E0705"]),
    ("E1103", &["E1101"]),
    ("E1140", &["E1143"]),
    ("F0401", &["E0401"]),
    ("I0014", &["I0022"]),
    ("R0101", &["R1702"]),
    ("R0102", &["R1703"]),
    ("W0132", &["C0112"]),
    ("W0154", &["C0123"]),
    ("W0234", &["E0301"]),
    ("W0235", &["W0246"]),
    ("W0402", &["W4901"]),
    ("W0633", &["E0633"]),
    ("W0703", &["W0718"]),
    ("W1111", &["E1128"]),
    ("W1403", &["W1404"]),
    ("W1505", &["W4902"]),
    ("W1511", &["W4903"]),
    ("W1512", &["W4904"]),
    ("W1513", &["W4905"]),
    ("W1516", &["W1518"]),
    ("W1517", &["W1518"]),
];

/// The reason `check` is refused; None for a message id that is not an old id. A check that is not a message id (a
/// symbol, a category, a checker name or a comma list) enables messages reported under other ids, so cctao could not
/// match their findings to it. An old id is accepted by pylint, which reports its findings under the current id, so
/// the rule would read clean; the reason names the current id(s), joined by " or ".
pub(crate) fn pylint_refusal(check: &str) -> Option<String> {
    if !is_message_id(check) {
        return Some(format!(
            "pylint check `{check}` is not a message id (a letter C, R, W, E, F or I and four digits, as C0121), so cctao cannot tell which findings it selects"
        ));
    }
    let (_, new) = OLD_IDS.iter().find(|(old, _)| *old == check)?;
    let new = new.join(" or ");
    Some(format!("pylint check `{check}` is an old id of {new} in pylint 4.1.4, whose findings carry {new}: name {new}"))
}

/// One of the letters C, R, W, E, F or I, then exactly four ASCII digits.
fn is_message_id(check: &str) -> bool {
    match check.as_bytes() {
        [letter, digits @ ..] => b"CRWEFI".contains(letter) && digits.len() == 4 && digits.iter().all(u8::is_ascii_digit),
        [] => false,
    }
}

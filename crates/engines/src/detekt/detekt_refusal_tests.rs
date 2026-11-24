//! ZUT for detekt_refusal.rs: well-formed names pass, whether or not temperare's list holds them; the first part that
//! is malformed is named.

use super::detekt_refusal;

#[test]
fn detekt_refusal_names_the_first_malformed_part() {
    let rows: [(&str, Option<&str>); 14] = [
        ("MagicNumber", None),
        ("style", None),
        ("empty-blocks", None),
        ("MagicNumber,UnusedPrivateMember", None),
        ("style,naming", None),
        ("ALL", None),
        ("NoSuchRule", None),
        ("Style", None),
        ("MagicNumber,NoSuchRule,ALL", None),
        ("", Some("an empty name")),
        ("MagicNumber,", Some("an empty name")),
        ("detekt.style.MagicNumber", Some("`detekt.style.MagicNumber`")),
        ("style.MagicNumber", Some("`style.MagicNumber`")),
        ("Magic Number", Some("`Magic Number`")),
    ];
    for (check, named) in rows {
        let reason = detekt_refusal(check);
        match named {
            None => assert_eq!(reason, None, "{check}"),
            Some(token) => {
                let reason = reason.unwrap_or_else(|| panic!("{check} was not refused"));
                assert!(reason.contains(token), "{check}: {reason}");
            }
        }
    }
}

//! ZUT for detekt_unlisted.rs: a check whose every part is a listed rule or rule set gives no stop; otherwise the first
//! part that is not is named, with the detekt release the list was taken from.

use super::detekt_unlisted;
use crate::types::Stop;

#[test]
fn detekt_unlisted_names_the_first_part_not_in_the_list() {
    let rows: [(&str, Option<&str>); 7] = [
        ("NoSuchRule", Some("NoSuchRule is not in temperare's list of detekt codes (taken from detekt 1.23.8)")),
        ("ALL", Some("ALL is not in temperare's list of detekt codes (taken from detekt 1.23.8)")),
        ("Style", Some("Style is not in temperare's list of detekt codes (taken from detekt 1.23.8)")),
        (
            "MagicNumber,NoSuchRule,ALL",
            Some("NoSuchRule is not in temperare's list of detekt codes (taken from detekt 1.23.8)"),
        ),
        ("MagicNumber", None),
        ("style", None),
        ("MagicNumber,empty-blocks", None),
    ];
    for (check, said) in rows {
        let expected = said.map(|said| Stop::NotListed { said: said.to_string() });
        assert_eq!(detekt_unlisted(check), expected, "{check}");
    }
}

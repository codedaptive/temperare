//! ZUT for oid_of_revision: absent, valid and unusable revision answers.

use super::oid_of_revision::oid_of_revision;
use ccvcslib::{Revision, VcsError};

#[test]
fn revision_answers_become_oids_or_fail() {
    let cases = [
        ("none", None, Ok(None)),
        (
            "40 hex",
            Some("0123456789abcdef0123456789abcdef01234567"),
            Ok(Some("0123456789abcdef0123456789abcdef01234567")),
        ),
        ("short", Some("0123456789ab"), Err(())),
        ("non-hex", Some("g123456789abcdef0123456789abcdef01234567"), Err(())),
    ];
    for (name, text, expected) in cases {
        let actual = oid_of_revision(text.map(|s| Revision(s.to_string())));
        match expected {
            Ok(expected) => assert_eq!(actual.map(|oid| oid.map(|v| v.as_str().to_string())), Ok(expected.map(str::to_string)), "{name}"),
            Err(()) => assert!(matches!(actual, Err(VcsError::Failed(_))), "{name}: {actual:?}"),
        }
    }
}

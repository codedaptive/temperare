//! ZUT for dart_refusal.rs: one lower-case code is run; any other shape is refused with a why naming the check.

use super::dart_refusal;

#[test]
fn dart_refusal_accepts_one_lower_case_code() {
    let rows = [
        ("unused_local_variable", false),
        ("prefer_single_quotes", false),
        ("uri_does_not_exist", false),
        ("a", false),
        ("non_constant_identifier_names2", false),
        ("", true),
        ("UNUSED_LOCAL_VARIABLE", true),
        ("Unused_local_variable", true),
        ("9lives", true),
        ("_private", true),
        ("unused-local-variable", true),
        ("unused_local_variable,prefer_single_quotes", true),
        ("unused local", true),
        ("unused_*", true),
    ];
    let mut failed = Vec::new();
    for (check, refused) in rows {
        let result = std::panic::catch_unwind(|| match dart_refusal(check) {
            None => assert!(!refused, "{check:?}: not refused"),
            Some(why) => {
                assert!(refused, "{check:?}: refused with {why:?}");
                assert!(why.contains(&format!("`{check}`")), "{check:?}: {why:?}");
                assert!(why.contains("lower case"), "{check:?}: {why:?}");
            }
        });
        if result.is_err() { failed.push(check); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

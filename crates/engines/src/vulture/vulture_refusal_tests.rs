use super::vulture_refusal;

#[test]
fn vulture_refusal_accepts_only_supported_codes() {
    let rows = [
        ("V101", None), ("V102", None), ("V103", None), ("V104", None),
        ("V105", None), ("V106", None), ("V107", None), ("V201", None),
        ("F401", Some("V104")), ("F841", Some("V107")),
        ("V108", Some("codes")), ("v103", Some("codes")), ("", Some("codes")),
        ("V103,V104", Some("codes")), ("all", Some("codes")), ("unused-function", Some("codes")),
    ];
    let mut failed = Vec::new();
    for (check, expected) in rows {
        let result = std::panic::catch_unwind(|| match expected {
            None => assert_eq!(vulture_refusal(check), None, "{check:?}"),
            Some(token) => {
                let reason = vulture_refusal(check).expect("must refuse the check");
                if token == "codes" {
                    for code in ["V101", "V102", "V103", "V104", "V105", "V106", "V107", "V201"] {
                        assert!(reason.contains(code), "{check:?}: {reason:?} missing {code}");
                    }
                } else {
                    assert!(reason.contains(token), "{check:?}: {reason:?} missing {token}");
                }
            }
        });
        if result.is_err() { failed.push(check); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

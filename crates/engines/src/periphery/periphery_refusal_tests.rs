use super::periphery_refusal;

#[test]
fn periphery_refusal_cases() {
    let rows = [
        ("all", None),
        ("unused", None),
        ("assignOnlyProperty", None),
        ("redundantProtocol", None),
        ("redundantConformance", None),
        ("redundantPublicAccessibility", None),
        ("superfluousIgnoreCommand", None),
        ("Unused", Some("periphery check `Unused` is not a Periphery result hint: all, unused, assignOnlyProperty, redundantProtocol, redundantConformance, redundantPublicAccessibility or superfluousIgnoreCommand")),
        ("unknown", Some("periphery check `unknown` is not a Periphery result hint: all, unused, assignOnlyProperty, redundantProtocol, redundantConformance, redundantPublicAccessibility or superfluousIgnoreCommand")),
        ("", Some("periphery check `` is not a Periphery result hint: all, unused, assignOnlyProperty, redundantProtocol, redundantConformance, redundantPublicAccessibility or superfluousIgnoreCommand")),
    ];
    for (check, expected) in rows {
        assert_eq!(periphery_refusal(check).as_deref(), expected, "{check}");
    }
}

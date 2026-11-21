//! Unit tests for loosening.rs: `before` and `after` serialize when present and are left out when absent.

use super::{Loosening, LooseningKind};

#[test]
fn before_and_after_serialize_only_when_present() {
    let rows = [
        (
            Loosening {
                kind: LooseningKind::SetOlder,
                about: "standard".to_string(),
                before: Some("2".to_string()),
                after: Some("1".to_string()),
            },
            "kind = \"set_older\"\nabout = \"standard\"\nbefore = \"2\"\nafter = \"1\"\n",
        ),
        (
            Loosening { kind: LooseningKind::RuleGone, about: "r1".to_string(), before: None, after: None },
            "kind = \"rule_gone\"\nabout = \"r1\"\n",
        ),
    ];
    for (loosening, text) in rows {
        assert_eq!(toml::to_string(&loosening).unwrap(), text);
    }
}

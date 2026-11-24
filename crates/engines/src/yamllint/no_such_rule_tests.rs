//! ZUT for no_such_rule.rs on yamllint 1.38.0's real stderr for an unknown rule `nosuch` (fixtures/no_such_rule.txt),
//! and on stderr that names another rule or says nothing of rules.

use super::no_such_rule;
use procio::Output;

const SAID: &str = "invalid config: no such rule: \"nosuch\"";

#[test]
fn no_such_rule_cases() {
    let captured: &[u8] = include_bytes!("fixtures/no_such_rule.txt");
    let rows: [(&str, &[u8], &str, Option<&str>); 6] = [
        ("captured, its check", captured, "nosuch", Some(SAID)),
        ("captured, another check", captured, "truthy", None),
        ("captured, a check the name starts with", captured, "nos", None),
        ("after other lines", b"a warning\ninvalid config: no such rule: \"nosuch\"\n", "nosuch", Some(SAID)),
        ("another message", b"invalid config: not a mapping\n", "nosuch", None),
        ("empty", b"", "nosuch", None),
    ];
    for (label, stderr, check, expected) in rows {
        let out = Output { code: Some(255), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(no_such_rule(&out, check).as_deref(), expected, "{label}");
    }
}

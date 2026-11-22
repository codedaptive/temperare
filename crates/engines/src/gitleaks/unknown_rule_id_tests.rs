//! ZUT for unknown_rule_id.rs: gitleaks' words for an unknown rule id, in stderr as `error_lines` makes it from the
//! real gitleaks 8.30.1 stderr (fixtures/unknown_rule.txt), picked out when they name a part of the check.

use super::{error_lines, unknown_rule_id};
use procio::Output;

#[test]
fn unknown_rule_id_is_gitleaks_words_about_a_part_of_the_check() {
    let said = Some("Requested rule no-such-rule not found in rules");
    let rows: [(&str, &str, Option<&str>); 5] = [
        ("the check itself", "no-such-rule", said),
        ("a part of a comma list", "aws-access-token,no-such-rule", said),
        ("another id", "aws-access-token", None),
        ("an id the named one only starts with", "no-such", None),
        ("an id that only starts with the named one", "no-such-rule-2", None),
    ];
    let stderr = error_lines(include_bytes!("fixtures/unknown_rule.txt"));
    for (name, check, want) in rows {
        let out = Output { code: Some(1), stdout: Vec::new(), stderr: stderr.clone() };
        assert_eq!(unknown_rule_id(&out, check), want.map(str::to_string), "{name}");
    }
}

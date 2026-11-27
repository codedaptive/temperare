//! Unit test for no_file_rule.rs: plain data; equality covers every field.

use super::NoFileRule;
use rules::Engine;

fn no_file_rule() -> NoFileRule {
    NoFileRule { rule: "no-print".to_string(), engine: Engine::Ruff, why: "no file of the folder matches its paths".to_string() }
}

#[test]
fn no_file_rules_compare_by_every_field() {
    assert_eq!(
        no_file_rule(),
        NoFileRule { rule: "no-print".to_string(), engine: Engine::Ruff, why: "no file of the folder matches its paths".to_string() }
    );
    assert_ne!(no_file_rule(), NoFileRule { rule: "other".to_string(), ..no_file_rule() });
    assert_ne!(no_file_rule(), NoFileRule { engine: Engine::Clippy, ..no_file_rule() });
    assert_ne!(no_file_rule(), NoFileRule { why: "none of the files is one its engine checks".to_string(), ..no_file_rule() });
}

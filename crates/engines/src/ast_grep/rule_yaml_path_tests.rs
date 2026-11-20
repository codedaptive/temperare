//! Unit tests for rule_yaml_path: absolute checks are rejected without filesystem access.

use super::rule_yaml_path::rule_yaml_path;
use std::path::Path;

#[test]
fn absolute_check_is_refused_even_when_folder_does_not_exist() {
    assert_eq!(
        rule_yaml_path(Path::new("/nonexistent-engines-rule-folder"), "/outside/names.yml"),
        Err("ast-grep rule /outside/names.yml is an absolute path: a check names a file inside the rule file's folder".to_string())
    );
}

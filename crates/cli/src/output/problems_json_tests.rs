//! Unit tests for problems_json.rs: the object holds the rule file and each problem in order.

use super::problems_json;
use serde_json::json;
use std::path::Path;

#[test]
fn problems_in_order_with_their_fields() {
    let problems = vec![
        rules::Problem { line: 1, col: 11, what: "version must be 1".to_string(), fix: "Write version = 1.".to_string() },
        rules::Problem { line: 3, col: 1, what: "unknown key".to_string(), fix: "Remove the key.".to_string() },
    ];
    assert_eq!(
        problems_json(Path::new("repo/temperare.toml"), &problems),
        json!({
            "rule_file": "repo/temperare.toml",
            "problems": [
                {"line": 1, "col": 11, "what": "version must be 1", "fix": "Write version = 1."},
                {"line": 3, "col": 1, "what": "unknown key", "fix": "Remove the key."},
            ],
        })
    );
}

#[test]
fn no_problems_is_an_empty_list() {
    assert_eq!(problems_json(Path::new("t.toml"), &[]), json!({"rule_file": "t.toml", "problems": []}));
}

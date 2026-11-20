//! Unit tests for rule_file_read.rs: the unreadable path (reading a real file is the integration test's, tests/read_rule_file.rs).

use crate::{Problem, RuleFile};
use std::path::Path;

#[test]
fn an_unreadable_path_is_one_problem_with_no_position() {
    let path = Path::new("/nonexistent-tm-rules-zut/no-such-folder/temperare.toml");
    let problems: Vec<Problem> = RuleFile::read(path).expect_err("the file does not exist");
    assert_eq!(problems.len(), 1);
    assert_eq!((problems[0].line, problems[0].col), (0, 0));
    assert!(problems[0].what.starts_with("cannot read "), "{}", problems[0].what);
    assert!(problems[0].what.contains("/nonexistent-tm-rules-zut/no-such-folder/temperare.toml"));
    assert!(!problems[0].fix.is_empty());
}

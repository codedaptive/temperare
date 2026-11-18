//! ZIT for temp_rule_write.rs (and the plain struct in temp_rule.rs) on real temp files: the file's place, name and contents.

use super::super::TempRule;

#[test]
fn the_written_file_holds_the_text() {
    let rule = TempRule::write("id: a\nlanguage: JavaScript\n").unwrap();
    assert_eq!(std::fs::read_to_string(&rule.path).unwrap(), "id: a\nlanguage: JavaScript\n");
}

#[test]
fn the_file_is_named_for_the_process_under_the_temp_folder() {
    let rule = TempRule::write("").unwrap();
    assert_eq!(rule.path.parent(), Some(std::env::temp_dir().as_path()));
    let name = rule.path.file_name().unwrap().to_str().unwrap();
    let prefix = format!("tm-ast-grep-{}-", std::process::id());
    assert!(name.starts_with(&prefix) && name.ends_with(".yml"), "{name}");
}

#[test]
fn two_rules_written_by_one_process_get_two_files() {
    let first = TempRule::write("one").unwrap();
    let second = TempRule::write("two").unwrap();
    assert_ne!(first.path, second.path);
    assert_eq!(std::fs::read_to_string(&first.path).unwrap(), "one");
    assert_eq!(std::fs::read_to_string(&second.path).unwrap(), "two");
}

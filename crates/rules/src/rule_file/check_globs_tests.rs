//! ZUT for check_globs.rs.

use super::check_globs::check_globs;

#[test]
fn good_globs_are_kept_and_a_bad_one_is_a_problem_at_its_string() {
    let text = "0123456789\n\"src/[a.rs\"";
    let globs = vec![("ok/**".to_string(), 0), ("src/[a.rs".to_string(), 11), ("**/*.rs".to_string(), 0)];
    let mut out = Vec::new();
    assert_eq!(check_globs(text, "rule \"r\"", &globs, &mut out), vec!["ok/**".to_string(), "**/*.rs".to_string()]);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (2, 1));
    assert!(out[0].what.starts_with("rule \"r\": bad glob \"src/[a.rs\": "), "{}", out[0].what);
}

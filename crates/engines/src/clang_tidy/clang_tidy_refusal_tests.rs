//! ZUT for clang_tidy_refusal.rs: which checks are refused, the cases and the refusal text written out in full.

use super::clang_tidy_refusal;

#[test]
fn clang_tidy_refusal_cases() {
    let why = |check: &str| {
        Some(format!("clang-tidy check `{check}` is not one check name (letters, digits, `-`, `.` and `_`, starting with a letter), so cctao cannot tell which findings it selects"))
    };
    let rows: [(&str, &str, Option<String>); 9] = [
        ("a dashed name", "bugprone-branch-clone", None),
        ("a dotted name", "clang-analyzer-core.NullDereference", None),
        ("an underscore and digits", "cert_err33-c", None),
        ("empty", "", why("")),
        ("a glob", "bugprone-*", why("bugprone-*")),
        ("a list", "a-check,b-check", why("a-check,b-check")),
        ("a leading dash", "-a-check", why("-a-check")),
        ("a leading digit", "9check", why("9check")),
        ("a space", "a check", why("a check")),
    ];
    for (label, check, expected) in rows {
        assert_eq!(clang_tidy_refusal(check), expected, "{label}");
    }
}

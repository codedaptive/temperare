//! ZUT for ignore_listed.rs: a `.gitleaksignore` lists something when a line is neither blank nor a comment.

use super::ignore_listed;

#[test]
fn ignore_listed_is_true_for_an_entry() {
    let rows: [(&str, &str, bool); 4] = [
        ("a fingerprint line", "# why\nconfig.py:aws-access-token:1\n", true),
        ("an indented entry", "  \n   config.py:aws-access-token:1  \n", true),
        ("blank lines and comments only", "\n  \n# config.py:aws-access-token:1\n  # more\n", false),
        ("empty", "", false),
    ];
    for (name, text, want) in rows {
        assert_eq!(ignore_listed(text), want, "{name}");
    }
}

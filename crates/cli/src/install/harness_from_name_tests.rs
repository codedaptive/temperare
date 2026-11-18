//! ZUT for harness_from_name.rs: each name read back; any other text is no harness.

use super::Harness;

#[test]
fn from_name_reads_each_name_back() {
    assert_eq!(Harness::from_name("claude"), Some(Harness::Claude));
    assert_eq!(Harness::from_name("codex"), Some(Harness::Codex));
    assert_eq!(Harness::from_name("cursor"), Some(Harness::Cursor));
}

#[test]
fn from_name_refuses_other_text() {
    for text in ["", "Claude", "vscode", " claude", "claude,codex"] {
        assert_eq!(Harness::from_name(text), None, "{text:?}");
    }
}

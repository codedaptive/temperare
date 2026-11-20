//! ZUT for not_in_list_why.rs: the why with the installed version read, and with it unknown.

use super::{not_in_list_why, CodeList};

fn list() -> CodeList<'static> {
    CodeList { source: "src", version: "0.10.0", codes: vec!["SC2086"] }
}

#[test]
fn the_why_names_the_installed_version_and_the_lists_version() {
    assert_eq!(
        not_in_list_why("shellcheck", Some("0.11.0"), "SC9999", &list()),
        "shellcheck 0.11.0: SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 0.10.0)"
    );
}

#[test]
fn an_unread_version_is_said_unknown() {
    assert_eq!(
        not_in_list_why("shellcheck", None, "SC9999", &list()),
        "shellcheck (version unknown): SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 0.10.0)"
    );
}

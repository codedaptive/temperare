//! ZUT for run_install.rs: the exact lines `failed` writes on `err`, and its exit 1.

use super::run_install::failed;

#[test]
fn an_unrecordable_path_refusal_is_written_as_given_and_returns_1() {
    let mut err = Vec::new();
    let code = failed(
        "the path \"/h\\nfile /victim\" is not valid UTF-8 or holds a newline, carriage return or tab, which the \
         install record cannot hold",
        "Use a HOME, project folder and binary path that are valid UTF-8 with no newline, carriage return or tab; then \
         run temperare install again.",
        &mut err,
    );
    assert_eq!(code, 1);
    assert_eq!(
        err,
        b"tm: cannot install: the path \"/h\\nfile /victim\" is not valid UTF-8 or holds a newline, carriage return \
          or tab, which the install record cannot hold\nnext: Use a HOME, project folder and binary path that are \
          valid UTF-8 with no newline, carriage return or tab; then run temperare install again.\n"
    );
}

#[test]
fn a_disk_error_is_written_after_the_prefix_and_returns_1() {
    let mut err = Vec::new();
    let code = failed("disk full", "Fix the cause above and run temperare install again.", &mut err);
    assert_eq!(code, 1);
    assert_eq!(err, b"tm: cannot install: disk full\nnext: Fix the cause above and run temperare install again.\n");
}

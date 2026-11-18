//! ZUT for run_uninstall.rs: the exact lines `failed` writes on `err`, and its exit 1.

use super::run_uninstall::failed;

#[test]
fn with_the_record_kept_it_writes_the_error_and_run_uninstall_again() {
    let mut err = Vec::new();
    let code = failed("permission denied", "Fix the cause above and run temperare uninstall again.", &mut err);
    assert_eq!(code, 1);
    assert_eq!(
        err,
        b"tm: cannot uninstall: permission denied\nnext: Fix the cause above and run temperare uninstall again.\n"
    );
}

#[test]
fn with_the_record_removed_it_writes_the_error_and_the_by_hand_step() {
    let mut err = Vec::new();
    let code = failed(
        "folder busy",
        "The record is removed; remove each of these folders by hand only if it is empty: /h/.local/state/temperare.",
        &mut err,
    );
    assert_eq!(code, 1);
    assert_eq!(
        err,
        b"tm: cannot uninstall: folder busy\nnext: The record is removed; remove each of these folders by hand only \
          if it is empty: /h/.local/state/temperare.\n"
    );
}

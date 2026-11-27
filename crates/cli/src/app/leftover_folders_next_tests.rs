//! Unit tests for leftover_folders_next.rs: exact guidance for remaining recorded folders.

use std::path::Path;

use super::leftover_folders_next::leftover_folders_next;

#[test]
fn one_folder_is_named_with_the_empty_only_instruction() {
    assert_eq!(
        leftover_folders_next(&[Path::new("/h/.local")]),
        "The record is removed; remove each of these folders by hand only if it is empty: /h/.local."
    );
}

#[test]
fn three_folders_are_named_in_the_given_order() {
    assert_eq!(
        leftover_folders_next(&[
            Path::new("/h/.local/state/cctao"),
            Path::new("/h/.local"),
            Path::new("/h/.local/state"),
        ]),
        "The record is removed; remove each of these folders by hand only if it is empty: \
         /h/.local/state/cctao, /h/.local, /h/.local/state."
    );
}

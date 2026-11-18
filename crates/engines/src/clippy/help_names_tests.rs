//! ZUT for help_names.rs, on small hand-written help texts in the shape `clippy-driver -W help` prints.

use super::help_names;

const HELP: &str = "\
Available lint options:
    -W <foo>           Warn about <foo>

Lint checks provided by rustc:

    name           default  meaning
    ----           -------  -------
    dead-code      warn     detects unused code
    unused-imports warn     imports that are never used

Lint groups provided by rustc:

    name    sub-lints
    ----    ---------
    unused  dead-code, unused-imports

Lint checks loaded by this crate:

    name                     default  meaning
    ----                     -------  -------
    clippy::bool-comparison  warn     comparing to true

Lint groups loaded by this crate:

    name           sub-lints
    ----           ---------
    clippy::style  clippy::bool-comparison

";

#[test]
fn lint_checks_reads_every_lint_checks_section_with_dashes_as_underscores() {
    assert_eq!(help_names(HELP, "Lint checks"), ["dead_code", "unused_imports", "clippy::bool_comparison"]);
}

#[test]
fn lint_groups_reads_every_lint_groups_section() {
    assert_eq!(help_names(HELP, "Lint groups"), ["unused", "clippy::style"]);
}

#[test]
fn a_title_with_no_section_reads_nothing() {
    assert!(help_names(HELP, "Lint tools").is_empty());
}

#[test]
fn rows_before_the_dashes_row_are_not_names() {
    let help = "Lint checks provided by rustc:\n\n    name  default  meaning\n";
    assert!(help_names(help, "Lint checks").is_empty());
}

#[test]
fn a_blank_line_ends_the_table() {
    let help = "Lint checks provided by rustc:\n    ----  -------\n    a-b  warn  x\n\n    c-d  warn  y\n";
    assert_eq!(help_names(help, "Lint checks"), ["a_b"]);
}

//! ZUT for skill_text.rs: the front matter a harness reads, the commands (as `temperare`) and exit codes the skill
//! must teach, and when `tm` answers.

use super::SKILL_TEXT;

#[test]
fn front_matter_names_the_skill_and_describes_it_on_one_line() {
    let mut lines = SKILL_TEXT.lines();
    assert_eq!(lines.next(), Some("---"));
    assert_eq!(lines.next(), Some("name: temperare"));
    assert!(lines.next().is_some_and(|line| line.starts_with("description: ") && line.len() > 13));
    assert_eq!(lines.next(), Some("---"));
}

#[test]
fn teaches_both_commands_as_temperare() {
    assert!(SKILL_TEXT.contains("- `temperare check [--base <rev>] [--json] [<path>...]`: "));
    assert!(SKILL_TEXT.contains("- `temperare rules`: "));
    assert!(SKILL_TEXT.contains("(temperare check)"));
    assert!(!SKILL_TEXT.contains("`tm check"));
    assert!(!SKILL_TEXT.contains("`tm rules"));
    assert!(!SKILL_TEXT.contains("(tm check)"));
}

#[test]
fn says_tm_is_the_short_name_only_where_install_made_the_link() {
    assert!(SKILL_TEXT.lines().any(|line| line == "`tm` is its short name where `temperare install` made the link."));
    assert!(!SKILL_TEXT.contains("answer every command"));
}

#[test]
fn gives_every_exit_code() {
    for code in ["- 0: ", "- 1: ", "- 2: ", "- 3: ", "- 4: ", "- 5: "] {
        assert!(SKILL_TEXT.contains(code), "{code}");
    }
    assert!(SKILL_TEXT.contains("1: a quality error finding refuses"));
    assert!(SKILL_TEXT.contains("3: a rule could not run because its engine is not available"));
    assert!(SKILL_TEXT.contains("4: security error findings only"));
    assert!(SKILL_TEXT.contains("5: an engine failed"));
}

#[test]
fn says_tessera_checks_are_not_available_yet() {
    assert!(!SKILL_TEXT.contains("Tessera when it is there"));
    assert!(SKILL_TEXT.contains("Tessera's checks are not available yet"));
    assert!(SKILL_TEXT.contains("installing Tessera changes nothing"));
}

#[test]
fn says_not_available_is_never_clean() {
    assert!(SKILL_TEXT.contains("\"Not available\" is never \"clean\""));
}

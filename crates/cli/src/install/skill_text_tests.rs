//! ZUT for skill_text.rs: front matter, commands, exit codes and savings guidance.

use super::SKILL_TEXT;

#[test]
fn front_matter_names_the_skill_and_describes_it_on_one_line() {
    let mut lines = SKILL_TEXT.lines();
    assert_eq!(lines.next(), Some("---"));
    assert_eq!(lines.next(), Some("name: cctao"));
    assert!(lines.next().is_some_and(|line| line.starts_with("description: ") && line.len() > 13));
    assert_eq!(lines.next(), Some("---"));
}

#[test]
fn teaches_both_commands_as_cctao() {
    for token in ["`cctao check", "`cctao rules", "tao.toml"] {
        assert!(SKILL_TEXT.contains(token), "{token}");
    }
}

#[test]
fn teaches_rules_loosened_and_its_three_exits() {
    let start = SKILL_TEXT.find("- `cctao rules --loosened --base <rev>`: ");
    let rest = start.map_or("", |i| &SKILL_TEXT[i + 2..]);
    let end = rest.find("\n- ").into_iter().chain(rest.find("\n\n")).min().unwrap_or(rest.len());
    let bullet = rest[..end].split_whitespace().collect::<Vec<_>>().join(" ");
    for token in ["exit 0 nothing", "1 something loosened", "2 a bad --base or rule file"] {
        assert!(bullet.contains(token), "{token}: {bullet}");
    }
}

#[test]
fn does_not_advertise_a_short_name() {
    assert!(!SKILL_TEXT.contains("short name"));
}

#[test]
fn gives_every_exit_code() {
    for code in ["- 0: ", "- 1: ", "- 2: ", "- 3: ", "- 4: ", "- 5: ", "- 6: "] {
        assert!(SKILL_TEXT.contains(code), "{code}");
    }
    assert!(SKILL_TEXT.contains("1: a quality error finding refuses"));
    assert!(SKILL_TEXT.contains("3: a rule could not run because its engine, or what the rule needs, is not available"));
    assert!(SKILL_TEXT.contains("put the rule file inside the Cargo project it checks"));
    assert!(SKILL_TEXT.contains("4: security error findings only"));
    assert!(SKILL_TEXT.contains("5: an engine failed"));
    assert!(SKILL_TEXT.contains("6: a rule of the repository's own covers nothing"));
}

#[test]
fn says_tessera_checks_are_not_available_yet() {
    assert!(!SKILL_TEXT.contains("Tessera when it is there"));
    assert!(SKILL_TEXT.contains("Tessera's checks are not available yet"));
    assert!(SKILL_TEXT.contains("installing Tessera changes nothing"));
}

#[test]
fn savings_contract_names_the_savings_line() {
    assert!(SKILL_TEXT.lines().any(|line| line == "## Tokens saved"));
    assert!(SKILL_TEXT.contains("`cctao check`"));
    assert!(SKILL_TEXT.contains("[cctao]"));
}

#[test]
fn savings_contract_explains_the_savings_ask() {
    for token in ["total cctao tokens saved", "your cctao calls"] {
        assert!(SKILL_TEXT.contains(token), "{token}");
    }
}

#[test]
fn savings_contract_names_saved_json_and_the_off_switch() {
    assert!(SKILL_TEXT.lines().any(|line| line == "`saved`. `savings_line = false` in `~/.config/cctao/config.toml` turns the line and its ask off."));
}

#[test]
fn says_not_available_is_never_clean() {
    assert!(SKILL_TEXT.contains("\"Not available\" is never \"clean\""));
}

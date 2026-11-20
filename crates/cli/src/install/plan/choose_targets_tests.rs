//! Unit tests for choose_targets.rs: --target, -y, the picker and the no-terminal case, with their notes.

use super::choose_targets;
use crate::{Harness, InstallArgs};

const BOTH: [Harness; 2] = [Harness::Claude, Harness::Cursor];

fn never(_: &[Harness]) -> Vec<Harness> {
    panic!("the picker must not be asked")
}

#[test]
fn targets_keep_the_detected_ones_in_order_deduped() {
    let args = InstallArgs {
        targets: Some(vec![Harness::Cursor, Harness::Claude, Harness::Cursor]),
        yes: true,
        ..Default::default()
    };
    assert_eq!(choose_targets(&args, &BOTH, true, &mut never), (vec![Harness::Cursor, Harness::Claude], vec![]));
}

#[test]
fn an_undetected_target_is_noted_by_name_and_not_wired() {
    let args = InstallArgs { targets: Some(vec![Harness::Codex, Harness::Claude]), ..Default::default() };
    let (chosen, notes) = choose_targets(&args, &BOTH, false, &mut never);
    assert_eq!(chosen, vec![Harness::Claude]);
    assert_eq!(notes, vec!["codex not found (no .codex folder in HOME); not wired".to_string()]);
}

#[test]
fn yes_wires_every_detected_harness() {
    let args = InstallArgs { yes: true, ..Default::default() };
    assert_eq!(choose_targets(&args, &BOTH, true, &mut never), (BOTH.to_vec(), vec![]));
}

#[test]
fn at_a_terminal_the_picker_gets_the_detected_list_and_its_answer_is_used() {
    let mut asked = Vec::new();
    let mut pick = |choices: &[Harness]| {
        asked.push(choices.to_vec());
        vec![Harness::Cursor]
    };
    let (chosen, notes) = choose_targets(&InstallArgs::default(), &BOTH, true, &mut pick);
    assert_eq!(chosen, vec![Harness::Cursor]);
    assert!(notes.is_empty());
    assert_eq!(asked, vec![BOTH.to_vec()]);
}

#[test]
fn no_terminal_and_no_yes_wires_nothing_and_says_how() {
    let (chosen, notes) = choose_targets(&InstallArgs::default(), &BOTH, false, &mut never);
    assert!(chosen.is_empty());
    assert_eq!(
        notes,
        vec!["no terminal to ask which harnesses to wire, so none was; to wire them, run temperare install \
              --target claude,cursor -y"
            .to_string()]
    );
}

#[test]
fn nothing_detected_names_the_folders_looked_for_and_asks_no_one() {
    let (chosen, notes) = choose_targets(&InstallArgs::default(), &[], true, &mut never);
    assert!(chosen.is_empty());
    assert_eq!(notes, vec!["no harness found (looked for .claude, .codex, .cursor in HOME); no skill wired"]);
}

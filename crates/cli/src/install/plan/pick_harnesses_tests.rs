//! ZUT for pick_harnesses.rs: the numbered prompt and each kind of answer, from in-memory input.

use super::pick_harnesses;
use crate::Harness;

const BOTH: [Harness; 2] = [Harness::Claude, Harness::Cursor];
const PROMPT: &str = "Wire the temperare skill into which harnesses?\n  1. claude\n  2. cursor\n\
                      Numbers or names, separated by commas; all; or Enter for none: ";

fn answer(detected: &[Harness], input: &str) -> (Vec<Harness>, String) {
    let mut out = Vec::new();
    let chosen = pick_harnesses(detected, &mut input.as_bytes(), &mut out);
    (chosen, String::from_utf8(out).unwrap())
}

#[test]
fn the_prompt_numbers_the_detected_harnesses_from_one() {
    assert_eq!(answer(&BOTH, "\n").1, PROMPT);
}

#[test]
fn an_empty_answer_or_end_of_input_is_none() {
    assert_eq!(answer(&BOTH, "\n").0, vec![]);
    assert_eq!(answer(&BOTH, "   \n").0, vec![]);
    assert_eq!(answer(&BOTH, "").0, vec![]);
}

#[test]
fn all_is_every_detected_harness() {
    assert_eq!(answer(&BOTH, "all\n").0, BOTH.to_vec());
}

#[test]
fn numbers_pick_by_position() {
    assert_eq!(answer(&BOTH, "2\n").0, vec![Harness::Cursor]);
    assert_eq!(answer(&BOTH, "2, 1\n").0, vec![Harness::Cursor, Harness::Claude]);
}

#[test]
fn names_and_numbers_mix_and_repeats_count_once() {
    assert_eq!(answer(&BOTH, "cursor,1,2,claude\n").0, vec![Harness::Cursor, Harness::Claude]);
}

#[test]
fn a_bad_token_is_none_and_names_the_token() {
    for (input, token) in [("3\n", "3"), ("0\n", "0"), ("1,vscode\n", "vscode"), ("codex\n", "codex"), ("1,\n", "")] {
        let (chosen, out) = answer(&BOTH, input);
        assert_eq!(chosen, vec![], "{input:?}");
        assert_eq!(
            out,
            format!("{PROMPT}'{token}' is not one of the choices; no harness wired\n"),
            "{input:?}"
        );
    }
}

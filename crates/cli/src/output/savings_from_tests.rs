//! ZUT for savings_from: rounding, totals, run counts, flooring and percentages.

use super::{savings_from, RuleCount, Savings};

fn count(chars: u64, ran: bool) -> RuleCount {
    RuleCount { rule: "r".into(), engine: "e".into(), chars, ran }
}

#[test]
fn savings_contract_rounds_character_counts_half_up_for_both_totals() {
    for (chars, tokens) in [(0, 0), (1, 0), (2, 1), (5, 1), (6, 2), (2_161_007, 540_252)] {
        let actual = savings_from(&[count(chars, true)], chars);
        assert_eq!(actual.baseline_tokens, tokens, "baseline {chars}");
        assert_eq!(actual.output_tokens, tokens, "output {chars}");
    }
}

#[test]
fn savings_contract_sums_run_characters_before_rounding() {
    let actual = savings_from(&[count(2, true), count(2, true)], 0);
    assert_eq!(actual.engine_runs, 2);
    assert_eq!(actual.baseline_chars, 4);
    assert_eq!(actual.baseline_tokens, 1);
}

#[test]
fn savings_contract_floors_savings_at_zero_when_output_exceeds_baseline() {
    let actual = savings_from(&[count(4, true)], 8);
    assert_eq!(actual.tokens_saved, 0);
    assert_eq!(actual.percent, 0);
}

#[test]
fn savings_contract_zero_baseline_has_zero_percent() {
    let actual = savings_from(&[count(0, true)], 8);
    assert_eq!(actual.baseline_tokens, 0);
    assert_eq!(actual.tokens_saved, 0);
    assert_eq!(actual.percent, 0);
}

#[test]
fn savings_contract_percentage_rounds_down() {
    let actual = savings_from(&[count(12, true)], 4);
    assert_eq!((actual.baseline_tokens, actual.output_tokens, actual.tokens_saved, actual.percent), (3, 1, 2, 66));
}

#[test]
fn savings_contract_counts_runs_independently_of_character_totals() {
    assert_eq!(savings_from(&[count(0, true), count(8, true), count(0, false)], 4), Savings {
        engine_runs: 2, baseline_chars: 8, output_chars: 4, baseline_tokens: 2,
        output_tokens: 1, tokens_saved: 1, percent: 50,
    });
}

#[test]
fn savings_contract_empty_counts_and_empty_output_are_all_zero() {
    assert_eq!(savings_from(&[], 0), Savings {
        engine_runs: 0, baseline_chars: 0, output_chars: 0, baseline_tokens: 0,
        output_tokens: 0, tokens_saved: 0, percent: 0,
    });
}

#[test]
fn savings_contract_large_percentage_calculation_does_not_overflow() {
    let actual = savings_from(&[count(u64::MAX, true)], 0);
    assert_eq!(actual.baseline_tokens, 4_611_686_018_427_387_904);
    assert_eq!(actual.percent, 100);
}

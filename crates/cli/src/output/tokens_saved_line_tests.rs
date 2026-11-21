//! Unit tests for tokens_saved_line: exact wording, comma grouping and no newline.

use super::{tokens_saved_line, Savings};

#[test]
fn savings_contract_renders_the_complete_line_with_literal_example() {
    let savings = Savings { engine_runs: 3, baseline_chars: 2_161_007, output_chars: 54_368,
        baseline_tokens: 540_252, output_tokens: 13_592, tokens_saved: 526_660, percent: 97 };
    assert_eq!(tokens_saved_line(&savings), "[temperare] tokens saved ≈ 526,660 (97%) — this output ≈ 13,592 tok vs the raw output of 3 engine run(s) ≈ 540,252 tok (estimate). At the end of your reply, tell the user the total temperare tokens saved this turn — sum each such line across your temperare calls — e.g. \"🌱 temperare saved ~N tokens this turn\".");
}

#[test]
fn savings_contract_groups_numbers_at_thousands_boundaries() {
    for (value, expected) in [(0, "0"), (999, "999"), (1_000, "1,000"), (1_234_567, "1,234,567")] {
        let savings = Savings { engine_runs: value, baseline_chars: 0, output_chars: 0,
            baseline_tokens: value, output_tokens: value, tokens_saved: value, percent: 1000 };
        let line = tokens_saved_line(&savings);
        assert!(line.contains(&format!("tokens saved ≈ {expected} (1000%)")), "{line}");
        assert!(line.contains(&format!("output ≈ {expected} tok vs")), "{line}");
        assert!(line.contains(&format!("of {expected} engine run(s) ≈ {expected} tok")), "{line}");
        assert!(!line.contains('\n'));
    }
}

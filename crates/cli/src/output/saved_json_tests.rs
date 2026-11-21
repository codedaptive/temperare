//! Unit tests for saved_json: the counted rules in order and integer savings fields.

use super::{saved_json, RuleCount, Savings};
use serde_json::json;

#[test]
fn savings_contract_emits_counted_rules_in_order_without_ran_and_all_totals() {
    let counts = [
        RuleCount { rule: "z".into(), engine: "ruff".into(), chars: 12, ran: true },
        RuleCount { rule: "a".into(), engine: "clippy".into(), chars: 0, ran: false },
    ];
    let savings = Savings { engine_runs: 1, baseline_chars: 12, output_chars: 4,
        baseline_tokens: 3, output_tokens: 1, tokens_saved: 2, percent: 66 };
    assert_eq!(saved_json(&counts, &savings), json!({
        "counted": [{"rule":"z", "engine":"ruff", "chars":12}, {"rule":"a", "engine":"clippy", "chars":0}],
        "engine_runs":1, "baseline_chars":12, "output_chars":4, "baseline_tokens":3,
        "output_tokens":1, "tokens_saved":2, "percent":66
    }));
}

#[test]
fn savings_contract_empty_counts_are_an_empty_array() {
    let savings = Savings { engine_runs: 0, baseline_chars: 0, output_chars: 0,
        baseline_tokens: 0, output_tokens: 0, tokens_saved: 0, percent: 0 };
    assert_eq!(saved_json(&[], &savings), json!({
        "counted":[], "engine_runs":0, "baseline_chars":0, "output_chars":0,
        "baseline_tokens":0, "output_tokens":0, "tokens_saved":0, "percent":0
    }));
}

//! `saved_json`: the savings estimate as the `saved` value of tm check's `--json` object.

use super::{RuleCount, Savings};
use serde_json::json;

/// `{"counted": [{"rule", "engine", "chars"} per count, in input order], "engine_runs", "baseline_chars",
/// "output_chars", "baseline_tokens", "output_tokens", "tokens_saved", "percent"}`, every number a plain integer.
/// A count's `ran` is not printed.
pub fn saved_json(counts: &[RuleCount], savings: &Savings) -> serde_json::Value {
    let counted: Vec<serde_json::Value> = counts
        .iter()
        .map(|count| json!({"rule": count.rule, "engine": count.engine, "chars": count.chars}))
        .collect();
    json!({
        "counted": counted,
        "engine_runs": savings.engine_runs,
        "baseline_chars": savings.baseline_chars,
        "output_chars": savings.output_chars,
        "baseline_tokens": savings.baseline_tokens,
        "output_tokens": savings.output_tokens,
        "tokens_saved": savings.tokens_saved,
        "percent": savings.percent,
    })
}

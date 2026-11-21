//! `savings_from`: the savings estimate from the rules' counts and the characters of tm's own output.

use super::{RuleCount, Savings};

/// The `Savings` for `counts` and `output_chars`: K the counts that ran (have a counted run), baseline
/// characters the sum of every count's `chars` (saturating), tokens at characters / 4 rounded half up, N = B - X
/// floored at 0, P = N * 100 / B rounded down (0 when B is 0), with no overflow for large inputs.
pub fn savings_from(counts: &[RuleCount], output_chars: u64) -> Savings {
    let baseline_chars = counts.iter().fold(0u64, |sum, count| sum.saturating_add(count.chars));
    let baseline_tokens = tokens(baseline_chars);
    let output_tokens = tokens(output_chars);
    let tokens_saved = baseline_tokens.saturating_sub(output_tokens);
    Savings {
        engine_runs: counts.iter().filter(|count| count.ran).map(|_| 1u64).sum(),
        baseline_chars,
        output_chars,
        baseline_tokens,
        output_tokens,
        tokens_saved,
        percent: percent(tokens_saved, baseline_tokens),
    }
}

/// Characters / 4 rounded half up. Written as quotient plus remainder test rather than `(chars + 2) / 4` so that
/// `u64::MAX` neither overflows nor saturates into rounding down.
fn tokens(chars: u64) -> u64 {
    chars / 4 + u64::from(chars % 4 >= 2)
}

/// `saved * 100 / baseline` rounded down, 0 when `baseline` is 0; in u128 so `saved * 100` cannot overflow. The result
/// is at most 100 because `saved` never exceeds `baseline`.
fn percent(saved: u64, baseline: u64) -> u64 {
    if baseline == 0 {
        return 0;
    }
    u64::try_from(u128::from(saved) * 100 / u128::from(baseline)).unwrap_or(100)
}

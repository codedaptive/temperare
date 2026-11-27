//! `Savings`: cctao check's estimate of the tokens its output saves against the raw output of the engines it ran
//! for rules other than watch rules.

/// The estimate: `engine_runs` (K) the engine runs counted, `baseline_chars` and `output_chars` the characters of
/// their raw output and of cctao's own output, `baseline_tokens` (B) and `output_tokens` (X) those at characters / 4
/// rounded half up, `tokens_saved` (N) B - X floored at 0, and `percent` (P) N * 100 / B rounded down, 0 when B is 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Savings {
    pub engine_runs: u64,
    pub baseline_chars: u64,
    pub output_chars: u64,
    pub baseline_tokens: u64,
    pub output_tokens: u64,
    pub tokens_saved: u64,
    pub percent: u64,
}

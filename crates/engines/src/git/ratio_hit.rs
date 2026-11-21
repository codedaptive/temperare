//! `ratio_hit`: the one finding a change gets when it adds too much test for its code.

use super::{Counts, TestRatio};
use crate::Hit;

/// A finding at the top test file's first added line when the change adds at least `floor` lines of test and more
/// than `max_ratio` lines of test per line of code; None otherwise, and when no test file adds a line.
pub(crate) fn ratio_hit(counts: &Counts, ratio: &TestRatio) -> Option<Hit> {
    let (test, code) = (counts.test, counts.code);
    if test < ratio.floor || !over(test, code, ratio.max_ratio) {
        return None;
    }
    let (path, line) = counts.top_test.clone()?;
    let (max, floor) = (ratio.max_ratio, ratio.floor);
    let what = match code {
        0 => format!(
            "this change adds {test} lines of test for 0 lines of code (test only; the rule's limit is {max} past {floor} lines)"
        ),
        _ => format!(
            "this change adds {test} lines of test for {code} lines of code ({} to 1; the rule's limit is {max} past {floor} lines)",
            tenths_text(test, code)
        ),
    };
    Some(Hit { path, line: Some(line), col: None, what })
}

/// Whether `test / code` is more than `max_ratio` as declared. `format!("{}", max_ratio)` is the shortest text that
/// reads back as the same f64: the decimal the rule file declared, for up to 15 significant digits (0.7 prints "0.7"); a
/// longer declaration reads back as a shorter decimal. With that text read as `n`
/// over `10^d` the test is `test * 10^d > n * code`, exact in integers. In f64, 0.7 * 90 is 62.99999999999999, so 63
/// lines against 90 would read as over. A text `over_declared` cannot read counts as over, so the rule reports and
/// never reads clean on a limit tm could not read.
fn over(test: u32, code: u32, max_ratio: f64) -> bool {
    over_declared(test, code, &format!("{max_ratio}")).unwrap_or(true)
}

/// The exact comparison on `max_text`, or None when the text is not digits with at most one '.' or the integers do
/// not fit. Neither happens: `read_test_ratio` bounds `max_ratio` to 0.1..=100, which Rust prints in plain digits
/// (never an exponent), at most 17 significant digits and 18 decimals, so every product stays under 10^30 in u128.
/// Should it happen, the caller counts the change as over.
fn over_declared(test: u32, code: u32, max_text: &str) -> Option<bool> {
    let (whole, decimals) = max_text.split_once('.').unwrap_or((max_text, ""));
    if !whole.bytes().chain(decimals.bytes()).all(|b| b.is_ascii_digit()) {
        return None;
    }
    let n: u128 = format!("{whole}{decimals}").parse().ok()?;
    let scale = 10u128.checked_pow(u32::try_from(decimals.len()).ok()?)?;
    Some(u128::from(test).checked_mul(scale)? > n.checked_mul(u128::from(code))?)
}

/// `test / code` to one decimal, half up, in integers so no float rounding shows: tenths = (20T + C) / (2C).
fn tenths_text(test: u32, code: u32) -> String {
    let (test, code) = (u64::from(test), u64::from(code));
    let tenths = (20 * test + code) / (2 * code);
    format!("{}.{}", tenths / 10, tenths % 10)
}

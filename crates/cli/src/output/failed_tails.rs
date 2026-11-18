//! `failed_tails`: the stderr tail of each failed rule, for the text report that exit 5's next line points at.

use super::serde_name::serde_name;
use report::Report;

/// For each rule in `report.failed`, in order: `stderr of <rule> (<engine>):` and then each line of its stderr tail
/// indented two spaces, or `stderr of <rule> (<engine>): (empty)` when the tail is empty. `<engine>` is the rule-file
/// name. Every line ends with '\n'; a report with no failed rule gives the empty string.
pub fn failed_tails(report: &Report) -> String {
    let mut text = String::new();
    for failed in &report.failed {
        let head = format!("stderr of {} ({})", failed.rule, serde_name(serde_json::to_value(failed.engine)));
        if failed.stderr_tail.is_empty() {
            text.push_str(&format!("{head}: (empty)\n"));
            continue;
        }
        text.push_str(&format!("{head}:\n"));
        failed.stderr_tail.lines().for_each(|line| text.push_str(&format!("  {line}\n")));
    }
    text
}

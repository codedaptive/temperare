//! `unknown_selector`: ruff's words saying a selector of the rule's check names no rule it knows.

use procio::Output;

/// The first line of `out`'s stderr that contains "Unknown rule selector `<part>`" or "Unknown rule selector:
/// `<part>`", where `<part>` is one of `check`'s comma-separated parts exactly as ruff splits them (untrimmed, as
/// `ruff_selects` reads them), trimmed; `None` when no line does. ruff given a selector it does not know exits 2 and
/// prints either "ruff failed" then "  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI", or, from its
/// argument parser, "error: invalid value 'ZZZ999' for '--select <RULE_CODE>': Unknown rule selector: `ZZZ999`". A
/// line naming another selector (one in the rule's config file, say) is not about the check, so it is left to the
/// reader.
pub(crate) fn unknown_selector(out: &Output, check: &str) -> Option<String> {
    let names: Vec<String> = check
        .split(',')
        .flat_map(|part| [format!("Unknown rule selector `{part}`"), format!("Unknown rule selector: `{part}`")])
        .collect();
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| names.iter().any(|name| line.contains(name.as_str())))
        .map(|line| line.trim().to_string())
}

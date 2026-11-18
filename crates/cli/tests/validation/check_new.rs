//! `Check::new`: the check of one case that checks.txt lists.

use std::path::PathBuf;

use crate::check::Check;
use crate::write_head_once::write_head_once;

/// Every acceptance case in report order, one `order|case|section` line each; `scripts/accept.sh` reads it too.
const CHECKS: &str = include_str!("checks.txt");

impl Check {
    /// The check of `case` (its test's name), which set up what `setup` says in one line. When ACCEPT_PARTS names a
    /// folder, the report head is written there first (once per run) and this case's part will be
    /// `<order>-<case>.md`. Panics when checks.txt does not list `case`, so no case that builds a Check is missing
    /// from the list.
    pub fn new(case: &'static str, setup: &str) -> Check {
        let order = order_of(case)
            .unwrap_or_else(|| panic!("{case} is not in crates/cli/tests/validation/checks.txt; add it there"));
        let part = std::env::var_os("ACCEPT_PARTS").map(PathBuf::from).map(|parts| {
            write_head_once(&parts);
            parts.join(format!("{order}-{case}.md"))
        });
        Check { part, case, setup: setup.to_string(), saw: Vec::new(), not_run: None }
    }
}

/// The order checks.txt gives `case`; None when no line lists it. Blank lines and `#` lines are not cases.
fn order_of(case: &str) -> Option<&'static str> {
    CHECKS.lines().filter(|line| !line.is_empty() && !line.starts_with('#')).find_map(|line| {
        let mut fields = line.splitn(3, '|');
        let order = fields.next()?;
        (fields.next()? == case).then_some(order)
    })
}

//! `left_out`: a file in the rule's scope that rubocop's stock config inspects and the repository's config did not.

use rules::{RelPath, Rule};

/// The first non-empty line of `stock_list` (`rubocop --list-target-files --force-default-config .`, one path per
/// line, relative to the folder it ran in) that `rule` applies to and that `inspected` (the check run's files, the
/// same relative form) does not hold; `None` when there is none. Such a file was left out by the repository's own
/// config (its `AllCops` `Exclude` or `Include` replaces the stock list), so the rule would read clean over a file it
/// never checked.
pub(crate) fn left_out(stock_list: &str, inspected: &[RelPath], rule: &Rule) -> Option<String> {
    stock_list
        .lines()
        .filter(|line| !line.is_empty())
        .map(RelPath::new)
        .find(|path| rule.applies(path) && !inspected.contains(path))
        .map(|path| path.as_str().to_string())
}

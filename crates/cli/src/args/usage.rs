//! `USAGE`: the text `tm help` prints.

/// tm's usage: one line per command form, then where the rule file comes from, and that `temperare` is the program
/// while `tm` answers only where `temperare install` made the `tm` link, which install does not always make. Then the
/// savings paragraph: the tokens-saved estimate that opens check's text output when an engine ran (characters / 4, an
/// upper bound), the `savings_line` switch in ~/.config/temperare/config.toml, and the log at
/// ~/.local/state/temperare/savings.jsonl. Last, tm check's exit codes, one line each, worded from OUTPUT.md's "Exit
/// codes" table plus 6 (SPEC-baseline "Which rules apply"), and the order in which the first that applies wins.
pub const USAGE: &str = "\
usage:
tm check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]
tm rules [--rules <path>] [--json]
tm baseline test [--set <name@version>] [--json]
tm install [--no-place] [--short-name] [--target claude,codex,cursor] [--mode skill] [--location local] [-y]
tm uninstall
tm help
tm --version

The rule file is --rules <path>, else temperare.toml at the top of the git repository.
temperare is the program; tm answers the same commands only where temperare install made the tm link.

When at least one engine ran, tm check's text output opens with an estimate of the tokens saved against the raw
output of the engines it ran (characters / 4).
It is an upper bound: the saving against reading that output whole.
savings_line = false in ~/.config/temperare/config.toml turns the line and its ask off; the JSON saved object stays.
Each check that ran an engine appends a line to ~/.local/state/temperare/savings.jsonl.

tm check exits with:
0  nothing refuses; warnings alone end here
1  a kept quality error finding
2  nothing was checked: a usage error, no rule file or a problem in it, or a bad path or --base
3  a rule was not available
4  kept error findings, all of them security ones
5  a rule failed: its engine gave no answer that can be read, or the rule was refused
6  a rule of the repository's own covers nothing
The first that applies wins, in this order: 2, 1, 5, 3, 4, 6, 0.
";

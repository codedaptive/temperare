//! `USAGE`: the text `cctao help` prints for Code Chi Tao.

/// Tao's usage: one line per command form, then where the rule file comes from. Then the savings paragraph: the
/// tokens-saved estimate that opens check's text output when an engine ran for a rule other than a watch rule
/// (characters / 4, an upper bound), the `savings_line` switch in ~/.config/cctao/config.toml, and the log at
/// ~/.local/state/cctao/savings.jsonl. Last, cctao check's exit codes, one line each, worded from OUTPUT.md's "Exit
/// codes" table plus 6 (SPEC-baseline "Which rules apply"), and the order in which the first that applies wins; then
/// one line for the exit codes of `cctao rules --loosened`.
pub const USAGE: &str = "\
usage:
cctao check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]
cctao rules [--rules <path>] [--json]
cctao rules --loosened --base <rev> [--rules <path>] [--json]
cctao baseline test [--set <name@version>] [--json]
cctao install [--no-place] [--target claude,codex,cursor] [--mode skill] [--location local] [-y]
cctao uninstall
cctao help
cctao --version

The rule file is --rules <path>, else tao.toml at the top of the git repository.

When an engine ran for a rule other than a watch rule, cctao check's text output opens with an estimate of the tokens
saved against the raw output of the engines it ran (characters / 4).
It is an upper bound: the saving against reading that output whole.
savings_line = false in ~/.config/cctao/config.toml turns the line and its ask off; the JSON saved object stays.
Each check that ran an engine for a rule other than a watch rule appends a line to
~/.local/state/cctao/savings.jsonl.

cctao check exits with:
0  nothing refuses; warnings alone end here
1  a kept quality error finding
2  nothing was checked: a usage error, no rule file or a problem in it, or a bad path or --base
3  a rule was not available
4  kept error findings, all of them security ones
5  a rule failed: its engine gave no answer that can be read, or the rule was refused
6  a rule of the repository's own covers nothing
The first that applies wins, in this order: 2, 1, 5, 3, 4, 6, 0.
cctao rules --loosened exits 0 when nothing loosened, 1 when something did, 2 for a bad --base or rule file.
";

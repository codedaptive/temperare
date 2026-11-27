//! `vale_filter`: the `--filter` argument that keeps one rule and makes Vale fail when it has no such rule.

/// The one argument `--filter=<expr>` for `check`, a rule name `vale_refusal` let through:
/// `.Name == "<check>" || (none(Rules, {.Name == "<check>"}) && int("unknown") == 0)`. Vale keeps the loaded rules
/// the expression is true for, so only `check` runs. When no loaded rule is named `check`, the right side is reached
/// and `int("unknown")` fails, so Vale stops with an error object whose Text starts `invalid operation:
/// int(unknown)` and echoes the expression (`vale_unknown` reads it). A plain `.Name == "<check>"` would keep no rule
/// and read as clean. Measured with Vale 3.28.1: the error in fixtures/unknown.txt, the plain filter's clean read in
/// fixtures/fixtures.txt ("Filter and config precedence").
pub(crate) fn vale_filter(check: &str) -> String {
    format!("--filter=.Name == \"{check}\" || (none(Rules, {{.Name == \"{check}\"}}) && int(\"unknown\") == 0)")
}

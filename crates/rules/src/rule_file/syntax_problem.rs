//! `syntax_problem`: the Problem for a TOML syntax error in rule-file text.

use super::problem_at::problem_at;
use crate::Problem;

/// The Problem for a TOML syntax error, at its span (1:1 when toml gives none).
pub(crate) fn syntax_problem(text: &str, e: &toml::de::Error) -> Problem {
    problem_at(
        text,
        e.span().map_or(0, |span| span.start),
        format!("TOML syntax error: {}", e.message().trim()),
        "Correct the TOML syntax at this position.",
    )
}

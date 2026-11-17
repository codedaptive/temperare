//! `wrong_type`: the Problem for a value of the wrong TOML type.

use super::problem_at::problem_at;
use crate::Problem;
use toml::Spanned;
use toml::de::DeValue;

/// The Problem at `value`: `key` must be `expected` (e.g. "a string"), naming the type it is.
pub(crate) fn wrong_type(text: &str, key: &str, value: &Spanned<DeValue>, expected: &str) -> Problem {
    problem_at(
        text,
        value.span().start,
        format!("`{key}` must be {expected}, not {}", value.get_ref().type_str()),
        &format!("Write `{key}` as {expected}."),
    )
}

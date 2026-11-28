//! `check_globs`: the globs of a rule, kit or override that compile.

use super::problem_at::problem_at;
use crate::{Problem, Scope};

/// The globs (each with its byte offset) that compile the way a rule's scope compiles them; each that does not is a
/// Problem at its string whose what is `<owner>: bad glob "<glob>": <reason>`, `owner` naming the table that holds
/// the glob (e.g. `rule "r"`, `kit "core"`).
pub(crate) fn check_globs(text: &str, owner: &str, globs: &[(String, usize)], out: &mut Vec<Problem>) -> Vec<String> {
    globs
        .iter()
        .filter_map(|(glob, at)| match Scope::build(std::slice::from_ref(glob), &[], None) {
            Ok(_) => Some(glob.clone()),
            Err(e) => {
                out.push(problem_at(
                    text,
                    *at,
                    format!("{owner}: {e}"),
                    "Write a glob such as \"src/**/*.rs\": `*` stays within one folder, `**` crosses folders.",
                ));
                None
            }
        })
        .collect()
}

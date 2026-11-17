//! `check_globs`: the globs of a rule or kit that compile.

use super::problem_at::problem_at;
use crate::{Problem, Scope};

/// The globs (each with its byte offset) that compile the way a rule's scope compiles them; each that does not is a
/// Problem at its string.
pub(crate) fn check_globs(text: &str, globs: &[(String, usize)], out: &mut Vec<Problem>) -> Vec<String> {
    globs
        .iter()
        .filter_map(|(glob, at)| match Scope::build(std::slice::from_ref(glob), &[], None) {
            Ok(_) => Some(glob.clone()),
            Err(e) => {
                out.push(problem_at(
                    text,
                    *at,
                    e,
                    "Write a glob such as \"src/**/*.rs\": `*` stays within one folder, `**` crosses folders.",
                ));
                None
            }
        })
        .collect()
}

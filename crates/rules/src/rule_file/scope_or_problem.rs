//! `scope_or_problem`: a built Scope, or the Problem for a failed build.

use super::problem_at::problem_at;
use crate::{Problem, Scope};

/// The scope in `built`; on Err(e), a Problem at byte `at` whose what is `what(e)`, and None. Callers pass the build
/// result rather than the globs because, once each glob has compiled alone, a set fails only at globset's size
/// limit, which no unit test can reach quickly; this seam lets the failure branch be tested with a plain Err.
pub(crate) fn scope_or_problem(
    built: Result<Scope, String>,
    text: &str,
    at: usize,
    what: impl FnOnce(String) -> String,
    fix: &str,
    out: &mut Vec<Problem>,
) -> Option<Scope> {
    match built {
        Ok(scope) => Some(scope),
        Err(e) => {
            out.push(problem_at(text, at, what(e), fix));
            None
        }
    }
}

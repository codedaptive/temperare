//! `rubocop_refusal`: why cctao will not run a rubocop check, if it will not.

/// The reason `check` is refused, naming its first comma-separated part that is not a full cop name; `None` when
/// every part is one. A full cop name is two or more `/`-separated segments, each an ASCII uppercase letter followed
/// by ASCII letters or digits (`Lint/UselessAssignment`, `RSpec/Capybara/CurrentPathExpectation`). So a department
/// (`Style`), an empty part, a part with spaces, an option (`-a`) and a wildcard are refused before running: a
/// department would run every cop in it, a set that changes from one rubocop version to the next, so a rule's check
/// names its cops.
pub(crate) fn rubocop_refusal(check: &str) -> Option<String> {
    let part = check.split(',').find(|part| !is_cop_name(part))?;
    Some(format!("rubocop check `{check}`: `{part}` is not a full cop name such as `Lint/UselessAssignment`"))
}

/// Two or more `/`-separated segments, each an ASCII uppercase letter then ASCII letters or digits.
fn is_cop_name(part: &str) -> bool {
    part.contains('/')
        && part.split('/').all(|segment| {
            segment.starts_with(|c: char| c.is_ascii_uppercase()) && segment.chars().all(|c| c.is_ascii_alphanumeric())
        })
}

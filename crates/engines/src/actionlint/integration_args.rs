//! `integration_args`: the ShellCheck and pyflakes integrations a check turns on.

/// For `check`: the program the run then needs on PATH, and the two integration arguments. `shellcheck` turns on
/// only ShellCheck and needs it; `pyflakes` only pyflakes and needs it; any other check turns both off and needs
/// neither. actionlint drops an integration's kind silently when its program is not found, so the flow looks the
/// program up first; the reader keeps only the rule's kind, so another check never needs either.
pub(crate) fn integration_args(check: &str) -> (Option<&'static str>, [&'static str; 2]) {
    match check {
        "shellcheck" => (Some("shellcheck"), ["-shellcheck=shellcheck", "-pyflakes="]),
        "pyflakes" => (Some("pyflakes"), ["-shellcheck=", "-pyflakes=pyflakes"]),
        _ => (None, ["-shellcheck=", "-pyflakes="]),
    }
}

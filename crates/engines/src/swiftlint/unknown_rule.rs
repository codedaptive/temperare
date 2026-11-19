//! `unknown_rule`: swiftlint's warning that the `--only-rule` id it was given is not a rule it knows.

use procio::Output;

/// The first line of `out`'s stderr that says "is not a valid rule identifier", trimmed; `None` when no line does.
/// swiftlint given an id it does not know still exits 0 and prints an empty JSON array, so this warning is the only
/// sign the run checked nothing. It is swiftlint's own verdict for that run, made with the rule's config applied (a
/// custom rule the config defines is known), so tm reads it rather than keeping a list of ids of its own.
pub(crate) fn unknown_rule(out: &Output) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| line.contains("is not a valid rule identifier"))
        .map(|line| line.trim().to_string())
}

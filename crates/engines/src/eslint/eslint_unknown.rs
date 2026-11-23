//! `eslint_unknown`: ESLint's words saying the loaded config defines no rule named by the check.

use procio::Output;

/// The words ESLint 10 puts in the line saying a rule is not defined: `Could not find "` for a rule its plugin lacks
/// (`Key "rules": Key "<check>": Could not find "<name>" in plugin "<p>".`, the core rules being plugin `@`);
/// `but could not find plugin "` for a plugin the config does not load (the CLI's own message, from
/// `eslint/messages/config-plugin-missing.js`: `A configuration object specifies rule "<check>", but could not find
/// plugin "<p>".`); and `Could not find plugin "`, the raw form of that error (`Key "rules": Key "<check>": Could
/// not find plugin "<p>" in configuration.`).
const NOT_DEFINED: [&str; 3] = ["Could not find \"", "but could not find plugin \"", "Could not find plugin \""];

/// The first stderr line of `out` that names `check` in double quotes and carries one of `NOT_DEFINED`, trimmed; None
/// when no line does. A line naming another rule (one the repository's config sets) is not about the check, so the
/// rule is never blamed for it and the run is left to the reader.
pub(crate) fn eslint_unknown(out: &Output, check: &str) -> Option<String> {
    let quoted = format!("\"{check}\"");
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| line.contains(&quoted) && NOT_DEFINED.iter().any(|words| line.contains(words)))
        .map(|line| line.trim().to_string())
}

//! `too_old_why`: the why of a rule whose check the installed engine does not know.

/// `<name> <version> does not know the check `<check>`: <said>`, with `(version unknown)` for the version when it was
/// not read; `said` is the engine's own words, or for clippy cctao's words for a name its help does not list.
pub(crate) fn too_old_why(name: &str, version: Option<&str>, check: &str, said: &str) -> String {
    format!("{name} {} does not know the check `{check}`: {said}", version.unwrap_or("(version unknown)"))
}

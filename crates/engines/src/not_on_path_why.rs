//! `not_on_path_why`: the why of a program not found on PATH.

/// `<program> not found on PATH`: the one text for a missing program, read by `stopped` for a rule's outcome and by
/// `availability_from` for an engine's availability, so the two never differ.
pub(crate) fn not_on_path_why(program: &str) -> String {
    format!("{program} not found on PATH")
}

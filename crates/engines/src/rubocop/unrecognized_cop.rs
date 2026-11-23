//! `unrecognized_cop`: rubocop's words saying a cop of the rule's check is one it does not know.

use procio::Output;

/// The first line of `out`'s stderr containing `Unrecognized cop or department: <part>.`, where `<part>` is one of
/// `check`'s comma-separated parts, trimmed; `None` when no line does. rubocop given `--only=Style/NoSuchCop` exits 2
/// and prints that line (then `Did you mean? ...`) after its usual stderr. The trailing `.` is part of the match, so
/// a check `Style/Foo` is never blamed for a line naming `Style/FooBar`.
pub(crate) fn unrecognized_cop(out: &Output, check: &str) -> Option<String> {
    let said: Vec<String> = check.split(',').map(|part| format!("Unrecognized cop or department: {part}.")).collect();
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| said.iter().any(|words| line.contains(words.as_str())))
        .map(|line| line.trim().to_string())
}

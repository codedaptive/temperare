//! `no_compile_database`: clang-tidy's words saying it found no compile database, read as why the rule is not
//! available.

use procio::Output;

/// None unless a line of `out`'s stderr, trimmed, is "Error while trying to load a compilation database:". clang-tidy
/// then checks the file without flags and still exits 0 with findings, which tm must not report: Some(why), naming the
/// file quoted on the stderr line that starts `Could not auto-detect compilation database for file "`, or "a file it
/// was given" when no line does.
pub(crate) fn no_compile_database(out: &Output) -> Option<String> {
    const FILE_LINE: &str = "Could not auto-detect compilation database for file \"";
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !stderr.lines().any(|line| line.trim() == "Error while trying to load a compilation database:") {
        return None;
    }
    let file = stderr
        .lines()
        .find_map(|line| line.strip_prefix(FILE_LINE)?.split_once('"').map(|(file, _)| file))
        .unwrap_or("a file it was given");
    Some(format!(
        "no compile database (compile_commands.json) or compile flags (compile_flags.txt) for {file} or any folder above it, so clang-tidy would check it without flags"
    ))
}

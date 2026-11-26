//! `probed_version`: an engine's version as tm reports it, read in the test because the version is machine-specific
//! and so cannot be written into the expected value.

use std::path::Path;

use crate::env::tm_env;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

/// The version `<program> <args>` prints, run in `dir` in the environment tm gets, `args` being the engine's own
/// version probe (engines' `VersionProbe`), read as engines' reader for that program reads it, written out here by
/// hand:
/// - `eslint` (`eslint_version`): stdout's first non-empty line, trimmed, one leading `v` stripped (`v10.15.0`);
/// - `markdownlint-cli2` (`markdownlint_version`, probe `--help`, which exits 2): on stdout's first non-empty line,
///   the word after a leading `markdownlint-cli2 v`, whatever the exit code;
/// - `pmd` (`pmd_version`): the second word of the first stdout line starting `PMD ` (its ASCII-art banner comes
///   first);
/// - any other program (`version_word`): on stdout's first non-empty line, the first word starting with an ASCII
///   digit, with a trailing `,` or `)` trimmed.
///
/// Panics when the probe exits other than 0 (markdownlint-cli2 aside) or prints no version that starts with an ASCII
/// digit.
pub fn probed_version(program: &str, args: &[&str], dir: &Path) -> String {
    let home = TempHome::new();
    let probe = run_clean(program, args, dir, tm_env(None, home.path()));
    let stdout = String::from_utf8_lossy(&probe.stdout).to_string();
    let call = format!("{program} {}", args.join(" "));
    if program != "markdownlint-cli2" {
        assert_eq!(probe.code, Some(0), "{call}: {stdout}");
    }
    let first = stdout.lines().find(|line| !line.trim().is_empty()).unwrap_or("");
    let version = match program {
        "eslint" => first.trim().strip_prefix('v').or(Some(first.trim())),
        "markdownlint-cli2" => {
            first.trim_start().strip_prefix("markdownlint-cli2 v").and_then(|rest| rest.split_whitespace().next())
        }
        "pmd" => stdout.lines().find(|line| line.starts_with("PMD ")).and_then(|line| line.split_whitespace().nth(1)),
        _ => first
            .split_whitespace()
            .find(|w| w.starts_with(|c: char| c.is_ascii_digit()))
            .map(|w| w.trim_end_matches([',', ')'])),
    };
    version
        .filter(|v| v.starts_with(|c: char| c.is_ascii_digit()))
        .map(str::to_string)
        .unwrap_or_else(|| panic!("no version in {call}: {stdout}"))
}

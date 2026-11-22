//! `probed_version`: an engine's version as tm reports it, read in the test because the version is machine-specific
//! and so cannot be written into the expected value.

use std::path::Path;

use crate::env::tm_env;
use crate::run_clean::run_clean;
use crate::temp_home::TempHome;

/// The version word of `<program> <args>` run in `dir` in the environment tm gets, `args` being the engine's own
/// version probe (engines' `VersionProbe`: `--version` for most, `version` for Periphery), read as engines'
/// `version_word` reads it: on stdout's first non-empty line, the first word starting with an ASCII digit, with a
/// trailing `,` or `)` trimmed. Panics when the probe does not exit 0 or prints no such word.
pub fn probed_version(program: &str, args: &[&str], dir: &Path) -> String {
    let home = TempHome::new();
    let probe = run_clean(program, args, dir, tm_env(None, home.path()));
    let stdout = String::from_utf8_lossy(&probe.stdout).to_string();
    let call = format!("{program} {}", args.join(" "));
    assert_eq!(probe.code, Some(0), "{call}: {stdout}");
    stdout
        .lines()
        .find(|line| !line.trim().is_empty())
        .and_then(|line| line.split_whitespace().find(|w| w.starts_with(|c: char| c.is_ascii_digit())))
        .map(|w| w.trim_end_matches([',', ')']).to_string())
        .unwrap_or_else(|| panic!("no version word in {call}: {stdout}"))
}

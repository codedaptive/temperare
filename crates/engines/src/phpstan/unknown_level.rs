//! `unknown_level`: PHPStan's words saying it has no level the rule's check names.

use procio::Output;

/// The first stderr line of `out` that, trimmed, starts `Level config file ` and ends
/// `config.level{check}.neon was not found.`, trimmed; None when there is none. PHPStan 2.3.7 given `--level=11`
/// prints only that line (naming its own phar's `conf/config.level11.neon`) and exits 1. A line naming another level
/// is not about the rule's check, so it is left alone.
pub(crate) fn unknown_level(out: &Output, check: &str) -> Option<String> {
    let end = format!("config.level{check}.neon was not found.");
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("Level config file ") && line.ends_with(&end))
        .map(str::to_string)
}

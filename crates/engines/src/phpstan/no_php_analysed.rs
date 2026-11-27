//! `no_php_analysed`: PHPStan saying no file was left for it to analyse.

use procio::Output;

/// Whether some stderr line of `out`, trimmed, ends with `No files found to analyse.`. PHPStan 2.3.7 prints
/// ` [ERROR] No files found to analyse.` (padded with spaces), with exit 1, when no file it was given is left to
/// analyse. With cctao's wrapper emptying `excludePaths`, that is the config's `stubFiles` naming every file given: a
/// stub is read for its types, never analysed. A `fileExtensions` without `php` does not do it, since PHPStan applies
/// it only to the folders it walks. Only that sentence is matched, since its stderr carries other blocks.
pub(crate) fn no_php_analysed(out: &Output) -> bool {
    String::from_utf8_lossy(&out.stderr).lines().any(|line| line.trim().ends_with("No files found to analyse."))
}

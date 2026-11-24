//! `detekt_output`: one detekt run's output, reshaped so its SARIF report sits where the reader reads it.

use procio::Output;

/// `out` with stdout replaced by the SARIF report file's bytes (empty when detekt wrote none), and stderr by detekt's
/// console stdout followed by its stderr; the exit code is unchanged. detekt writes SARIF only to a file and prints its
/// findings and its config errors on stdout, so this is the shape the shared readers take:
/// `parse_detekt` reads the SARIF log from stdout through `parse_sarif`, and `unknown_rule` and `parse_failed` find
/// detekt's own words in stderr. `check_run` counts this Output, which holds everything detekt produced (console and
/// report), so the run needs no `raw_chars` of its own.
pub(crate) fn detekt_output(out: Output, sarif: Option<Vec<u8>>) -> Output {
    let mut stderr = out.stdout;
    stderr.extend_from_slice(&out.stderr);
    Output { code: out.code, stdout: sarif.unwrap_or_default(), stderr }
}

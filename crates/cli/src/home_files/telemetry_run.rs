//! `TelemetryRun`: the run's own fields in a telemetry line.
//! A plain struct: its fields are proven through telemetry_line_tests.rs.

use std::path::Path;

/// What one `cctao check` was asked and how it ended, as its telemetry line carries it.
pub(crate) struct TelemetryRun<'a> {
    /// The rule file's folder, absolute.
    pub(crate) folder: &'a Path,
    /// The base commit id; None without a base.
    pub(crate) base: Option<&'a str>,
    /// The `--kind` asked for; None without one.
    pub(crate) kind: Option<rules::Kind>,
    /// Each path argument, under the rule file's folder; empty for the whole tree.
    pub(crate) paths: &'a [rules::RelPath],
    /// The exit code.
    pub(crate) exit: u8,
}

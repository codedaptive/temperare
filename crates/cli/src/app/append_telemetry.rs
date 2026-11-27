//! `append_telemetry`: one `tm check`'s line in the telemetry file of its repository, under HOME.
//! Tested in integration/run_check_home_tests.rs
//! (a_check_appends_one_telemetry_line_named_after_the_root_and_prints_the_same) and tests/world/telemetry_real.rs.

use super::first_parent_root::first_parent_root;
use super::CheckWorld;
use crate::home_files::{append_telemetry_line, telemetry_line, TelemetryRun};
use crate::output::version_after_name;
use report::Report;
use std::path::Path;

/// Appends `run` and `report`'s line to the telemetry file under `home` named after HEAD's first-parent root, asked
/// of git at the repository top of the rule file's folder (`HEAD`, then `HEAD~n`), the time from `world.now`. With
/// no HOME (None or empty) git is not asked. No repository top, an unborn HEAD, a root git cannot give or a failed
/// write: no line. Nothing is printed; every failure is silent.
pub(crate) fn append_telemetry(run: &TelemetryRun, report: &Report, home: Option<&Path>, world: &CheckWorld) {
    let Some(home) = home.filter(|home| !home.as_os_str().is_empty()) else {
        return;
    };
    let Ok(top) = (world.toplevel)(run.folder) else {
        return;
    };
    let at = |n: u64| (world.rev_parse)(&top, &if n == 0 { "HEAD".to_string() } else { format!("HEAD~{n}") });
    let Some((head, root)) = first_parent_root(&at) else {
        return;
    };
    let line = telemetry_line(
        run,
        (world.now)(),
        &version_after_name(env!("CARGO_PKG_VERSION"), env!("TEMPERARE_COMMIT")),
        head.as_str(),
        report,
    );
    append_telemetry_line(Some(home), root.as_str(), &line);
}

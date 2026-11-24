//! `configs_stop`: why a stylelint rule stops on the configs its files are given, if it does.

use super::printed_config;
use crate::types::Stop;
use procio::Output;

/// For each of `files` (relative to the rule's folder) in order, `probe(file)` (stylelint's `--print-config` for it)
/// read by `printed_config`; the first `Stop` either gives (a probe that could not run, a config that does not load,
/// turns on fix or quiet) is the stop, and no later file is probed. Every file is probed, since nested configs and
/// `overrides` give each file its own. After them all: no file has `check` on is `NotAvailable` (the config never
/// turns it on, so a run would be silent, never clean); some on and some off is `Failed` naming the first file it is
/// off for (an exception hidden in the config); all on is `None`.
pub(crate) fn configs_stop(files: &[String], check: &str, probe: &dyn Fn(&str) -> Result<Output, Stop>) -> Option<Stop> {
    let mut any_on = false;
    let mut first_off = None;
    for file in files {
        match probe(file).and_then(|out| printed_config(&out, check, file)) {
            Ok(true) => any_on = true,
            Ok(false) => first_off = first_off.or(Some(file)),
            Err(stop) => return Some(stop),
        }
    }
    match (any_on, first_off) {
        (false, _) => Some(Stop::NotAvailable {
            why: format!("stylelint's config does not turn on {check} for any file the rule covers"),
        }),
        (true, Some(file)) => Some(Stop::Failed {
            code: None,
            stderr: format!("stylelint's config turns {check} off for {file}; exceptions go in the rule file"),
        }),
        (true, None) => None,
    }
}

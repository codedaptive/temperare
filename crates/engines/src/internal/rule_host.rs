//! `rule_host`: the host of one rule of a shared group.

use crate::host::Host;
use procio::{Cmd, Output, ProcError};
use std::cell::Cell;

/// A host for one rule of a shared group (`run_group`): `run` and `raw_chars` its own, so its check run can be shared
/// and its count kept apart; `on_path` and `base` the check's `host`, so the rule sees what it would see alone.
pub(crate) fn rule_host<'a>(
    host: &Host<'a>,
    run: &'a dyn Fn(&Cmd) -> Result<Output, ProcError>,
    raw_chars: &'a Cell<Option<usize>>,
) -> Host<'a> {
    Host { run, on_path: host.on_path, raw_chars, base: host.base }
}

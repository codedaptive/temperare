//! `answered`: a SwiftPM step's output, or why it stopped the rule. It runs a program, so its test is the zone
//! integration test tests/periphery.rs and it has no unit-test sibling.

use crate::host::Host;
use crate::ran::ran;
use crate::types::Stop;
use procio::{Cmd, Output};

/// The output of a SwiftPM step that exited 0; any other exit is `Failed` with its code and its stderr, or its stdout
/// when stderr is empty (SwiftPM prints some build errors there).
pub(crate) fn answered(cmd: &Cmd, host: &Host) -> Result<Output, Stop> {
    let out = ran(cmd, host)?;
    if out.code == Some(0) {
        return Ok(out);
    }
    let printed = if out.stderr.is_empty() { &out.stdout } else { &out.stderr };
    Err(Stop::Failed { code: out.code, stderr: String::from_utf8_lossy(printed).into_owned() })
}

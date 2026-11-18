//! `run_install`: `temperare install` from the host and the flags to the exit code (S-32).
//!
//! `failed`, the two lines it writes on a failure, is unit-tested in the sibling run_install_tests.rs, and the `next:`
//! line for a record that is not written in unrecorded_next_tests.rs; the disk steps it runs are covered by the zone
//! integration target `tests/install` on a temporary HOME. Its decisions are the pure plan (install/plan/), which has
//! its own unit tests.

use std::io::{self, Write};
use std::path::Path;

use crate::install::{
    apply_install, choose_targets, detect_harnesses, make_dirs, parse_record, plan_install, read_record_text,
    record_text, tm_owner, unrecordable_path, write_through_temp,
};
use crate::{Harness, InstallArgs, InstallHost, InstallPaths, RecordEntry};

use super::unrecorded_next::unrecorded_next;

/// Chooses the harnesses to wire (`pick` asks at a terminal), plans the install, prints the plan's notes on `out` and
/// carries it out, one line per step on `out`. A path to place or record that is not valid UTF-8 or holds a newline, a
/// carriage return or a tab (`unrecordable_path`) is refused before any write. The record at
/// `.local/state/temperare/installed` is read through `read_record_text` before any step, so a symbolic link or
/// anything else that is not a plain file there refuses the install with nothing placed. The record then gains what
/// this run placed, merged with what an earlier install recorded, and is written through `write_through_temp` even
/// when a step fails, so uninstall still finds everything placed; a link made at the record path after the read is
/// replaced, its target untouched. When that write fails, the record on disk is left as it was, and the `next:` line
/// names what this run added that it does not (`unrecorded_next`; the error shown is the failed step's, else the
/// write's). Returns 0, skips included; 1 when the install is refused, the record cannot be read or written, or a
/// disk step fails, with the `failed` text on `err`. Write errors on `out` and `err` are ignored.
pub fn run_install(
    args: &InstallArgs,
    host: &InstallHost,
    pick: &mut dyn FnMut(&[Harness]) -> Vec<Harness>,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> u8 {
    let paths = InstallPaths::under(&host.home);
    let detected = detect_harnesses(&host.home);
    let (targets, mut notes) = choose_targets(args, &detected, host.interactive, pick);
    let owner = tm_owner(host.path_var.as_deref(), &host.cwd, &paths.link, &[host.exe.clone(), paths.binary.clone()]);
    let plan = plan_install(args, host, &targets, owner.as_deref());
    if let Some(path) = unrecordable_path(&plan, &paths.record) {
        return failed(
            &format!(
                "the path {path:?} is not valid UTF-8 or holds a newline, carriage return or tab, which the install \
                 record cannot hold"
            ),
            "Use a HOME, project folder and binary path that are valid UTF-8 with no newline, carriage return or tab; \
             then run temperare install again.",
            err,
        );
    }
    notes.extend(plan.notes.iter().cloned());
    for note in &notes {
        let _ = writeln!(out, "{note}");
    }
    let mut placed = match read_record(&paths.record) {
        Ok(placed) => placed,
        Err(e) => return failed(&e.to_string(), UNREAD, err),
    };
    let earlier = placed.clone();
    let applied =
        make_dirs(&paths.state_dir, &mut placed).and_then(|()| apply_install(&plan, &host.exe, &mut placed, out));
    let written = write_through_temp(&paths.record, &mut record_text(&placed).as_bytes(), false);
    match (applied, written) {
        (Ok(()), Ok(())) => 0,
        (Err(e), Ok(())) => failed(&e.to_string(), AGAIN, err),
        (Err(e), Err(_)) | (Ok(()), Err(e)) => failed(&e.to_string(), &unrecorded_next(&earlier, &placed), err),
    }
}

/// The entries of the record at `record`; none when there is no record yet.
fn read_record(record: &Path) -> io::Result<Vec<RecordEntry>> {
    Ok(read_record_text(record)?.map_or_else(Vec::new, |text| parse_record(&text)))
}

/// The `next:` line when the record cannot be read, before any step.
const UNREAD: &str = "Fix the cause above and run temperare install again; nothing was placed.";

/// The `next:` line after a disk step fails and the record holding what was placed is written.
const AGAIN: &str = "Fix the cause above and run temperare install again; temperare uninstall removes what was placed.";

/// Writes `tm: cannot install: <what>` and `next: <next>` on `err`; returns 1.
pub(super) fn failed(what: &str, next: &str, err: &mut dyn Write) -> u8 {
    let _ = writeln!(err, "tm: cannot install: {what}\nnext: {next}");
    1
}

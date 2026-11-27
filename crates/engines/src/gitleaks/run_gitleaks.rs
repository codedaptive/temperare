//! `run_gitleaks`: gitleaks' flow for one rule: a guard against exceptions outside the rule file, then the standard
//! flow with the rule's config kept and stderr cut to errors.

use super::{error_lines, suppression_why};
use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::Rule;
use std::cell::Cell;
use std::ffi::OsString;
use std::path::Path;

/// First the guard: when `suppression_why(folder, rule.config)` finds an allowlist in the config gitleaks would read
/// (or a file it extends), such a config that does not parse, or a `.gitleaksignore` that lists anything, the rule is
/// `Failed` with no exit code and that why, and nothing runs. Refusal and availability settle before any flow, so a
/// refused check is `Failed` and an absent gitleaks is `NotOnPath`, whatever the config holds; an unknown rule id
/// under such a config reads `Failed`, since gitleaks never runs to say so.
///
/// Then `run_plain` over a host whose `run` removes GITLEAKS_CONFIG and GITLEAKS_CONFIG_TOML from every command, so a
/// caller's environment cannot replace the rule's config, and replaces the stderr of the Output `host.run` returns
/// with its `error_lines`, so every stderr cctao can print for the rule (a Failed rule's, an unknown id's words) holds
/// error lines only. The run is counted raw: `check_run` counts the cleaned Output, so once `run_plain` returns, whatever
/// its result, `host.raw_chars` is set to the raw stdout's plus raw stderr's characters (lossy UTF-8, as `check_run`
/// counts) when the command ran. `on_path` and `base` are `host`'s.
pub(crate) fn run_gitleaks(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    if let Some(stderr) = suppression_why(folder, rule.config.as_deref()) {
        return Err(Stop::Failed { code: None, stderr });
    }
    let raw = Cell::new(None);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let mut cmd = cmd.clone();
        cmd.env_remove.extend(["GITLEAKS_CONFIG", "GITLEAKS_CONFIG_TOML"].map(OsString::from));
        let out = (host.run)(&cmd)?;
        raw.set(Some(chars(&out.stdout) + chars(&out.stderr)));
        Ok(Output { stderr: error_lines(&out.stderr), ..out })
    };
    let got = run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base });
    if let Some(count) = raw.get() {
        host.raw_chars.set(Some(count));
    }
    got
}

fn chars(bytes: &[u8]) -> usize {
    String::from_utf8_lossy(bytes).chars().count()
}

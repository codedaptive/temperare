//! `run_plain`: the standard flow of one rule: one check run built by `entry_cmd`, read by the entry's reader.

use super::{entry_cmd, param_args, param_why, temp_config, threshold_run, Entry, Pick};
use crate::check_run::check_run;
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::ffi::OsStr;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical), each step able to stop the rule:
/// 1. a check the entry refuses is `Failed` with no exit code and the refusal as stderr; nothing runs;
/// 2. params `param_args` cannot use are `Failed` with no exit code and `param_why`'s text;
/// 3. a temporary config `temp_config` cannot make is `Failed` with no exit code and its text;
/// 4. a threshold `threshold_run` refuses or cannot write is `Failed` with no exit code and its text;
/// 5. the check run, through `check_run` so it is counted: `entry_cmd` with the rule's check as the pick, the param
///    arguments then the config argument, and the rule's config, in `folder`. On a threshold run the threshold's
///    arguments follow, `entry_cmd` gets no config (the threshold's arguments carry the rule's config, ahead of the
///    threshold) and the threshold's environment is added to the command's. An entry that picks by check file has
///    a flow of its own, so here it is `Failed` with no exit code, and nothing runs. The temporary config and the
///    threshold's file or folder are removed right after the run, whatever its outcome;
/// 6. output in which the engine says it does not know the rule's check (`entry.unknown`) is `UnknownCheck` with its
///    words;
/// 7. output the reader cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_plain(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    if let Some(why) = (entry.refuse)(&rule.check) {
        return Err(failed(why));
    }
    let mut args = param_args(entry, &rule.params).map_err(|err| failed(param_why(entry.name, err)))?;
    let config = temp_config(entry, &rule.params).map_err(failed)?;
    let threshold = threshold_run(entry, rule, folder).map_err(failed)?;
    let pick = match entry.pick {
        Pick::Arg(_) | Pick::None => OsStr::new(&rule.check),
        Pick::File(_) => return Err(failed(format!("{} has no standard run for a check file", entry.name))),
    };
    args.extend(config.as_ref().map(|(_, arg)| arg.clone()));
    let rule_config = match &threshold {
        Some(threshold) => {
            args.extend_from_slice(&threshold.args);
            None
        }
        None => rule.config.as_deref(),
    };
    let mut cmd = entry_cmd(entry, pick, &args, rule_config, folder);
    cmd.env.extend(threshold.iter().flat_map(|threshold| threshold.env.iter().cloned()));
    let ran = check_run(&cmd, host);
    drop(config);
    drop(threshold);
    let out = ran?;
    if let Some(said) = (entry.unknown)(&out, &rule.check) {
        return Err(Stop::UnknownCheck { said });
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}

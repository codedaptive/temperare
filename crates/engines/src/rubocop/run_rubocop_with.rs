//! `run_rubocop_with`: rubocop's flow for one rule, reading the folder's files through a given function: the check
//! run, then the probes that make a rule `Failed` when the repository's rubocop config hides a file or a cop from it.

use super::{cop_excludes, inspected_files, left_out, rubocop_probe, todo_why, with_rubocop_env};
use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::Rule;
use std::cell::RefCell;
use std::path::Path;

/// The hits of `rule` under `folder`, the first of these steps that stops deciding:
/// 1. `read` gives the text of `folder/.rubocop`: `Failed` with no exit code, and nothing runs. rubocop reads that
///    file as extra command-line options, which could change what it checks or prints.
/// 2. `run_plain` over a host whose `run` adds `with_rubocop_env`'s environment and keeps the stdout of the command
///    carrying `--only=<check>`, the check run (the only command `run_plain` runs). Its `Stop` is returned.
/// 3. `rubocop --show-cops=<check> --debug [--config=<config>]`: `todo_why` of what it prints is `Failed`.
/// 4. When that dump gives a cop of `check` an `Exclude` list, `rubocop --show-cops=<check> --force-default-config`
///    prints the stock lists; a cop whose list holds an entry the stock one does not is `Failed`, naming the first.
///    An Exclude equal to the stock one (Style/Documentation's spec and test folders) is stock scope, not an exception.
/// 5. `rubocop --list-target-files --force-default-config .`: `left_out` of what it prints, against the check run's
///    `inspected_files`, is `Failed`.
/// 6. No file the check run inspected is one `rule` applies to: `NoFiles`. rubocop's own config decides which files it
///    inspects, so this is settled from its answer, not from a list of extensions before the run.
/// 7. Else the check run's hits.
///
/// The probes (3 to 5) run through `rubocop_probe` with `host` itself: they are not counted as the check run, and they
/// never touch the kept stdout. Any probe that exits other than 0 is `Failed` with its exit code and stderr.
pub(crate) fn run_rubocop_with(
    read: &dyn Fn(&Path) -> Option<String>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let options = folder.join(".rubocop");
    if read(&options).is_some() {
        return Err(failed(format!(
            "{} gives rubocop options of its own; tm runs rubocop with its own options only",
            options.display()
        )));
    }
    let only = format!("--only={}", rule.check);
    let kept = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let out = (host.run)(&with_rubocop_env(cmd.clone()))?;
        if cmd.args.iter().any(|arg| arg == only.as_str()) {
            kept.replace(out.stdout.clone());
        }
        Ok(out)
    };
    let hits = run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })?;
    let show = format!("--show-cops={}", rule.check);
    let config = rule.config.as_ref().map(|config| format!("--config={config}"));
    let mut args = vec![show.as_str(), "--debug"];
    args.extend(config.as_deref());
    let repo = rubocop_probe(&args, folder, host)?;
    if let Some(why) = todo_why(&repo, &rule.check, read) {
        return Err(failed(why));
    }
    beyond_stock(&repo, &show, rule, folder, host)?;
    let inspected = inspected_files(&kept.into_inner());
    let stock_list = rubocop_probe(&["--list-target-files", "--force-default-config", "."], folder, host)?;
    if let Some(path) = left_out(&stock_list, &inspected, rule) {
        return Err(failed(format!(
            "the repository's rubocop config (AllCops Exclude or Include) leaves out {path}, a file in the rule's paths; exceptions go in the rule file"
        )));
    }
    if !inspected.iter().any(|path| rule.applies(path)) {
        return Err(Stop::NoFiles { why: "no file rubocop inspects is in the rule's paths".to_string() });
    }
    Ok(hits)
}

/// Step 4: `Failed` for the first cop of `rule`'s check whose `Exclude` in `repo` (the repository's dump) holds an
/// entry the stock dump's does not. The stock dump is run only when some cop has an `Exclude` at all.
fn beyond_stock(repo: &str, show: &str, rule: &Rule, folder: &Path, host: &Host) -> Result<(), Stop> {
    let cops: Vec<&str> = rule.check.split(',').collect();
    if cops.iter().all(|cop| cop_excludes(repo, cop).is_empty()) {
        return Ok(());
    }
    let stock = rubocop_probe(&[show, "--force-default-config"], folder, host)?;
    for cop in cops {
        let stock_excludes = cop_excludes(&stock, cop);
        if let Some(extra) = cop_excludes(repo, cop).into_iter().find(|entry| !stock_excludes.contains(entry)) {
            return Err(failed(format!(
                "the repository's rubocop config excludes {extra} from {cop}, beyond rubocop's stock Exclude; exceptions go in the rule file"
            )));
        }
    }
    Ok(())
}

fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}

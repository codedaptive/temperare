//! `run_stylelint_with`: stylelint's flow for one rule, given the files a lister keeps: each file's config probed, then
//! one check run over them.

use super::{configs_stop, parse_stylelint, stylelint_report};
use crate::entry::{run_plain, Entry, TempFile};
use crate::host::Host;
use crate::no_script::no_script;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, Rule};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical), each step able to stop it:
/// 1. the files `sources` lists for `folder` (its `Stop` as it is), kept when `rule` applies to them; when `no_script`
///    finds none the rule applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1
///    (`engine_listed`) settles that before availability; this guard decides only when that listing failed and this
///    one succeeds;
/// 2. every kept file's config, by `configs_stop` over a probe running `stylelint --print-config
///    [--config=<rule's config>] ./<file>` in `folder` through `ran`, so it is not counted: a config that does not load,
///    turns on fix or quiet, or turns the check off for some files stops the rule before the check run;
/// 3. an empty suppressions file (`{}`) written for the run, so stylelint never reads the folder's
///    `stylelint-suppressions.json`; an error writing it is `Failed` with no exit code;
/// 4. `run_plain` over a host whose `run` appends `--suppress-location=<that file>` and `./<file>` for each kept file
///    to the command and runs it with `host.run`; `on_path`, `raw_chars` and `base` are `host`'s. The file is removed
///    right after, whatever the outcome.
///
/// A `Failed` from `run_plain` whose run printed a report holds a file whose result is no answer (ignored, rewritten,
/// not parsed, an invalid option), and its stderr also holds Node's warnings and the whole report, so its stderr is
/// `parse_stylelint`'s why, naming the file. Every other stop passes through unchanged.
pub(crate) fn run_stylelint_with(
    sources: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let listed = sources(folder)?;
    if let Some(stop) = no_script(rule, &listed) {
        return Err(stop);
    }
    let files: Vec<String> = listed
        .iter()
        .map(|file| file.to_string_lossy().into_owned())
        .filter(|file| rule.applies(&RelPath::new(file)))
        .collect();
    let probe = |file: &str| {
        let mut args = vec![OsString::from("--print-config")];
        args.extend(entry.config.zip(rule.config.as_deref()).map(|(option, config)| OsString::from(format!("{option}{config}"))));
        args.push(OsString::from(format!("./{file}")));
        ran(&Cmd { args, cwd: Some(folder.to_path_buf()), ..Cmd::new(entry.program) }, host)
    };
    if let Some(stop) = configs_stop(&files, &rule.check, &probe) {
        return Err(stop);
    }
    let suppressions = TempFile::write("stylelint-suppressions", "json", "{}").map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot write stylelint's empty suppressions file: {err}"),
    })?;
    let mut location = OsString::from("--suppress-location=");
    location.push(&suppressions.path);
    let tail: Vec<OsString> =
        std::iter::once(location).chain(files.iter().map(|file| OsString::from(format!("./{file}")))).collect();
    let seen: RefCell<Option<Output>> = RefCell::new(None);
    let run = |cmd: &Cmd| {
        let mut cmd = cmd.clone();
        cmd.args.extend(tail.iter().cloned());
        let got = (host.run)(&cmd);
        *seen.borrow_mut() = got.as_ref().ok().cloned();
        got
    };
    let got = run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base });
    drop(suppressions);
    match (got, seen.into_inner()) {
        (Err(Stop::Failed { code, stderr }), Some(out)) if stylelint_report(&out).is_some() => {
            let stderr = parse_stylelint(&out, &rule.check, folder, folder).err().map_or(stderr, |err| err.why);
            Err(Stop::Failed { code, stderr })
        }
        (got, _) => got,
    }
}

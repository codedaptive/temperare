//! `run_pmd_with`: PMD's flow for one rule, the Java files it checks listed through a given lister and handed over in
//! a file list.

use crate::entry::{run_plain, Entry, TempFile};
use crate::host::Host;
use crate::no_script::no_script;
use crate::types::{Hit, Stop};
use procio::Cmd;
use rules::{RelPath, Rule};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The files `files` lists under `folder` (its `Stop` when it fails); `no_script`'s `NoFiles` when none is one `rule`
/// applies to, and nothing runs (`engine_listed` settles that before availability; this decides only when that listing
/// failed and this one succeeds). A kept file whose path holds `,` makes the rule `Failed` with no exit code, naming
/// the first, and nothing runs: PMD reads `,` in a file list as a separator (fixtures/fixtures.txt). Those `rule`
/// applies to are written one per line, relative to `folder`, to a `TempFile`; a write error is `Failed` with no exit
/// code. Then `run_plain` over a host whose `run` appends `--file-list=<that file>` to the command and removes
/// CLASSPATH from its environment (PMD's launcher puts CLASSPATH ahead of PMD's own rulesets, so a caller's could
/// replace them), and runs it with `host.run`; the file lives until `run_plain` returns. `on_path`, `raw_chars` and
/// `base` are `host`'s: the output is not altered.
pub(crate) fn run_pmd_with(
    files: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let listed = files(folder)?;
    if let Some(stop) = no_script(rule, &listed) {
        return Err(stop);
    }
    let kept: Vec<_> = listed.iter().map(|file| file.to_string_lossy()).filter(|file| rule.applies(&RelPath::new(file))).collect();
    if let Some(file) = kept.iter().find(|file| file.contains(',')) {
        let stderr = format!("{file}: pmd reads ',' in --file-list as a separator, so tm cannot hand it this file");
        return Err(Stop::Failed { code: None, stderr });
    }
    let lines: String = kept.iter().map(|file| format!("{file}\n")).collect();
    let list = TempFile::write("pmd-files", "txt", &lines)
        .map_err(|err| Stop::Failed { code: None, stderr: format!("cannot write PMD's file list: {err}") })?;
    let run = |cmd: &Cmd| {
        let mut cmd = cmd.clone();
        let mut arg = OsString::from("--file-list=");
        arg.push(&list.path);
        cmd.args.push(arg);
        cmd.env_remove.push(OsString::from("CLASSPATH"));
        (host.run)(&cmd)
    };
    run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })
}

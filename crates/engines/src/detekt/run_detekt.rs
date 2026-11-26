//! `run_detekt`: detekt's flow for one rule. No unit-test sibling, as clang_tidy/run_clang_tidy.rs: it runs git and
//! the engine and reads the files it writes, so tests/detekt.rs covers it.

use super::{detekt_args, detekt_layer, detekt_output, detekt_unlisted, kotlin_files, typed_why};
use crate::entry::{run_plain, Entry, TempFile};
use crate::failed::failed;
use crate::host::Host;
use crate::no_script::no_script;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, Rule};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// The JVM's own option variables. A caller's could load an agent into detekt's JVM or change what it prints.
const JAVA_OPTIONS: [&str; 3] = ["JAVA_TOOL_OPTIONS", "JDK_JAVA_OPTIONS", "_JAVA_OPTIONS"];

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `DETEKT`. The central `refused` has already
/// settled the rule's refusals and params (detekt takes none), and availability is settled before the flow, so an
/// absent detekt reads not on PATH whatever the well-formed check. Each step can stop the rule:
/// 1. the Kotlin sources `kotlin_files` lists under `folder`, or its `Stop`; when `no_script` finds none the rule
///    applies to, nothing runs and the rule is its `NoFiles`. `engine_hits_with`'s step 1 (`no_file_listed`) settles
///    that before availability; this guard decides only when that listing failed and this one succeeds;
/// 2. a check with a part that is neither a rule nor a rule set of temperare's list is `detekt_unlisted`'s
///    `NotListed`, naming the part and the list's version; `stopped` adds the engine and its version. Nothing runs
///    for it, so a name tm cannot pick is never clean;
/// 3. a check that picks any rule needing type resolution, by its name or through its set, is `NotAvailable` with
///    `typed_why`'s why, and nothing runs: tm passes detekt no classpath, so such a rule would report nothing;
/// 4. the config: the rule's `config` joined to `folder`, else `folder`/detekt.yml when it is a file, else none;
/// 5. tm's layer (`detekt_layer`) and an empty report file, each a `TempFile` (a write error is `Failed` with no exit
///    code), both removed when the flow returns, after the run;
/// 6. `detekt_args` for the sources the rule applies to, the config, the layer and the report; its Err is `Failed`
///    with no exit code, and nothing runs;
/// 7. `run_plain` over a host whose `run` appends those arguments to the command, removes JAVA_TOOL_OPTIONS,
///    JDK_JAVA_OPTIONS and _JAVA_OPTIONS from its environment, runs it with `host.run`, then reads the report file
///    (a read error or an empty file is no report) and gives `detekt_output` of the two. JAVA_HOME is kept: a detekt
///    that is not Homebrew's needs it to find its Java. `on_path`, `raw_chars` and `base` are `host`'s.
pub(crate) fn run_detekt(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let files = kotlin_files(folder)?;
    if let Some(stop) = no_script(rule, &files) {
        return Err(stop);
    }
    if let Some(stop) = detekt_unlisted(&rule.check) {
        return Err(stop);
    }
    if let Some(why) = typed_why(&rule.check) {
        return Err(Stop::NotAvailable { why });
    }
    let kept: Vec<PathBuf> =
        files.into_iter().filter(|file| rule.applies(&RelPath::new(&file.to_string_lossy()))).collect();
    let config = match &rule.config {
        Some(config) => Some(folder.join(config)),
        None => Some(folder.join("detekt.yml")).filter(|path| path.is_file()),
    };
    let layer = TempFile::write("detekt", "yml", &detekt_layer(&rule.check)).map_err(|err| failed(format!("cannot write detekt's config layer: {err}")))?;
    let report = TempFile::write("detekt-report", "sarif", "").map_err(|err| failed(format!("cannot make detekt's report file: {err}")))?;
    let args = detekt_args(&kept, config.as_deref(), &layer.path, &report.path).map_err(failed)?;
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let mut cmd = cmd.clone();
        cmd.args.extend(args.iter().cloned());
        cmd.env_remove.extend(JAVA_OPTIONS.map(OsString::from));
        let out = (host.run)(&cmd)?;
        let sarif = std::fs::read(&report.path).ok().filter(|bytes| !bytes.is_empty());
        Ok(detekt_output(out, sarif))
    };
    run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base })
}

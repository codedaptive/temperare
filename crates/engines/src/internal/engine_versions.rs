//! `engine_versions`: the version of each engine a rule file names, each probed once.

use crate::availability::availability_from;
use crate::engine_version::engine_version;
use crate::entry_of::entry_of;
use crate::host::Host;
use crate::types::{Availability, EngineVersion};
use rules::{Engine, RuleFile};
use std::path::Path;

/// Each distinct engine of `file.rules`, once, in the order the rules first name it, with its version. The version is
/// read (`engine_version`, one probe per engine) only when `availability_from` says the engine is available and
/// `folder`, the rule file's canonical folder, resolved; otherwise it is `None` and nothing runs. The probes do not
/// touch `host.raw_chars`.
pub(crate) fn engine_versions(file: &RuleFile, folder: Option<&Path>, host: &Host) -> Vec<EngineVersion> {
    let mut engines: Vec<Engine> = Vec::new();
    for rule in &file.rules {
        if !engines.contains(&rule.engine) {
            engines.push(rule.engine);
        }
    }
    engines
        .into_iter()
        .map(|engine| EngineVersion { engine, version: version(engine, folder, host) })
        .collect()
}

/// `engine`'s version when it is available and the folder resolved; `None` otherwise, with nothing run.
fn version(engine: Engine, folder: Option<&Path>, host: &Host) -> Option<String> {
    if availability_from(engine, folder, host.on_path) != Availability::Available {
        return None;
    }
    engine_version(entry_of(engine), folder?, host)
}

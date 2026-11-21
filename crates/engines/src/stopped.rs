//! `stopped`: the outcome of a rule that stopped without findings, a too-old check and a check not in temperare's
//! list of its engine's codes naming the engine's version.

use crate::entry::too_old_why;
use crate::not_on_path_why::not_on_path_why;
use crate::types::{EngineVersion, Stop};
use rules::{Engine, Rule, RuleOutcome};

/// The outcome of `rule` when it stopped without findings. `NotAvailable` and `NoCargoProject` are `NotAvailable`
/// with their why as it is;
/// `NotOnPath` is `NotAvailable` with `not_on_path_why` naming the program (`rule_run_of` sets the cause from the
/// stop, `not_available_cause_of`). A check the engine said it does not know is
/// `NotAvailable`, with `too_old_why` naming the engine by its rule-file name (`Engine::name`) and its version, the
/// first of `versions` for the rule's engine (`(version unknown)` when there is none or it is `None`). A check not in
/// temperare's list of its engine's codes (`NotListed`) is `NotAvailable` with `<engine> <version>: ` before what it
/// said, the engine and version found the same way. `NoFiles` is
/// `Findings` with none: `rule_run_of` settles a `NoFiles` itself, setting the rule's `no_files` flag, before it
/// would reach here, so this arm only keeps the match whole.
pub(crate) fn stopped(rule: &Rule, stop: Stop, versions: &[EngineVersion]) -> RuleOutcome {
    let (id, engine) = (rule.id.clone(), rule.engine);
    match stop {
        Stop::NotAvailable { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::NoCargoProject { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::NotOnPath { program } => RuleOutcome::NotAvailable { rule: id, engine, why: not_on_path_why(&program) },
        Stop::Failed { code, stderr } => RuleOutcome::Failed { rule: id, engine, code, stderr },
        Stop::NoFiles { .. } => RuleOutcome::Findings(Vec::new()),
        Stop::UnknownCheck { said } => {
            let why = too_old_why(engine.name(), version_of(engine, versions), &rule.check, &said);
            RuleOutcome::NotAvailable { rule: id, engine, why }
        }
        Stop::NotListed { said } => {
            let version = version_of(engine, versions).unwrap_or("(version unknown)");
            RuleOutcome::NotAvailable { rule: id, engine, why: format!("{} {version}: {said}", engine.name()) }
        }
    }
}

/// The version of `engine`: the first of `versions` for it, None when there is none or it is `None`.
fn version_of(engine: Engine, versions: &[EngineVersion]) -> Option<&str> {
    versions.iter().find(|v| v.engine == engine).and_then(|v| v.version.as_deref())
}

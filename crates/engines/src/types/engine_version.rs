//! `EngineVersion`: one engine of a check with the version read from it.

use rules::Engine;

/// One engine a check's rules name, with its version: meant as one item of an `engines: [{engine, version}]` field
/// the report does not have yet; nothing in report or cli reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineVersion {
    /// The engine.
    pub engine: Engine,
    /// The version its probe read; `None` when the engine is not available, has no probe, the rule file's folder
    /// could not be resolved, or the probe gave no version.
    pub version: Option<String>,
}

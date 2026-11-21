//! `EngineVersion`: one engine the rules name and the version it reported, if any.

use rules::Engine;
use serde::Serialize;

/// One engine in a report: its rule-file name and its version text, `null` when the version could not be read or the
/// engine is absent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EngineVersion {
    pub engine: Engine,
    pub version: Option<String>,
}

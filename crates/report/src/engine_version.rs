//! `EngineVersion`: one engine that ran and the version it reported.

use rules::Engine;
use serde::Serialize;

/// One engine in a report: its rule-file name and its version text, `null` when the version could not be read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EngineVersion {
    pub engine: Engine,
    pub version: Option<String>,
}

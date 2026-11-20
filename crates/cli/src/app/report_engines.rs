//! `report_engines`: the engines a check named, as the report carries them.

/// Each engine of the check as a report entry, field for field and in the same order; none dropped, none added.
pub(crate) fn report_engines(engines: Vec<engines::EngineVersion>) -> Vec<report::EngineVersion> {
    engines
        .into_iter()
        .map(|one| report::EngineVersion { engine: one.engine, version: one.version })
        .collect()
}

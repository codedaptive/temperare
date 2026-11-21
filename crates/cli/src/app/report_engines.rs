//! `report_engines`: the engines that ran, as the report carries them.

use engines::Availability;
use rules::Engine;

/// The report's engines (SPEC-baseline:158, "one per engine that ran, in first-use order"): of `engines`, in the order
/// given (the order the rules first name each engine), each that `availability` finds here and whose entry has a
/// version probe, mapped field for field; its version stays None when the probe gave none. An engine not found, with
/// no adapter or never available, and one with no version probe (Tessera), is left out, whatever its rules' outcomes.
pub(crate) fn report_engines(
    engines: Vec<engines::EngineVersion>,
    availability: &dyn Fn(Engine) -> Availability,
) -> Vec<report::EngineVersion> {
    engines
        .into_iter()
        .filter(|one| {
            engines::entry_for(one.engine).is_some_and(|entry| entry.version.is_some())
                && availability(one.engine) == Availability::Available
        })
        .map(|one| report::EngineVersion { engine: one.engine, version: one.version })
        .collect()
}

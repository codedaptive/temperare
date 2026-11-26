//! `report_engines`: the engines that ran, as the report carries them.

use engines::Availability;
use rules::Engine;

/// The report's engines (SPEC-baseline.md:228, "one per engine that ran, in first-use order"): of `engines`, in the
/// order given (the order the rules first name each engine), each that `availability` finds for the rule file's folder
/// (the folder the caller's closure asks about) and whose entry (`entry_of`) has a version probe, mapped field for
/// field; its version stays None when the probe gave none. An engine whose entry has no version probe (Tessera), and
/// one not found for that folder or never available, is left out, whatever its rules' outcomes.
pub(crate) fn report_engines(
    engines: Vec<engines::EngineVersion>,
    availability: &dyn Fn(Engine) -> Availability,
) -> Vec<report::EngineVersion> {
    engines
        .into_iter()
        .filter(|one| {
            engines::entry_of(one.engine).version.is_some()
                && availability(one.engine) == Availability::Available
        })
        .map(|one| report::EngineVersion { engine: one.engine, version: one.version })
        .collect()
}

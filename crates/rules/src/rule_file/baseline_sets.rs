//! `baseline_sets`: the baseline sets built into `tm`, each with its ready mark.

use super::baseline_set::BaselineSet;
use super::embedded_sets::EMBEDDED_SETS;

/// Every baseline set built into `tm`, in the order `tm` holds them, as ("name@version", ready): the list a caller tests
/// with `baseline_rules`. Ready false means a rule file's `baseline` refuses the set; `baseline_rules` still loads it.
pub fn baseline_sets() -> Vec<(String, bool)> {
    baseline_sets_in(EMBEDDED_SETS)
}

/// `baseline_sets` with `sets` in place of the sets built into `tm`.
pub(crate) fn baseline_sets_in(sets: &[BaselineSet]) -> Vec<(String, bool)> {
    sets.iter().map(|set| (format!("{}@{}", set.name, set.version), set.ready)).collect()
}

//! `EMBEDDED_SETS`: the baseline sets built into `tm`.

use super::baseline_set::BaselineSet;

/// Every released version of every baseline set, each a rule file under `crates/rules/baseline/<name>/<version>.toml`.
/// A released version is frozen and stays in this list, so a rule file taking it gets the same rules in every later
/// `tm`.
pub(crate) const EMBEDDED_SETS: &[BaselineSet] = &[
    BaselineSet { name: "house", version: 1, text: include_str!("../../baseline/house/1.toml") },
    BaselineSet { name: "standard", version: 1, text: include_str!("../../baseline/standard/1.toml") },
];

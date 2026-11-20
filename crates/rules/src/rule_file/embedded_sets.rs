//! `EMBEDDED_SETS`: the baseline sets built into `tm`.

use super::baseline_set::BaselineSet;

/// Every released version of every baseline set. Each lives in its own folder,
/// `crates/rules/baseline/<name>/<version>/`: the rule file `set.toml`, and beside it the ast-grep YAML files its rules
/// name in `check` (relative to that folder), each listed in the set's `yaml` as (path, include_str!(..)). A released
/// version is frozen and stays in this list, so a rule file taking it gets the same rules in every later `tm`.
pub(crate) const EMBEDDED_SETS: &[BaselineSet] = &[
    BaselineSet { name: "house", version: 1, text: include_str!("../../baseline/house/1/set.toml"), yaml: &[] },
    BaselineSet { name: "standard", version: 1, text: include_str!("../../baseline/standard/1/set.toml"), yaml: &[] },
];

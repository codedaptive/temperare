//! `EMBEDDED_SETS`: the baseline sets built into `tm`.

use super::baseline_set::BaselineSet;
use super::house_1::HOUSE_1;
use super::standard_1::STANDARD_1;

/// Every version of every baseline set built into `tm`, ready or not. Each lives in its own folder,
/// `crates/rules/baseline/<name>/<version>/`: the rule file `set.toml`, and beside it the ast-grep YAML files its rules
/// name in `check` (relative to that folder), each listed in the set's `yaml` as (path, include_str!(..)); and each
/// rule's case files at `cases/<rule id>/<failing|passing>/<file>`, `<file>` as the rule's case lists it in `files`,
/// each listed in the set's `cases` as (that path, include_str!(..)). A set that carries files is a const in a file of
/// its own, which lists them (house@1: `HOUSE_1`; standard@1: `STANDARD_1`). A version once ready is frozen and stays
/// in this list, so a rule file taking it gets the same rules in every later `tm`.
pub(crate) const EMBEDDED_SETS: &[BaselineSet] = &[HOUSE_1, STANDARD_1];

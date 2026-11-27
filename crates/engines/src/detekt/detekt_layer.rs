//! `detekt_layer`: cctao's own detekt config layer for a check.

use super::picked_rules;

/// The YAML of the config layer cctao passes as the last `--config` file, after the repository's own. Measured with
/// detekt 1.23.8 and this layer for MagicNumber, the repository config alone and then `--config repo.yml,layer.yml`
/// (`fixtures/commands.txt`, "Layer probes"):
/// - `config: validation: true` with an empty `excludes`: a repository with `validation: false`, or with `excludes:
///   'style>.*>bogusKey'`, and an unknown `style: MagicNumber: bogusKey` ran alone with exit 2, and with this layer
///   with exit 3, the property named.
/// - `output-reports: active: true`: a repository's `output-reports: active: false` alone left no SARIF file; with
///   this layer the file was written. A repository's `exclude: ['SarifOutputReport']` alone still wrote it, so that
///   key is left alone.
/// - for each picked rule set, `active: true` and empty `excludes` and `includes`; for each picked rule under it,
///   `active: true`, empty `excludes` and `includes`, and an empty `ignoreAnnotated`. A repository's `style: active:
///   false`, `MagicNumber: active: false`, `excludes`, `includes` or `ignoreAnnotated` on the rule, or `excludes` or
///   `includes` on the set, each hid some MagicNumber findings alone; with this layer every one was found. A
///   set-level `ignoreAnnotated` failed validation (exit 3, alone and with this layer), so the layer has none.
///
/// Rules sit under their sets in `picked_rules` order, which keeps each set's rules together.
pub(crate) fn detekt_layer(check: &str) -> String {
    let mut yaml = String::from("config:\n  validation: true\n  excludes: ''\noutput-reports:\n  active: true\n");
    let mut open_set = None;
    for (set, rule) in picked_rules(check) {
        if open_set != Some(set) {
            yaml.push_str(&format!("{set}:\n  active: true\n  excludes: []\n  includes: []\n"));
            open_set = Some(set);
        }
        yaml.push_str(&format!("  {rule}:\n    active: true\n    excludes: []\n    includes: []\n    ignoreAnnotated: []\n"));
    }
    yaml
}

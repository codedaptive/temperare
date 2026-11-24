//! `ignored_listed`: whether a hadolint config hides findings through `ignored`, which tm cannot turn off (S-67 (1)).

use crate::yaml::{yaml_entries, YamlValue};

/// Why the config text `text` stops the rule, `name` being how the message names the file; None when it hides
/// nothing. `--ignore` cannot take back what the config's `ignored` lists (measured), so a config that lists
/// anything there leaves the rule with no answer; exceptions go in the rule file.
///
/// hadolint 2.15.1 reads its config with HsYAML, measured: YAML 1.2 core schema, keys and codes case-sensitive,
/// quoted keys and items, aliases and the `!!seq` and `!!str` tags followed, a `<<` merge not applied. It rejects a
/// duplicate key, a second document and a scalar `ignored` with an error on stderr, then runs on its defaults, which
/// the reader of its output stops on.
///
/// `text` is read with `yaml_entries`; what it cannot read is "`name`: line N: tm cannot read it: ..." and stops the
/// rule. It refuses duplicate keys, a second document and `<<`, so those stop the rule here even where hadolint would
/// have rejected the file: either way the rule gives no answer. An entry hides findings when its first key is
/// `ignored` in any ASCII case and it is a sequence item or a non-null scalar. The case is matched wider than hadolint
/// reads it: a key hadolint would pass over costs only a stopped rule, while a key missed is findings hidden behind a
/// clean read. A null `ignored`, an empty list and every other key (`override` cannot hide a rule) hide nothing.
pub(crate) fn ignored_listed(name: &str, text: &str) -> Option<String> {
    let entries = match yaml_entries(text) {
        Ok(entries) => entries,
        // `why` starts `line N: `; the line stays first, as in every message here.
        Err(why) => {
            let why = why.replacen(": ", ": tm cannot read it: ", 1);
            return Some(format!("{name}: {why}; put exceptions in the rule file"));
        }
    };
    entries
        .iter()
        .find(|entry| {
            entry.path.first().is_some_and(|key| key.eq_ignore_ascii_case("ignored"))
                && (entry.item || matches!(entry.value, YamlValue::Plain(_) | YamlValue::Quoted(_)))
        })
        .map(|entry| {
            format!(
                "{name}: line {}: `ignored` hides hadolint findings outside the rule file; put exceptions in the rule \
                 file (S-67 (1))",
                entry.line
            )
        })
}

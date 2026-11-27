//! YAML read through a parser (`saphyr-parser`) into entries a config reader can match: each mapping value and
//! sequence item with its key path, line and value (`yaml_entries`, `YamlEntry`, `YamlValue`), and the walk over the
//! parser's events behind it (`Walk`, one file per method, `scalar_value` and `core_value`). `core_value`, a plain or
//! tagged scalar's core-schema value, is shared with the crate.

mod core_value;
mod scalar_value;
mod walk;
mod walk_members;
mod walk_next;
mod walk_node;
mod walk_replay;
mod yaml_entries;
mod yaml_entry;
mod yaml_value;

#[cfg(test)]
mod yaml_entries_tests;

pub(crate) use core_value::core_value;
pub(crate) use yaml_entries::yaml_entries;
pub(crate) use yaml_entry::YamlEntry;
pub(crate) use yaml_value::YamlValue;

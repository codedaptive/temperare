//! `YamlEntry`: one mapping value or sequence item of a YAML text, with where it stands.

use super::YamlValue;

/// One node `yaml_entries` gives. `path` is the mapping keys from the document root down to and including this node's
/// key; a sequence item adds no key, so an item's path is its sequence's. `item` says the node is a sequence item.
/// `line` (from 1) is where the node starts, except that a mapping or sequence under a key is at the key's line, so a
/// message names the key (`exclude:`) rather than the line after it; an alias's entries are at the alias's line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct YamlEntry {
    pub(crate) path: Vec<String>,
    pub(crate) item: bool,
    pub(crate) line: usize,
    pub(crate) value: YamlValue,
}

//! `Walk`: the parser behind `yaml_entries` and what it has read so far. A type, tested through yaml_entries, so no
//! test file of its own.

use super::{YamlEntry, YamlValue};
use saphyr_parser::{Parser, StrInput};
use std::collections::HashMap;

/// An anchored node as an alias replays it: its own value, then each entry under it with its path below the node.
pub(super) type Anchored = (YamlValue, Vec<(Vec<String>, bool, YamlValue)>);

/// The parser and what the walk has read so far: the entries, and each completed anchor by the parser's id.
pub(super) struct Walk<'a> {
    pub(super) parser: Parser<'a, StrInput<'a>>,
    pub(super) anchors: HashMap<usize, Anchored>,
    pub(super) entries: Vec<YamlEntry>,
}

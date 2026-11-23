//! `Walk::node`: one node and what is under it. Tested through yaml_entries (yaml_entries_tests.rs), so no test file
//! of its own.

use super::core_value::core_value;
use super::scalar_value::scalar_value;
use super::walk::Walk;
use super::yaml_entries::DEEPEST;
use super::{YamlEntry, YamlValue};
use saphyr_parser::{Event, Span};

impl<'a> Walk<'a> {
    /// Reads the node `event` starts, at `path`, an item or not, and what is under it. A mapping, a sequence or an
    /// empty value (an empty span: it has no place of its own) is at `key_line`, the line of its key, if any. `depth`
    /// is how many mappings and sequences hold it; one more level past `DEEPEST` is an `Err` before the walk recurses.
    /// A scalar's core tag is checked by `core_value`: a scalar whose core tag its text does not fit, or with a `!!`
    /// tag tm does not read, is an `Err`. A `!!` tag on a mapping other than `!!map`, or on a sequence other than
    /// `!!seq`, is an `Err` naming it (`yaml_entries` says why). Any other tag on a collection is read past, as dart
    /// reads it (measured, dart/fixtures/commands.txt, "YAML forms" 12).
    pub(super) fn node(
        &mut self,
        (event, span): (Event<'a>, Span),
        path: &[String],
        item: bool,
        key_line: Option<usize>,
        depth: usize,
    ) -> Result<(), String> {
        let at = span.start.line();
        let mapping = matches!(event, Event::MappingStart(..));
        let (anchor, value) = match event {
            Event::Scalar(text, style, anchor, tag) => {
                core_value(&text, style, tag.as_deref()).map_err(|why| format!("line {at}: {why}"))?;
                (anchor, scalar_value(&text, style, tag.is_some()))
            }
            Event::Alias(id) => return self.replay(id, path, item, at),
            Event::MappingStart(anchor, tag) | Event::SequenceStart(anchor, tag) => {
                let (kind, own) = if mapping { ("mapping", "map") } else { ("sequence", "seq") };
                if let Some(tag) = tag.filter(|tag| tag.is_yaml_core_schema() && tag.suffix != own) {
                    return Err(format!("line {at}: a `!!{}` tag on a {kind}, which tm does not read", tag.suffix));
                }
                (anchor, YamlValue::Nested)
            }
            other => return Err(format!("line {at}: {other:?} where a node starts")),
        };
        let line = match (&value, key_line) {
            (YamlValue::Nested, Some(key)) => key,
            (YamlValue::Null, Some(key)) if span.start == span.end => key,
            _ => at,
        };
        let start = self.entries.len();
        self.entries.push(YamlEntry { path: path.to_vec(), item, line, value: value.clone() });
        if value == YamlValue::Nested {
            if depth == DEEPEST {
                return Err(format!("line {line}: nested deeper than {DEEPEST} levels"));
            }
            self.members(path, mapping, depth + 1)?;
        }
        if anchor > 0 {
            let under = self.entries[start + 1..].iter();
            let under = under.map(|entry| (entry.path[path.len()..].to_vec(), entry.item, entry.value.clone()));
            self.anchors.insert(anchor, (value, under.collect()));
        }
        Ok(())
    }
}

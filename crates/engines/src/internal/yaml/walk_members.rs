//! `Walk::members`: the members of one mapping or sequence. Tested through yaml_entries (yaml_entries_tests.rs), so no
//! test file of its own.

use super::core_value::core_value;
use super::scalar_value::scalar_value;
use super::walk::Walk;
use super::yaml_entries::MOST_ENTRIES;
use saphyr_parser::{Event, ScalarStyle};
use std::collections::HashSet;

impl Walk<'_> {
    /// Reads the members of the mapping (`mapping`) or sequence just started at `path`, `depth` levels down, up to its
    /// end. A key is its decoded scalar text; an anchored key is kept as its scalar value, so an alias to it replays
    /// that. A merge key `<<`, a key that repeats, a key that is not a scalar, a key whose core tag its text does not
    /// fit or with a `!!` tag cctao does not read, and entries past `MOST_ENTRIES` are each an `Err` naming the line
    /// (`yaml_entries` says why). A repeat is found by the key's core-schema value (`core_value`: `1` and `0x1` are
    /// one key, `"1"` and `1` two), as dart's reader finds one when it rejects the
    /// file and drops its options (measured, dart/fixtures/commands.txt); the message names the repeat's decoded text.
    pub(super) fn members(&mut self, path: &[String], mapping: bool, depth: usize) -> Result<(), String> {
        let mut keys = HashSet::new();
        loop {
            let (event, span) = self.next()?;
            let line = span.start.line();
            if self.entries.len() > MOST_ENTRIES {
                return Err(format!("line {line}: more than {MOST_ENTRIES} YAML entries (nested aliases?)"));
            }
            if matches!(event, Event::MappingEnd | Event::SequenceEnd) {
                return Ok(());
            }
            if !mapping {
                self.node((event, span), path, true, None, depth)?;
                continue;
            }
            let (key, read_as) = match event {
                Event::Scalar(key, ScalarStyle::Plain, ..) if key == "<<" => {
                    return Err(format!("line {line}: a merge key `<<`"));
                }
                Event::Scalar(key, style, anchor, tag) => {
                    let read_as = core_value(&key, style, tag.as_deref()).map_err(|why| format!("line {line}: {why}"))?;
                    if anchor > 0 {
                        self.anchors.insert(anchor, (scalar_value(&key, style, tag.is_some()), Vec::new()));
                    }
                    (key.into_owned(), read_as)
                }
                _ => return Err(format!("line {line}: a key that is not a scalar")),
            };
            if !keys.insert(read_as) {
                return Err(format!("line {line}: duplicate mapping key `{key}`"));
            }
            let value = self.next()?;
            self.node(value, &[path, std::slice::from_ref(&key)].concat(), false, Some(line), depth)?;
        }
    }
}


//! `yaml_entries`: a YAML text as the entries of its mappings and sequences, each with its key path and line.

use super::walk::Walk;
use super::YamlEntry;
use saphyr_parser::{Event, Parser};
use std::collections::HashMap;

/// The most entries `yaml_entries` gives. Real options files hold far fewer; nested aliases can multiply entries
/// without bound and hang cctao, so past this the text fails closed.
pub(super) const MOST_ENTRIES: usize = 100_000;

/// The most levels of mappings and sequences one inside another. Real config nests a handful; the walk recurses once
/// per level (an alias is replayed flat, without recursing), so a hostile file could otherwise overflow the stack.
pub(super) const DEEPEST: usize = 64;

/// The entries of `text`: one per mapping value and per sequence item, in document order; the root node gives none.
/// Keys are their scalar text, decoded, in any style. An alias stands for its anchored node, a key included, so the
/// node's entries (its own and those under it) are given again under the alias's path, item flag and line: nothing
/// hides behind an alias. Empty text, or only comments, is `Ok` with no entries; a leading byte-order mark is passed
/// over. Pure.
///
/// The `Err`s start `line N: ` and name what is wrong. A caller reads these entries to find what a config hides, so
/// what it cannot read is never passed over as clean. Dart 3.14.1's readings below are measured, except
/// where a bullet says unmeasured (dart/fixtures/commands.txt, "YAML forms"):
/// - text the parser rejects, with the parser's own message at its marker;
/// - a second document: dart rejects the file (`Only expected one document.`) and drops its options;
/// - a key that repeats in one mapping by its core-schema value (`core_value`: `1`, `0x1` and `1.0` are one key, `"1"`
///   and `1` two), at the repeat: dart finds repeats the same way, rejects the file (`Duplicate mapping key.`) and
///   drops every option in it, so a lint listed in either block would read as enabled while dart runs none;
/// - a key or value tagged `!!null`, `!!bool`, `!!int` or `!!float` whose text does not read as that type
///   (`core_value`): dart rejects the file (`Invalid int scalar.`) and drops its options, so reading on would enable a
///   lint dart never runs (measured for `!!int` as a key and `!!null`, `!!bool` and `!!int` as values; not for
///   `!!float`);
/// - any other `!!` tag on a scalar (`!!seq`, `!!binary`, `!!timestamp`, ...), and a `!!` tag on a mapping other than
///   `!!map` or on a sequence other than `!!seq`: dart rejects a file whose node does not fit its core tag (measured
///   for `!!int`, `!!bool` and `!!null` scalars) and drops its options; these tags are unmeasured, so cctao refuses them
///   rather than guess, and a refusal is never a clean read;
/// - a key that is not a scalar (a collection, after `?` or in flow, or an alias): an entry's path is text and such
///   a key has none; dart reads the section as an invalid format yet still applies an `ignore` beside it;
/// - a plain merge key `<<`: dart's reading differs by section (under `analyzer` it merges nothing; under `linter:
///   rules` the merged lint runs), so cctao cannot say what it does;
/// - an alias inside the node its anchor names, whose entries are not complete when the alias is read;
/// - more than `MOST_ENTRIES` entries or `DEEPEST` levels, at the line where the walk finds the bound passed.
pub(crate) fn yaml_entries(text: &str) -> Result<Vec<YamlEntry>, String> {
    let mut walk = Walk { parser: Parser::new_from_str(text), anchors: HashMap::new(), entries: Vec::new() };
    walk.next()?;
    if walk.next()?.0 == Event::StreamEnd {
        return Ok(Vec::new());
    }
    let root = walk.next()?;
    walk.node(root, &[], false, None, 0)?;
    walk.next()?;
    match walk.next()? {
        // The root node's own entry is first; the root is the document, not a value in it.
        (Event::StreamEnd, _) => Ok(walk.entries.split_off(1)),
        (_, span) => Err(format!("line {}: a second YAML document", span.start.line())),
    }
}

//! `ReadFn`: the shape of an engine's reader.

use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// A reader: `(out, check, cwd, folder)` to the hits. `out` is the check run's output, `check` the rule's check,
/// `cwd` the folder the engine ran in (its paths are read relative to it) and `folder` the rule file's folder (each
/// hit's path is made relative to it; a path outside it is skipped). Output that is not the engine's answer is a
/// `ParseError`. An entry's `read` has this shape; an entry whose reader needs no check (ast-grep) wraps its
/// parser in a closure that drops `check`.
pub type ReadFn = fn(&Output, &str, &Path, &Path) -> Result<Vec<Hit>, ParseError>;

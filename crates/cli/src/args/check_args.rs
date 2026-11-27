//! `CheckArgs`: the flags and paths of `cctao check`.

use std::path::PathBuf;

/// `cctao check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]`; `paths` keeps the
/// order they were given in.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CheckArgs {
    pub rules: Option<PathBuf>,
    pub base: Option<String>,
    pub kind: Option<rules::Kind>,
    pub json: bool,
    pub paths: Vec<PathBuf>,
}

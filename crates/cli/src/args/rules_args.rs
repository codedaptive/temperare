//! `RulesArgs`: the flags of `tm rules`.

use std::path::PathBuf;

/// `tm rules [--rules <path>] [--json]`: the rule file given with `--rules`, and whether to print JSON.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RulesArgs {
    pub rules: Option<PathBuf>,
    pub json: bool,
}

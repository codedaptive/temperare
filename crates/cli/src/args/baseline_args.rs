//! `BaselineArgs`: the flags of `cctao baseline test`.

/// `cctao baseline test [--set <name@version>] [--json]`: the one set given with `--set` (its text as typed; the run
/// checks it), and whether to print JSON.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BaselineArgs {
    pub set: Option<String>,
    pub json: bool,
}

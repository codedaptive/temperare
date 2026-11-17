//! `Command`: what a tm command line asks for.

use super::{CheckArgs, RulesArgs};

/// The command parsed from tm's arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check(CheckArgs),
    Rules(RulesArgs),
    Help,
    Version,
}

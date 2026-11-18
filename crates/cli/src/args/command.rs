//! `Command`: what a tm command line asks for.

use super::{CheckArgs, InstallArgs, RulesArgs};

/// The command parsed from tm's arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check(CheckArgs),
    Rules(RulesArgs),
    Install(InstallArgs),
    Uninstall,
    Help,
    Version,
}

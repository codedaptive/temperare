//! `Command`: what a tm command line asks for.

use super::{BaselineArgs, CheckArgs, InstallArgs, RulesArgs};

/// The command parsed from tm's arguments.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Check(CheckArgs),
    Rules(RulesArgs),
    Baseline(BaselineArgs),
    Install(InstallArgs),
    Uninstall,
    Help,
    Version,
}

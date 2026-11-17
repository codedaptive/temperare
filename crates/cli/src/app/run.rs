//! `run`: tm's whole run, from the command line to the exit code.

use super::run_check::run_check;
use super::run_rules::run_rules;
use super::write_usage_error::write_usage_error;
use super::{CheckWorld, RulesWorld};
use crate::{parse_args, Command, USAGE};
use std::io::Write;
use std::path::Path;

/// Runs the command `args` names (the program name left out) in `cwd`, printing on `out` and `err`, and returns the
/// exit code: a usage error is 2, help and version are 0, `tm rules` is `run_rules` and `tm check` is `run_check`,
/// both against the real git, disk and PATH. Write errors are ignored: a closed pipe is not tm's failure.
pub fn run(args: &[String], cwd: &Path, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    let command = match parse_args(args) {
        Ok(command) => command,
        Err(e) => {
            write_usage_error(err, &e);
            return 2;
        }
    };
    match command {
        Command::Help => {
            let _ = out.write_all(USAGE.as_bytes());
            0
        }
        Command::Version => {
            let _ = writeln!(out, "tm {}", env!("CARGO_PKG_VERSION"));
            0
        }
        Command::Rules(rules_args) => run_rules(&rules_args, cwd, &RulesWorld::real(), out, err),
        Command::Check(check_args) => run_check(&check_args, cwd, &CheckWorld::real(), out, err),
    }
}

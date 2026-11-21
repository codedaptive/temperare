//! `run`: tm's whole run, from the command line to the exit code.

use super::run_baseline_test::run_baseline_test;
use super::run_check::run_check;
use super::run_install::run_install;
use super::run_rules::run_rules;
use super::run_uninstall::run_uninstall;
use super::write_usage_error::write_usage_error;
use super::{BaselineWorld, CheckWorld, RulesWorld};
use crate::install::pick_harnesses;
use crate::{parse_args, version_text, Command, InstallHost, USAGE};
use std::io::Write;
use std::path::Path;

/// Runs the command `args` names (the program name left out) in `cwd`, printing on `out` and `err`, and returns the
/// exit code: a usage error is 2, help and version are 0, `tm rules` (also `tm rules --loosened`) is `run_rules` and
/// `tm check` is `run_check`, both against the real git, disk, PATH and clock (`tm check` also reads HOME).
/// `tm baseline test` is `run_baseline_test` on the sets built into tm, its cases in folders under the system temp
/// folder run by the real engines (`BaselineWorld::real`). `install` and `uninstall`
/// are `run_install` and `run_uninstall` on the real host (`InstallHost::real`), the harness picker reading stdin
/// and asking on stderr; a host that cannot be read (HOME not set or not absolute, the running binary not found) is
/// 2. Write errors are ignored: a closed pipe is not tm's failure.
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
            let _ = writeln!(out, "{}", version_text(env!("CARGO_PKG_VERSION"), env!("TEMPERARE_COMMIT")));
            0
        }
        Command::Rules(rules_args) => run_rules(&rules_args, cwd, &RulesWorld::real(), out, err),
        Command::Check(check_args) => run_check(&check_args, cwd, &CheckWorld::real(), out, err),
        Command::Baseline(baseline_args) => run_baseline_test(&baseline_args, &BaselineWorld::real(), out, err),
        Command::Install(install_args) => match real_host(cwd, err) {
            Ok(host) => run_install(
                &install_args,
                &host,
                &mut |detected| pick_harnesses(detected, &mut std::io::stdin().lock(), &mut std::io::stderr()),
                out,
                err,
            ),
            Err(code) => code,
        },
        Command::Uninstall => match real_host(cwd, err) {
            Ok(host) => run_uninstall(&host, out, err),
            Err(code) => code,
        },
    }
}

/// The real install host, or exit 2 with its error on `err` as `write_usage_error` prints it: `tm: <what>` and the
/// `next:` line for that cause.
fn real_host(cwd: &Path, err: &mut dyn Write) -> Result<InstallHost, u8> {
    InstallHost::real(cwd).map_err(|e| {
        write_usage_error(err, &e);
        2
    })
}

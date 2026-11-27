//! `parse_install_args`: the arguments after `cctao install`.

use super::flag_value::flag_value;
use super::{InstallArgs, UsageError};
use crate::Harness;

const FIX: &str = "Run cctao help to see the flags of cctao install.";

/// `--no-place`, `-y`, `--target <list>` (commas between claude, codex and cursor; each kept once,
/// in order), `--mode skill` and `--location local`. Any other value of those, any other flag and a positional
/// argument are usage errors. A flag given twice keeps its last value here: `parse_args` reports the repeat.
pub(super) fn parse_install_args(args: &[String]) -> Result<InstallArgs, UsageError> {
    let mut parsed = InstallArgs::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--no-place" => parsed.no_place = true,
            "-y" => parsed.yes = true,
            "--target" => parsed.targets = Some(targets(&flag_value("--target", "claude,codex,cursor", &mut rest)?)?),
            "--mode" => only(
                "--mode",
                &flag_value("--mode", "skill", &mut rest)?,
                "skill",
                "Write --mode skill: cctao wires a skill only, no MCP.",
            )?,
            "--location" => {
                let fix = "Write --location local, or leave it out to wire the harness folders in HOME.";
                only("--location", &flag_value("--location", "local", &mut rest)?, "local", fix)?;
                parsed.local = true;
            }
            flag if flag.starts_with('-') => {
                return Err(UsageError { what: format!("unknown flag '{flag}' for cctao install"), fix: FIX.to_string() })
            }
            other => {
                return Err(UsageError {
                    what: format!("unexpected argument '{other}' for cctao install"),
                    fix: FIX.to_string(),
                })
            }
        }
    }
    Ok(parsed)
}

/// The harnesses `list` names, each once in order; an unknown or empty name is a usage error naming it.
fn targets(list: &str) -> Result<Vec<Harness>, UsageError> {
    let mut chosen = Vec::new();
    for name in list.split(',') {
        let harness = Harness::from_name(name).ok_or_else(|| UsageError {
            what: format!("unknown harness '{name}' in --target"),
            fix: "Write --target with claude, codex or cursor, commas between them.".to_string(),
        })?;
        if !chosen.contains(&harness) {
            chosen.push(harness);
        }
    }
    Ok(chosen)
}

/// Nothing when `value` is the one value `flag` takes, else a usage error with `fix`.
fn only(flag: &str, value: &str, allowed: &str, fix: &str) -> Result<(), UsageError> {
    if value == allowed {
        return Ok(());
    }
    Err(UsageError { what: format!("{flag} {value} is not supported"), fix: fix.to_string() })
}

//! `parse_args`: tm's arguments to a `Command`, by hand on std, with no argument-parsing crate.

use super::parse_baseline_args::parse_baseline_args;
use super::parse_check_args::parse_check_args;
use super::parse_install_args::parse_install_args;
use super::parse_rules_args::parse_rules_args;
use super::{Command, UsageError};

/// The command `args` asks for; `args` excludes the program name. Pure: no I/O, no environment.
/// No arguments, `help`, `--help` or `-h` is Help; `--version` is Version; `uninstall` is Uninstall; none of them
/// takes anything after it. `baseline` takes its subcommand `test` and that subcommand's flags.
/// Flags come after the command. A flag the command takes but given twice is reported once the rest parses.
pub fn parse_args(args: &[String]) -> Result<Command, UsageError> {
    let Some(first) = args.first() else {
        return Ok(Command::Help);
    };
    let rest = &args[1..];
    let command = match first.as_str() {
        "help" | "--help" | "-h" => alone(first, Command::Help, rest)?,
        "--version" => alone(first, Command::Version, rest)?,
        "rules" => Command::Rules(parse_rules_args(rest)?),
        "check" => Command::Check(parse_check_args(rest)?),
        "baseline" => Command::Baseline(parse_baseline_args(rest)?),
        "install" => Command::Install(parse_install_args(rest)?),
        "uninstall" => alone(first, Command::Uninstall, rest)?,
        other => {
            return Err(UsageError {
                what: format!("unknown command '{other}'"),
                fix: "Run tm help to see the commands.".to_string(),
            })
        }
    };
    match repeated_flag(rest) {
        Some(flag) => Err(UsageError {
            what: format!("{flag} given twice"),
            fix: format!("Give {flag} once."),
        }),
        None => Ok(command),
    }
}

/// `command` when nothing follows `word`; else a usage error naming the first extra argument.
fn alone(word: &str, command: Command, rest: &[String]) -> Result<Command, UsageError> {
    match rest.first() {
        None => Ok(command),
        Some(extra) => Err(UsageError {
            what: format!("unexpected argument '{extra}' after {word}"),
            fix: format!("Run tm {word} with nothing after it."),
        }),
    }
}

/// The first flag that appears again later. Values never start with `-` (`flag_value`), so every argument that
/// does is a flag.
fn repeated_flag(rest: &[String]) -> Option<&String> {
    rest.iter()
        .enumerate()
        .filter(|(_, arg)| arg.starts_with('-'))
        .find(|(i, arg)| rest[i + 1..].contains(arg))
        .map(|(_, arg)| arg)
}

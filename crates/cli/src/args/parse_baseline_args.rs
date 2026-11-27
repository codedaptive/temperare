//! `parse_baseline_args`: the arguments after `cctao baseline`.

use super::flag_value::flag_value;
use super::{BaselineArgs, UsageError};

/// The subcommand `test`, then `--set <name@version>` and `--json`. No subcommand, another subcommand, any other
/// flag or a positional argument is a usage error. The set's text is kept as given: the run checks it. A flag
/// given twice keeps its last value here: `parse_args` reports the repeat.
pub(super) fn parse_baseline_args(args: &[String]) -> Result<BaselineArgs, UsageError> {
    let mut rest = args.iter();
    match rest.next().map(String::as_str) {
        Some("test") => {}
        None => return Err(subcommand_error("cctao baseline needs a subcommand".to_string())),
        Some(other) => return Err(subcommand_error(format!("unknown baseline subcommand '{other}'"))),
    }
    let mut parsed = BaselineArgs::default();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--set" => parsed.set = Some(flag_value("--set", "<name@version>", &mut rest)?),
            "--json" => parsed.json = true,
            flag if flag.starts_with('-') => {
                return Err(test_error(format!("unknown flag '{flag}' for cctao baseline test")))
            }
            other => return Err(test_error(format!("unexpected argument '{other}' for cctao baseline test"))),
        }
    }
    Ok(parsed)
}

fn subcommand_error(what: String) -> UsageError {
    UsageError { what, fix: "Run cctao baseline test.".to_string() }
}

fn test_error(what: String) -> UsageError {
    UsageError { what, fix: "Run cctao help to see the flags of cctao baseline test.".to_string() }
}

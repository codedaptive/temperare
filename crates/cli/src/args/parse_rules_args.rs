//! `parse_rules_args`: the arguments after `cctao rules`.

use std::path::PathBuf;

use super::flag_value::flag_value;
use super::{RulesArgs, UsageError};

/// `--rules <path>`, `--json`, `--loosened` and `--base <rev>`; any other flag or a positional argument is a usage
/// error. `--loosened` and `--base` come together or not at all, in either order: that is checked once every flag
/// is read. A flag given twice keeps its last value here: `parse_args` reports the repeat.
pub(super) fn parse_rules_args(args: &[String]) -> Result<RulesArgs, UsageError> {
    let mut parsed = RulesArgs::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--rules" => parsed.rules = Some(PathBuf::from(flag_value("--rules", "<path>", &mut rest)?)),
            "--json" => parsed.json = true,
            "--loosened" => parsed.loosened = true,
            "--base" => parsed.base = Some(flag_value("--base", "<rev>", &mut rest)?),
            flag if flag.starts_with('-') => {
                return Err(UsageError {
                    what: format!("unknown flag '{flag}' for cctao rules"),
                    fix: "Run cctao help to see the flags of cctao rules.".to_string(),
                })
            }
            other => {
                return Err(UsageError {
                    what: format!("unexpected argument '{other}' for cctao rules"),
                    fix: "Run cctao help to see the flags of cctao rules.".to_string(),
                })
            }
        }
    }
    match (parsed.loosened, parsed.base.is_some()) {
        (true, false) => Err(UsageError {
            what: "cctao rules --loosened needs --base <rev>".to_string(),
            fix: "Pass --base <rev>, the commit to compare with.".to_string(),
        }),
        (false, true) => Err(UsageError {
            what: "--base is only for cctao rules --loosened".to_string(),
            fix: "Add --loosened, or leave --base out.".to_string(),
        }),
        _ => Ok(parsed),
    }
}

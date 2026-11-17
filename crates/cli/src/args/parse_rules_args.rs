//! `parse_rules_args`: the arguments after `tm rules`.

use std::path::PathBuf;

use super::flag_value::flag_value;
use super::{RulesArgs, UsageError};

/// `--rules <path>` and `--json`; any other flag or a positional argument is a usage error. A flag given twice
/// keeps its last value here: `parse_args` reports the repeat.
pub(super) fn parse_rules_args(args: &[String]) -> Result<RulesArgs, UsageError> {
    let mut parsed = RulesArgs::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--rules" => parsed.rules = Some(PathBuf::from(flag_value("--rules", "<path>", &mut rest)?)),
            "--json" => parsed.json = true,
            flag if flag.starts_with('-') => {
                return Err(UsageError {
                    what: format!("unknown flag '{flag}' for tm rules"),
                    fix: "Run tm help to see the flags of tm rules.".to_string(),
                })
            }
            other => {
                return Err(UsageError {
                    what: format!("unexpected argument '{other}' for tm rules"),
                    fix: "Run tm help to see the flags of tm rules.".to_string(),
                })
            }
        }
    }
    Ok(parsed)
}

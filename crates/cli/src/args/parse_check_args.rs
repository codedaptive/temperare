//! `parse_check_args`: the arguments after `cctao check`.

use std::path::PathBuf;

use super::flag_value::flag_value;
use super::{CheckArgs, UsageError};

/// `--rules <path>`, `--base <rev>`, `--kind quality|security` and `--json`; every argument not starting with `-`
/// is a path, kept in order. Any other flag is a usage error. A flag given twice keeps its last value here:
/// `parse_args` reports the repeat.
pub(super) fn parse_check_args(args: &[String]) -> Result<CheckArgs, UsageError> {
    let mut parsed = CheckArgs::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--rules" => parsed.rules = Some(PathBuf::from(flag_value("--rules", "<path>", &mut rest)?)),
            "--base" => parsed.base = Some(flag_value("--base", "<rev>", &mut rest)?),
            "--kind" => parsed.kind = Some(kind(&flag_value("--kind", "quality|security", &mut rest)?)?),
            "--json" => parsed.json = true,
            flag if flag.starts_with('-') => {
                return Err(UsageError {
                    what: format!("unknown flag '{flag}' for cctao check"),
                    fix: "Run cctao help to see the flags of cctao check.".to_string(),
                })
            }
            path => parsed.paths.push(PathBuf::from(path)),
        }
    }
    Ok(parsed)
}

/// The kind named by `--kind`, through the rule file's own names.
fn kind(name: &str) -> Result<rules::Kind, UsageError> {
    rules::Kind::from_name(name).ok_or_else(|| UsageError {
        what: format!("unknown kind '{name}'"),
        fix: "Use --kind quality or --kind security.".to_string(),
    })
}

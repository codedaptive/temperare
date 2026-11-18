//! ZUT for parse_install_args.rs: each flag of tm install, the one value --mode and --location take, the --target
//! list, and the usage errors.

use super::parse_install_args::parse_install_args;
use super::{InstallArgs, UsageError};
use crate::Harness;

fn parse(args: &[&str]) -> Result<InstallArgs, UsageError> {
    parse_install_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
}

fn error(what: &str, fix: &str) -> Result<InstallArgs, UsageError> {
    Err(UsageError { what: what.to_string(), fix: fix.to_string() })
}

const INSTALL_FIX: &str = "Run tm help to see the flags of tm install.";
const TARGET_FIX: &str = "Write --target with claude, codex or cursor, commas between them.";

#[test]
fn nothing_is_the_default() {
    assert_eq!(parse(&[]), Ok(InstallArgs::default()));
}

#[test]
fn every_flag_together() {
    let args = ["--no-place", "--short-name", "-y", "--target", "cursor,claude"];
    assert_eq!(
        parse(&[&args[..], &["--mode", "skill", "--location", "local"]].concat()),
        Ok(InstallArgs {
            no_place: true,
            short_name: true,
            targets: Some(vec![Harness::Cursor, Harness::Claude]),
            local: true,
            yes: true,
        })
    );
}

#[test]
fn mode_skill_alone_changes_nothing() {
    assert_eq!(parse(&["--mode", "skill"]), Ok(InstallArgs::default()));
}

#[test]
fn a_target_list_is_kept_in_order_once_each() {
    let want = vec![Harness::Codex, Harness::Claude, Harness::Cursor];
    assert_eq!(parse(&["--target", "codex,claude,codex,cursor,claude"]).map(|a| a.targets), Ok(Some(want)));
}

#[test]
fn an_unknown_or_empty_target_name_is_refused_by_name() {
    assert_eq!(parse(&["--target", "claude,vscode"]), error("unknown harness 'vscode' in --target", TARGET_FIX));
    assert_eq!(parse(&["--target", "claude,,codex"]), error("unknown harness '' in --target", TARGET_FIX));
    assert_eq!(parse(&["--target", ""]), error("unknown harness '' in --target", TARGET_FIX));
    assert_eq!(parse(&["--target", "Claude"]), error("unknown harness 'Claude' in --target", TARGET_FIX));
}

#[test]
fn a_flag_without_its_value() {
    assert_eq!(parse(&["--target"]), error("--target needs a value", "Write --target claude,codex,cursor."));
    assert_eq!(parse(&["--mode", "-y"]), error("--mode needs a value", "Write --mode skill."));
    assert_eq!(parse(&["--location"]), error("--location needs a value", "Write --location local."));
}

#[test]
fn a_mode_other_than_skill_is_refused_with_no_mcp() {
    assert_eq!(
        parse(&["--mode", "mcp"]),
        error("--mode mcp is not supported", "Write --mode skill: Temperare wires a skill only, no MCP.")
    );
}

#[test]
fn a_location_other_than_local_is_refused() {
    assert_eq!(
        parse(&["--location", "global"]),
        error(
            "--location global is not supported",
            "Write --location local, or leave it out to wire the harness folders in HOME."
        )
    );
}

#[test]
fn an_unknown_flag_or_positional_is_refused() {
    assert_eq!(parse(&["--yes"]), error("unknown flag '--yes' for tm install", INSTALL_FIX));
    assert_eq!(parse(&["--json"]), error("unknown flag '--json' for tm install", INSTALL_FIX));
    assert_eq!(parse(&["claude"]), error("unexpected argument 'claude' for tm install", INSTALL_FIX));
}

//! tm's library: argument parsing, the rule-file listing, tm check, install and the run; the binary in main.rs is a thin shell.

mod app;
mod args;
mod install;
mod output;

pub use app::{discover_rule_file, run, run_check, run_install, run_rules, run_uninstall, CheckWorld, RulesWorld};
pub use args::{parse_args, CheckArgs, Command, InstallArgs, RulesArgs, UsageError, USAGE};
pub use install::{Harness, InstallHost, InstallPaths, InstallPlan, RecordEntry, PACKAGE_PREFIXES, SKILL_TEXT};
pub use output::{
    check_json, exit_code, failed_tails, next_step, problem_line, problems_json, rules_json, rules_text, version_text,
};

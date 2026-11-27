//! cctao's library: argument parsing, the rule-file listing and what it loosened since a base, cctao check and its
//! savings estimate, the files under HOME (the savings config switch and log, and the telemetry file), cctao baseline
//! test, install and the run; the binary in main.rs is a thin shell.

mod app;
mod args;
mod home_files;
mod install;
mod output;

pub use app::{
    discover_rule_file, run, run_baseline_test, run_check, run_install, run_rules, run_uninstall, BaselineWorld,
    CheckWorld, RulesWorld,
};
pub use args::{parse_args, BaselineArgs, CheckArgs, Command, InstallArgs, RulesArgs, UsageError, USAGE};
pub use home_files::{append_savings_log, savings_line_on, savings_log_line};
pub use install::{Harness, InstallHost, InstallPaths, InstallPlan, RecordEntry, PACKAGE_PREFIXES, SKILL_TEXT};
pub use output::{
    check_json, exit_code, failed_tails, loosened_json, loosened_text, next_step, problem_line, problems_json,
    rules_json, rules_text, saved_json, savings_from, tokens_saved_line, version_text, RuleCount, Savings,
};

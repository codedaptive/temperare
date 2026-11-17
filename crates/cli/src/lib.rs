//! tm's library: argument parsing, the rule-file listing, tm check and the run; the binary in main.rs is a thin shell.

mod app;
mod args;
mod output;

pub use app::{discover_rule_file, run, run_check, run_rules, CheckWorld, RulesWorld};
pub use args::{parse_args, CheckArgs, Command, RulesArgs, UsageError, USAGE};
pub use output::{
    check_json, exit_code, next_step, not_available_names, problem_line, problems_json, rules_json, rules_text,
};

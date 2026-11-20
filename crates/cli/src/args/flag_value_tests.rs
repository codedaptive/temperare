//! Unit tests for flag_value.rs: the next argument is the value unless it is missing or starts with '-'.

use super::flag_value::flag_value;
use super::UsageError;

fn strings(args: &[&str]) -> Vec<String> {
    args.iter().map(|a| a.to_string()).collect()
}

fn missing(flag: &str, fix: &str) -> UsageError {
    UsageError { what: format!("{flag} needs a value"), fix: fix.to_string() }
}

#[test]
fn takes_the_next_argument_and_leaves_the_rest() {
    let args = strings(&["r.toml", "--json"]);
    let mut rest = args.iter();
    assert_eq!(flag_value("--rules", "<path>", &mut rest), Ok("r.toml".to_string()));
    assert_eq!(rest.next(), Some(&"--json".to_string()));
}

#[test]
fn flag_last_is_a_missing_value() {
    let args = strings(&[]);
    assert_eq!(
        flag_value("--rules", "<path>", &mut args.iter()),
        Err(missing("--rules", "Write --rules <path>."))
    );
}

#[test]
fn next_argument_starting_with_dash_is_a_missing_value() {
    let args = strings(&["--json"]);
    assert_eq!(
        flag_value("--base", "<rev>", &mut args.iter()),
        Err(missing("--base", "Write --base <rev>."))
    );
}

#[test]
fn a_lone_dash_is_not_a_value() {
    let args = strings(&["-"]);
    assert_eq!(
        flag_value("--kind", "quality|security", &mut args.iter()),
        Err(missing("--kind", "Write --kind quality|security."))
    );
}

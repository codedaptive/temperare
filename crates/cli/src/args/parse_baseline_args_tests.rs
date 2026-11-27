//! ZUT for parse_baseline_args.rs: the `test` subcommand, `--set <name@version>` and `--json`; anything else is a
//! usage error.

use super::parse_baseline_args::parse_baseline_args;
use super::{BaselineArgs, UsageError};

fn usage(what: &str, fix: &str) -> UsageError {
    UsageError { what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn parse_baseline_args_rows() {
    const FLAGS: &str = "Run cctao help to see the flags of cctao baseline test.";
    const SUB: &str = "Run cctao baseline test.";
    let rows: Vec<(&[&str], Result<BaselineArgs, UsageError>)> = vec![
        (&["test"], Ok(BaselineArgs { set: None, json: false })),
        (&["test", "--json"], Ok(BaselineArgs { set: None, json: true })),
        (&["test", "--set", "house@1"], Ok(BaselineArgs { set: Some("house@1".to_string()), json: false })),
        (&["test", "--json", "--set", "nosuch"], Ok(BaselineArgs { set: Some("nosuch".to_string()), json: true })),
        (
            &["test", "--set", "house@1", "--set", "standard@1"],
            Ok(BaselineArgs { set: Some("standard@1".to_string()), json: false }),
        ),
        (&[], Err(usage("cctao baseline needs a subcommand", SUB))),
        (&["run"], Err(usage("unknown baseline subcommand 'run'", SUB))),
        (&["--json"], Err(usage("unknown baseline subcommand '--json'", SUB))),
        (&["test", "--set"], Err(usage("--set needs a value", "Write --set <name@version>."))),
        (&["test", "--set", "--json"], Err(usage("--set needs a value", "Write --set <name@version>."))),
        (&["test", "--rules", "t.toml"], Err(usage("unknown flag '--rules' for cctao baseline test", FLAGS))),
        (&["test", "--set=house@1"], Err(usage("unknown flag '--set=house@1' for cctao baseline test", FLAGS))),
        (&["test", "house@1"], Err(usage("unexpected argument 'house@1' for cctao baseline test", FLAGS))),
        (&["test", "test"], Err(usage("unexpected argument 'test' for cctao baseline test", FLAGS))),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(args, want)| {
            let got = parse_baseline_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>());
            (got != want).then(|| format!("{args:?}: got {got:?}, want {want:?}"))
        })
        .collect();
    assert!(wrong.is_empty(), "rows that fail:\n{}", wrong.join("\n"));
}

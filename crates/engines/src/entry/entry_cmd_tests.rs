//! ZUT for entry_cmd.rs: the argv of a fake entry of each pick kind; the config given, not given, and not taken by
//! the entry. Also `fake_entry`, the entry the other entry ZUTs build on. Nothing runs.

use super::{entry_cmd, run_plain, Entry, Params, Pick};
use procio::Output;
use rules::Engine;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

/// An entry `lint` running `lint check <pick> <params> <config> .`, refusing nothing, never saying a check is
/// unknown, reading no hits, with `run_plain` as its flow.
pub(super) fn fake_entry(pick: Pick, params: Params, config: Option<&'static str>) -> Entry {
    Entry {
        name: "lint",
        engine: Engine::Ruff,
        program: "lint",
        needs: &["lint"],
        never: None,
        args: &["check"],
        pick,
        params,
        config,
        after: &["."],
        version: None,
        refuse: |_| None,
        unknown: |_: &Output, _: &str| None,
        read: |_, _, _, _| Ok(Vec::new()),
        run: run_plain,
    }
}

fn argv(entry: &Entry, pick: &str, params: &[&str], config: Option<&str>) -> Vec<OsString> {
    let params: Vec<OsString> = params.iter().map(OsString::from).collect();
    let cmd = entry_cmd(entry, pick.as_ref(), &params, config, Path::new("/no/such/folder"));
    assert_eq!(cmd.program, OsString::from("lint"));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/no/such/folder")));
    cmd.args
}

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn an_arg_pick_joins_the_check_to_its_flag_between_the_fixed_args_then_params_config_and_after() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), Some("--config="));
    let got = argv(&entry, "E711", &["--allow=E1", "--allow=E2"], Some("ruff.toml"));
    assert_eq!(got, os(&["check", "--select=E711", "--allow=E1", "--allow=E2", "--config=ruff.toml", "."]));
}

#[test]
fn a_check_beginning_with_a_dash_stays_one_argument_with_its_flag() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), None);
    assert_eq!(argv(&entry, "--fix", &[], None), os(&["check", "--select=--fix", "."]));
}

#[test]
fn no_config_given_adds_no_config_argument() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), Some("--config="));
    assert_eq!(argv(&entry, "E711", &[], None), os(&["check", "--select=E711", "."]));
}

#[test]
fn an_entry_that_takes_no_config_drops_the_rules_config() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), None);
    assert_eq!(argv(&entry, "E711", &[], Some("ruff.toml")), os(&["check", "--select=E711", "."]));
}

#[test]
fn a_file_pick_passes_the_option_then_the_path_as_two_arguments() {
    let entry = fake_entry(Pick::File("--rule"), Params::Template, None);
    let got = argv(&entry, "/tmp/cctao-ast-grep-1-0.yml", &[], None);
    assert_eq!(got, os(&["check", "--rule", "/tmp/cctao-ast-grep-1-0.yml", "."]));
}

#[test]
fn a_none_pick_passes_no_check() {
    let entry = fake_entry(Pick::None, Params::Args(&[]), None);
    assert_eq!(argv(&entry, "slop", &[], None), os(&["check", "."]));
}

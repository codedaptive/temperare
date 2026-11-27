//! ZIT for threshold_run.rs on real temp files: a `File` reach writes the setting and passes it after the engine's
//! own config when the rule gives none, and after the rule's config when it does; a `Dir` reach writes the setting
//! in a folder named in the environment; both are removed when the value is dropped.

use crate::entry::entry_cmd_tests::fake_entry;
use crate::entry::{threshold_run, Entry, Params, Pick, Reach, TempDir, Threshold};
use rules::{Rule, RuleFile};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

static NESTING: [Threshold; 1] =
    [Threshold { name: "max_nesting", check: "nesting", max: 9, setting: |n| format!("nesting:\n  max: {n}\n") }];

fn limited(reach: Reach) -> Entry {
    let params = Params::Thresholds { args: &[], thresholds: &NESTING, reach };
    fake_entry(Pick::Arg("--only="), params, Some("--config="))
}

const FILE: Reach = Reach::File { option: "--config=", ext: "yml", own: Some(".lint.yml") };

fn rule(extra: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"swiftlint\"\ncheck = \"nesting\"\nfix = \"f\"\n{extra}");
    RuleFile::parse(&text, Path::new("/no/such/folder")).unwrap().rules.remove(0)
}

/// The path in an argument `--config=<path>`.
fn config_path(arg: &OsString) -> PathBuf {
    PathBuf::from(arg.to_str().unwrap().strip_prefix("--config=").unwrap())
}

#[test]
fn a_file_reach_writes_the_setting_and_removes_it_when_dropped() {
    let run = threshold_run(&limited(FILE), &rule("params = { max_nesting = 4 }\n"), Path::new("/no/such/folder"));
    let run = run.unwrap().unwrap();
    assert_eq!(run.args.len(), 1, "{:?}", run.args);
    let path = config_path(&run.args[0]);
    assert_eq!(std::fs::read_to_string(&path).unwrap(), "nesting:\n  max: 4\n");
    assert_eq!(path.parent(), Some(std::env::temp_dir().as_path()));
    let name = path.file_name().unwrap().to_str().unwrap().to_string();
    assert!(name.starts_with(&format!("cctao-lint-{}-", std::process::id())) && name.ends_with(".yml"), "{name}");
    assert!(run.env.is_empty(), "{:?}", run.env);
    drop(run);
    assert!(!path.exists());
}

#[test]
fn a_file_reach_passes_the_engines_own_config_first_when_the_rule_gives_none() {
    let folder = TempDir::write("own", ".lint.yml", "excluded: [x]\n").unwrap();
    let run = threshold_run(&limited(FILE), &rule("params = { max_nesting = 4 }\n"), &folder.path);
    let run = run.unwrap().unwrap();
    assert_eq!(run.args.len(), 2, "{:?}", run.args);
    assert_eq!(config_path(&run.args[0]), folder.path.join(".lint.yml"));
    assert_eq!(std::fs::read_to_string(config_path(&run.args[1])).unwrap(), "nesting:\n  max: 4\n");
}

#[test]
fn a_file_reach_passes_the_rules_config_first_and_not_the_engines_own() {
    let folder = TempDir::write("own", ".lint.yml", "excluded: [x]\n").unwrap();
    let rule = rule("config = \"strict.yml\"\nparams = { max_nesting = 4 }\n");
    let run = threshold_run(&limited(FILE), &rule, &folder.path).unwrap().unwrap();
    assert_eq!(run.args.len(), 2, "{:?}", run.args);
    assert_eq!(run.args[0], OsString::from("--config=strict.yml"));
    assert_eq!(std::fs::read_to_string(config_path(&run.args[1])).unwrap(), "nesting:\n  max: 4\n");
}

#[test]
fn a_dir_reach_writes_the_setting_in_a_folder_named_in_the_env_and_removes_it_when_dropped() {
    let entry = limited(Reach::Dir { env: "LINT_CONF_DIR", file: "lint.toml" });
    let run = threshold_run(&entry, &rule("params = { max_nesting = 2 }\n"), Path::new("/no/such/folder"));
    let run = run.unwrap().unwrap();
    assert!(run.args.is_empty(), "{:?}", run.args);
    assert_eq!(run.env.len(), 1, "{:?}", run.env);
    assert_eq!(run.env[0].0, OsString::from("LINT_CONF_DIR"));
    let dir = PathBuf::from(&run.env[0].1);
    assert_eq!(dir.parent(), Some(std::env::temp_dir().as_path()));
    assert_eq!(std::fs::read_to_string(dir.join("lint.toml")).unwrap(), "nesting:\n  max: 2\n");
    drop(run);
    assert!(!dir.exists());
}

#[test]
fn a_dir_reach_still_passes_the_rules_config() {
    let entry = limited(Reach::Dir { env: "LINT_CONF_DIR", file: "lint.toml" });
    let rule = rule("config = \"strict.yml\"\nparams = { max_nesting = 2 }\n");
    let run = threshold_run(&entry, &rule, Path::new("/no/such/folder")).unwrap().unwrap();
    assert_eq!(run.args, [OsString::from("--config=strict.yml")]);
}

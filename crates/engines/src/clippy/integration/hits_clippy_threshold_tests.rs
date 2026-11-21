//! Integration tests for hits_clippy.rs with a `max_nesting` threshold: the real `cargo_root` reads a canonical temp folder holding
//! an empty `Cargo.toml`, and the real `threshold_run` writes its `clippy.toml` into a real temporary folder. The fake
//! host answers clippy-driver help with the captured `lint_groups_help.txt`, locate-project with the temp folder's
//! manifest, and the clippy check run with a finished build, reading the threshold file while the run is on. A test
//! that makes a folder removes it before asserting.

use crate::clippy::hits_clippy_tests::{output, rules_with};
use crate::clippy::{hits_clippy, CLIPPY};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

/// A fresh, canonical temp folder holding an empty `Cargo.toml`.
fn crate_folder(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-hits-clippy-threshold-{name}", std::process::id()));
    fs::create_dir(&root).unwrap();
    fs::write(root.join("Cargo.toml"), "").unwrap();
    fs::canonicalize(root).unwrap()
}

/// What the fake saw of the clippy check run: its arguments, its environment, and the text of `clippy.toml` in the
/// folder its `CLIPPY_CONF_DIR` names, read during the run.
struct Seen {
    args: Vec<OsString>,
    env: Vec<(OsString, OsString)>,
    during: Option<String>,
}

/// `hits_clippy` for check `clippy::excessive_nesting` with `params` over a fresh crate folder, removed again before
/// this returns.
fn run_nesting(name: &str, params: &str) -> (Result<Vec<Hit>, Stop>, Vec<Seen>) {
    let root = crate_folder(name);
    let rules = rules_with(params).replace("clippy::bool_comparison", "clippy::excessive_nesting");
    let file = RuleFile::parse(&rules, &root).unwrap();
    let manifest = format!("{}\n", root.join("Cargo.toml").display()).into_bytes();
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        match (cmd.program.to_str(), cmd.args[0].to_str()) {
            (Some("clippy-driver"), Some("-W")) => Ok(output(0, include_bytes!("../fixtures/lint_groups_help.txt"), b"")),
            (Some("cargo"), Some("locate-project")) => Ok(output(0, &manifest, b"")),
            (Some("cargo"), Some("clippy")) => {
                let dir = cmd.env.iter().find(|(key, _)| key == "CLIPPY_CONF_DIR").map(|(_, dir)| PathBuf::from(dir));
                let during = dir.and_then(|dir| fs::read_to_string(dir.join("clippy.toml")).ok());
                seen.borrow_mut().push(Seen { args: cmd.args.clone(), env: cmd.env.clone(), during });
                Ok(output(0, br#"{"reason":"build-finished","success":true}"#, b""))
            }
            _ => panic!("unexpected command: {cmd:?}"),
        }
    };
    let host = Host { run: &run, on_path: &|_| panic!("unexpected PATH lookup"), raw_chars: &Cell::new(None) };
    let got = hits_clippy(&CLIPPY, &file.rules[0], &root, &host);
    fs::remove_dir_all(&root).unwrap();
    (got, seen.into_inner())
}

/// The one variable of `env`, which must be `CLIPPY_CONF_DIR`, as a path.
fn conf_dir(env: &[(OsString, OsString)]) -> &Path {
    assert_eq!(env.len(), 1, "{env:?}");
    assert_eq!(env[0].0, "CLIPPY_CONF_DIR");
    Path::new(&env[0].1)
}

#[test]
fn max_nesting_3_is_clippy_toml_in_the_clippy_conf_dir_during_the_run_and_the_folder_is_gone_after() {
    let (got, seen) = run_nesting("three", "{ max_nesting = 3 }");
    assert_eq!(got, Ok(vec![]));
    assert_eq!(seen.len(), 1);
    let want = ["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "--warn=clippy::excessive_nesting"];
    assert_eq!(seen[0].args, want.map(OsString::from));
    assert_eq!(seen[0].during.as_deref(), Some("excessive-nesting-threshold = 3\n"));
    let dir = conf_dir(&seen[0].env);
    assert!(!dir.exists(), "threshold folder remains at {}", dir.display());
}

#[test]
fn allow_still_reaches_clippy_beside_max_nesting() {
    let (got, seen) = run_nesting("allow", r#"{ allow = ["x"], max_nesting = 7 }"#);
    assert_eq!(got, Ok(vec![]));
    assert_eq!(seen.len(), 1);
    let tail: Vec<_> = seen[0].args.iter().skip(5).filter_map(|arg| arg.to_str()).collect();
    assert_eq!(tail, ["--warn=clippy::excessive_nesting", "--allow=x"]);
    assert_eq!(seen[0].during.as_deref(), Some("excessive-nesting-threshold = 7\n"));
    let dir = conf_dir(&seen[0].env);
    assert!(!dir.exists(), "threshold folder remains at {}", dir.display());
}

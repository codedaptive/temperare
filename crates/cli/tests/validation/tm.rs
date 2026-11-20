//! Runs the built `temperare` in a throwaway repository and reads its `--json` answer (the report, plus
//! `exit` and `next`; nothing else on stdout).

use serde_json::Value;

use crate::env::clean_env;
use crate::repo::Repo;
use crate::run_clean::run_clean;

pub struct Answer {
    pub code: i32,
    pub json: Value,
    pub stderr: String,
}

impl Answer {
    /// The kept findings.
    pub fn findings(&self) -> &Vec<Value> {
        self.json["findings"]
            .as_array()
            .unwrap_or_else(|| panic!("no findings list in {:#}", self.json))
    }

    /// The (path, line) of each kept finding, sorted.
    pub fn places(&self) -> Vec<(String, u64)> {
        let mut places: Vec<(String, u64)> = self
            .findings()
            .iter()
            .map(|f| (f["path"].as_str().unwrap_or("").to_string(), f["line"].as_u64().unwrap_or(0)))
            .collect();
        places.sort();
        places
    }
}

/// `tm <args>` in `repo`, in the clean environment.
pub fn tm(repo: &Repo, args: &[&str]) -> Answer {
    tm_with_path(repo, args, None)
}

/// As `tm`, with PATH replaced whole.
pub fn tm_with_path(repo: &Repo, args: &[&str], path: Option<&str>) -> Answer {
    let out = run_clean(env!("CARGO_BIN_EXE_temperare"), args, repo.path(), clean_env(path));
    let code = out.code.expect("tm ended by a signal");
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let json: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!(
            "tm {args:?} printed no JSON ({e}); exit {code}\nstdout: {}\nstderr: {stderr}",
            String::from_utf8_lossy(&out.stdout)
        )
    });
    assert_eq!(json["exit"], code, "the JSON's exit is the process's exit code: {json:#}");
    assert!(
        json["next"].as_str().is_some_and(|s| !s.is_empty()),
        "every answer names the next step: {json:#}"
    );
    Answer { code, json, stderr }
}

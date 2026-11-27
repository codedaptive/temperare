//! Done 9 (cargo-audit) (SPEC-engines-common Done 9): a rule naming the engine reports a known violation, in a file
//! the test made, with its rule, its place and its fix text, and the report lists the engine's version; with the
//! engine off the PATH, the same rule reports "not available" (exit 3). One security rule, `engine = "cargo-audit"`,
//! `check = "RUSTSEC-2021-0003"`, under a top-level owner; Cargo.lock (the engines test's) names smallvec 1.6.0, which
//! that advisory covers. Every cargo-audit finding is on Cargo.lock with no line: `--json` writes the line as null and
//! the text prints the path alone. engines runs cargo-audit with `--no-fetch` and `--no-yanked`, which keep it off the
//! network: `cctao` runs with HOME a folder the test made, whose .cargo/advisory-db links to the real HOME's, and
//! no CARGO_HOME, so engines finds the database from HOME and the test writes nothing to the real HOME. cctao runs with
//! PATH the folder that gives it cargo-audit (`cctao_path_folder`) then /usr/bin:/bin. The reported case runs on unix
//! only, where it makes that link. It is NOT RUN, with `cctao`'s reason, never PASS, when cargo-audit is not on
//! the PATH `cctao` gets (`on_cctao_path`, which runs nothing) or the real database has no `crates` folder;
//! `cctao` must then report the rule not available.

use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::cctao_with_path;

// Unix only: what the advisory case alone uses (it links the advisory database into HOME with a unix symlink).
#[cfg(unix)]
use {
    crate::absent_engine_reason::absent_engine_reason,
    crate::env::cctao_env,
    crate::probed_version::probed_version,
    crate::run_clean::run_clean,
    crate::temp_home::TempHome,
    crate::cctao::Answer,
    crate::cctao_path_folder::cctao_path_folder,
    serde_json::{json, Value},
    std::path::Path,
};

const RULE: &str = "no-vulnerable-crate";
const FIX: &str = "Upgrade smallvec to 1.6.1 or later.";

/// The engines test's lock: a root package depending on smallvec 1.6.0, which RUSTSEC-2021-0003 covers.
const VULNERABLE: &str = "version = 3\n\n[[package]]\nname = \"probe\"\nversion = \"0.1.0\"\ndependencies = [\"smallvec\"]\n\n[[package]]\nname = \"smallvec\"\nversion = \"1.6.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n";

// git is in /usr/bin; cargo-audit is in neither folder (it is installed elsewhere, such as /opt/homebrew/bin).
const PATH_WITHOUT_ENGINES: &str = "/usr/bin:/bin";

/// The rule file: one security rule of advisory RUSTSEC-2021-0003, owned by the file's top-level owner.
fn rules(fix: &str) -> String {
    format!(
        "version = 1\nowner = \"platform\"\n\n[[rule]]\nid = \"{RULE}\"\nengine = \"cargo-audit\"\n\
         check = \"RUSTSEC-2021-0003\"\nkind = \"security\"\nseverity = \"error\"\nfix = \"{fix}\"\n"
    )
}

/// A repository holding the rule file and Cargo.lock, committed. Without Cargo.lock the rule has no file to check,
/// which `cctao` settles before it looks for the engine.
fn lock_repo() -> Repo {
    let repo = Repo::new();
    repo.write("tao.toml", &rules(FIX));
    repo.write("Cargo.lock", VULNERABLE);
    repo.commit("a vulnerable lock");
    repo
}

/// `cctao <args>` in `repo` with HOME `home` and no CARGO_HOME, so engines' database is `<home>/.cargo/advisory-db`.
// Unix only: its one caller is the advisory case.
#[cfg(unix)]
fn run_cctao(repo: &Repo, args: &[&str], path: &str, home: &Path) -> procio::Output {
    let env = cctao_env(Some(path), home).into_iter().filter(|(k, _)| k != "CARGO_HOME").collect();
    run_clean(env!("CARGO_BIN_EXE_cctao"), args, repo.path(), env)
}

// Unix only: it links the real advisory database into the test HOME with a unix symlink.
#[cfg(unix)]
#[test]
fn a_cargo_audit_advisory_is_reported_on_cargo_lock_with_no_line() {
    let mut check = Check::new(
        "a_cargo_audit_advisory_is_reported_on_cargo_lock_with_no_line",
        "rule no-vulnerable-crate (cargo-audit, RUSTSEC-2021-0003, kind security), rule file owner platform; \
         Cargo.lock names smallvec 1.6.0; HOME's .cargo/advisory-db links to the real one, no CARGO_HOME; PATH is \
         cargo-audit's folder then /usr/bin:/bin; run with --json, then as text",
    );
    let repo = lock_repo();
    let real_db = Path::new(&std::env::var("HOME").expect("HOME is set")).join(".cargo/advisory-db");
    let home = TempHome::new();
    std::fs::create_dir(home.path().join(".cargo")).expect("make the test HOME's .cargo");
    std::os::unix::fs::symlink(&real_db, home.path().join(".cargo/advisory-db")).expect("link the advisory database");
    let path = cctao_path_folder("cargo-audit")
        .map_or(PATH_WITHOUT_ENGINES.to_string(), |dir| format!("{dir}:{PATH_WITHOUT_ENGINES}"));
    let args = ["check", "--json"];
    let out = run_cctao(&repo, &args, &path, home.path());
    let code = out.code.expect("cctao ended by a signal");
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let json: Value = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!("cctao {args:?} printed no JSON ({e}); exit {code}\nstdout: {}\nstderr: {stderr}", String::from_utf8_lossy(&out.stdout))
    });
    let a = Answer { code, json, stderr };
    assert_eq!(a.json["exit"], a.code, "the JSON's exit is the process's exit code: {:#}", a.json);
    assert!(
        a.json["next"].as_str().is_some_and(|s| !s.is_empty()),
        "every answer names the next step (mission 7): {:#}",
        a.json
    );
    check.saw_cctao(&args, a.code, &a.json);
    if let Some(why) = absent_engine_reason(&a, &["cargo-audit"], RULE) {
        check.not_run(why);
        return;
    }
    if !real_db.join("crates").is_dir() {
        let why = a.json["not_available"]
            .as_array()
            .and_then(|list| list.iter().find(|n| n["rule"] == RULE))
            .and_then(|n| n["why"].as_str())
            .unwrap_or_else(|| panic!("{} has no crates folder, yet cctao did not report {RULE}: {:#}", real_db.display(), a.json));
        check.not_run(format!("cctao: {why}"));
        return;
    }
    assert_eq!(a.code, 4, "a security finding never refuses (S-15): {:#}\n{}", a.json, a.stderr);
    let f = a.findings();
    assert_eq!(f.len(), 1, "one finding, on Cargo.lock: {:#}", a.json);
    assert_eq!(f[0]["rule"], RULE);
    assert_eq!(f[0]["engine"], "cargo-audit");
    assert_eq!(f[0]["path"], "Cargo.lock");
    assert_eq!(f[0].get("line"), Some(&Value::Null), "a finding with no line has line null: {:#}", a.json);
    assert_eq!(f[0]["fix"], FIX);
    let version = probed_version("cargo-audit", &["--version"], repo.path());
    assert_eq!(a.json["engines"], json!([{ "engine": "cargo-audit", "version": version }]), "{:#}", a.json);

    let out = run_cctao(&repo, &["check"], &path, home.path());
    let text = format!("{}\n{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert_eq!(out.code, Some(4), "the text run exits as the --json run: {text}");
    assert!(text.lines().any(|line| line.starts_with("Cargo.lock  ")), "the place is Cargo.lock alone: {text}");
    assert!(!text.contains("Cargo.lock:"), "no line or column follows Cargo.lock: {text}");
    assert!(!text.contains("null"), "the text never prints null: {text}");
}

#[test]
fn cargo_audit_off_path_is_not_available() {
    let mut check = Check::new(
        "cargo_audit_off_path_is_not_available",
        "rule no-vulnerable-crate (cargo-audit, RUSTSEC-2021-0003, kind security) over the same files; cctao runs with \
         PATH /usr/bin:/bin, which holds no cargo-audit",
    );
    let repo = lock_repo();
    let args = ["check", "--json"];
    let a = cctao_with_path(&repo, &args, Some(PATH_WITHOUT_ENGINES));
    check.saw_cctao(&args, a.code, &a.json);
    assert_eq!(a.code, 3, "not available has its own code: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing is reported as found: {:#}", a.json);
    let missing: Vec<(&str, &str)> = a.json["not_available"]
        .as_array()
        .unwrap_or_else(|| panic!("no not_available list: {:#}", a.json))
        .iter()
        .map(|n| (n["rule"].as_str().unwrap_or(""), n["engine"].as_str().unwrap_or("")))
        .collect();
    assert_eq!(missing, vec![(RULE, "cargo-audit")], "{:#}", a.json);
}

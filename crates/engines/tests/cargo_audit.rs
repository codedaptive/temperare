#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::PathBuf;
use std::sync::Mutex;

use engines::{no_file_rules, run_rules_versioned_at, NoFileRule, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());
const ROOT: &str = "version = 3\n\n[[package]]\nname = \"probe\"\nversion = \"0.1.0\"\n";
const VULNERABLE: &str = "version = 3\n\n[[package]]\nname = \"probe\"\nversion = \"0.1.0\"\ndependencies = [\"smallvec\"]\n\n[[package]]\nname = \"smallvec\"\nversion = \"1.6.0\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n";
const WHAT: &str = "RUSTSEC-2021-0003: smallvec 1.6.0: Buffer overflow in SmallVec::insert_many; upgrade to >=0.6.14, <1.0.0 or >=1.6.1";

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(case: &str, purpose: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-cargo-audit-{case}-{purpose}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

struct EnvGuard {
    cargo_home: Option<OsString>,
    path: Option<OsString>,
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        for (key, value) in [("CARGO_HOME", &self.cargo_home), ("PATH", &self.path)] {
            match value {
                Some(value) => std::env::set_var(key, value),
                None => std::env::remove_var(key),
            }
        }
    }
}

#[derive(Clone, Copy)]
enum Case {
    Advisory,
    Vulnerability,
    Clean,
    MissingDatabase,
    MissingEngine,
    MissingLock,
    UnknownAdvisory,
}

fn check_case(name: &str, case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let env = EnvGuard { cargo_home: std::env::var_os("CARGO_HOME"), path: std::env::var_os("PATH") };
    let repo = TempFolder::new(name, "repo");
    let home = TempFolder::new(name, "home");
    if !matches!(case, Case::MissingDatabase) {
        let db = PathBuf::from(std::env::var_os("HOME").unwrap()).join(".cargo/advisory-db");
        symlink(db, home.0.join("advisory-db")).unwrap();
    }
    std::env::set_var("CARGO_HOME", &home.0);
    git_support::git(&repo.0, &["init", "-q"], b"");
    fs::write(repo.0.join("tracked.txt"), "tracked\n").unwrap();
    git_support::git(&repo.0, &["add", "tracked.txt"], b"");
    if !matches!(case, Case::MissingLock) {
        let lock = if matches!(case, Case::Advisory | Case::Vulnerability) { VULNERABLE } else { ROOT };
        fs::write(repo.0.join("Cargo.lock"), lock).unwrap();
        git_support::git(&repo.0, &["add", "-f", "Cargo.lock"], b"");
    }
    let check = match case {
        Case::Advisory => "RUSTSEC-2021-0003",
        Case::Vulnerability => "vulnerability",
        Case::UnknownAdvisory => "RUSTSEC-2099-9999",
        _ => "all",
    };
    fs::write(repo.0.join("temperare.toml"), format!(
        "version = 1\nowner = \"probe\"\n[[rule]]\nid = \"audit\"\nengine = \"cargo-audit\"\ncheck = \"{check}\"\npaths = [\"**\"]\nkind = \"security\"\nseverity = \"warning\"\nfix = \"Upgrade the dependency.\"\n"
    )).unwrap();
    if matches!(case, Case::MissingEngine | Case::MissingLock) {
        let git = std::env::split_paths(env.path.as_ref().unwrap()).map(|dir| dir.join("git"))
            .find(|p| p.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)).unwrap();
        let bin = repo.0.join("bin");
        fs::create_dir(&bin).unwrap();
        symlink(fs::canonicalize(git).unwrap(), bin.join("git")).unwrap();
        std::env::set_var("PATH", bin);
    }
    let file = RuleFile::read(&repo.0.join("temperare.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    let no_files = if matches!(case, Case::MissingLock) { no_file_rules(&file, &repo.0) } else { vec![] };
    let db = home.0.join("advisory-db");
    let missing_db_why = format!("no advisory database at {}: fetch it with `cargo audit update-db --db {}` (tm never fetches it)", db.display(), db.display());
    drop(env);
    drop(repo);
    drop(home);

    assert_eq!(runs.len(), 1, "{runs:?}");
    let run = &runs[0];
    assert_eq!(run.rule, "audit");
    assert_eq!(run.engine, Engine::CargoAudit);
    match case {
        Case::Advisory | Case::Vulnerability => {
            assert_eq!(run.not_available_cause, None);
            let RuleOutcome::Findings(findings) = &run.outcome else { panic!("expected findings: {run:?}") };
            assert_eq!(findings.len(), 1, "{findings:?}");
            assert_eq!(findings[0].rule, "audit");
            assert_eq!(findings[0].engine, Engine::CargoAudit);
            assert_eq!(findings[0].path, RelPath::new("Cargo.lock"));
            assert_eq!(findings[0].line(), None);
            assert_eq!(findings[0].what, WHAT);
        }
        Case::Clean => assert_eq!(run.outcome, RuleOutcome::Findings(vec![])),
        Case::MissingLock => {
            assert_eq!(run.outcome, RuleOutcome::Findings(vec![]));
            assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"));
            assert_eq!(no_files, vec![NoFileRule { rule: "audit".into(), engine: Engine::CargoAudit, why: "no file its engine checks".into() }]);
        }
        Case::MissingDatabase | Case::MissingEngine | Case::UnknownAdvisory => {
            let (cause, expected) = match case {
                Case::MissingDatabase => (NotAvailableCause::Other, missing_db_why.as_str()),
                Case::MissingEngine => (NotAvailableCause::EngineNotOnPath, "cargo-audit not found on PATH"),
                Case::UnknownAdvisory => (NotAvailableCause::UnknownCheck, "RUSTSEC-2099-9999 is not in the advisory database at"),
                _ => unreachable!(),
            };
            assert_eq!(run.not_available_cause, Some(cause));
            let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else { panic!("expected not available: {run:?}") };
            assert_eq!(rule, "audit");
            assert_eq!(*engine, Engine::CargoAudit);
            if matches!(case, Case::UnknownAdvisory) {
                assert!(why.contains(expected), "{why}");
            } else {
                assert_eq!(why, expected);
            }
        }
    }
}

macro_rules! cases {
    ($( $name:ident => $case:ident ),+ $(,)?) => {
        $(#[test] fn $name() { check_case(stringify!($name), Case::$case); })+
    };
}

cases! {
    advisory_id_finds_smallvec => Advisory,
    vulnerability_kind_finds_smallvec => Vulnerability,
    clean_lock_has_no_findings => Clean,
    missing_database_is_not_available => MissingDatabase,
    missing_engine_is_not_available => MissingEngine,
    missing_lock_precedes_missing_engine => MissingLock,
    unknown_advisory_is_not_available => UnknownAdvisory,
}

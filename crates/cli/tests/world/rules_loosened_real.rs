//! RIT: `tm rules --loosened` compares working files with real git commits in disposable repositories.
//! RuleNotApplicable and SetOlder cannot be made by today's real rule files; formatter ZUTs cover them.

use serde_json::{json, Value};
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-loosened-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(fs::canonicalize(path).unwrap()),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

struct Repo {
    temp: TempFolder,
    repo: PathBuf,
    home: PathBuf,
}

impl Repo {
    fn new(init: bool) -> Self {
        let temp = TempFolder::new();
        let repo = temp.0.join("repo");
        let home = temp.0.join("home");
        fs::create_dir(&repo).unwrap();
        fs::create_dir(&home).unwrap();
        let result = Self { temp, repo, home };
        if init {
            result.git(&["init", "-q"]);
        }
        result
    }

    fn git(&self, args: &[&str]) -> String {
        let mut cmd = procio::Cmd::new("git");
        cmd.cwd = Some(self.repo.clone());
        cmd.env_remove = std::env::vars_os().map(|(name, _)| name).collect();
        cmd.env = vec![
            ("PATH".into(), std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into())),
            ("HOME".into(), self.home.as_os_str().into()),
            ("TMPDIR".into(), self.temp.0.as_os_str().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ];
        cmd.args = ["-c", "core.hooksPath=/dev/null", "-c", "user.name=Test", "-c", "user.email=test@example.invalid", "-c", "commit.gpgsign=false"]
            .iter().chain(args.iter()).map(OsString::from).collect();
        let output = procio::run(&cmd).unwrap();
        assert_eq!(output.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }

    fn write(&self, text: &str) {
        fs::write(self.repo.join("temperare.toml"), text).unwrap();
    }

    fn commit(&self, base: Option<&str>) -> String {
        if let Some(text) = base {
            self.write(text);
        }
        self.git(&["add", "-A"]);
        self.git(&["commit", "--allow-empty", "-q", "-m", "base"]);
        self.git(&["rev-parse", "HEAD"])
    }

    fn run(&self, args: &[&str]) -> (u8, String, String) {
        let args = args.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = cli::run(&args, &self.repo, &mut out, &mut err);
        (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
    }

    fn path(&self) -> String {
        self.repo.join("temperare.toml").display().to_string()
    }
}

const BASE: &str = r#"version = 1
baseline = ["standard@1"]
[kits.a]
paths = ["a/**"]
[kits.b]
paths = ["b/**"]
[kits.x]
paths = ["x/**"]
[[rule]]
id = "z-scope"
engine = "ruff"
check = "E1"
fix = "fix"
paths = ["z/**", "a/**"]
kits = ["b", "a"]
[[rule]]
id = "m-gone"
engine = "ruff"
check = "E2"
fix = "fix"
[[rule]]
id = "a-limited"
engine = "ruff"
check = "E3"
fix = "fix"
[[exception]]
rule = "z-scope"
path = "late.py"
reason = "legacy"
approved = "team"
expires = "2030-01-01"
[[exception]]
rule = "z-scope"
path = "drop.py"
reason = "legacy"
approved = "team"
expires = "2031-02-03"
"#;

const NOW: &str = r#"version = 1
baseline = ["standard@1"]
[kits.a]
paths = ["a/**"]
[kits.b]
paths = ["b/**"]
[kits.x]
paths = ["x/**"]
[[override]]
rule = "tm-rust-dead-code"
off = true
[[rule]]
id = "z-scope"
engine = "ruff"
check = "E1"
fix = "fix"
severity = "warning"
paths = ["a/**"]
exclude = ["gen/**"]
kits = ["a"]
outside_kits = ["x"]
[[rule]]
id = "a-limited"
engine = "ruff"
check = "E3"
fix = "fix"
paths = ["a/**", "b/**"]
kits = ["b", "a"]
[[exception]]
rule = "a-limited"
path = "new.py"
what = "E3"
reason = "legacy"
approved = "team"
[[exception]]
rule = "z-scope"
path = "late.py"
reason = "legacy"
approved = "team"
expires = "2032-01-01"
[[exception]]
rule = "z-scope"
path = "drop.py"
reason = "legacy"
approved = "team"
"#;

#[test]
fn all_reachable_rule_and_exception_kinds_in_order() {
    let repo = Repo::new(true);
    repo.commit(Some(BASE));
    repo.git(&["branch", "base"]);
    repo.write(NOW);
    let (code, out, err) = repo.run(&["rules", "--loosened", "--base", "base", "--json"]);
    assert_eq!(code, 1, "stdout: {out}\nstderr: {err}");
    assert_eq!(serde_json::from_str::<Value>(&out).unwrap(), json!({
        "rule_file": repo.path(), "base": "base", "count": 12, "exit": 1,
        "next": format!("have each loosening in loosened approved, or undo it in {}", repo.path()),
        "loosened": [
            {"kind":"rule_gone", "about":"tm-rust-dead-code"},
            {"kind":"severity_lowered", "about":"z-scope", "before":"error", "after":"warning"},
            {"kind":"path_removed", "about":"z-scope", "before":"z/**"},
            {"kind":"exclude_added", "about":"z-scope", "after":"gen/**"},
            {"kind":"kit_removed", "about":"z-scope", "before":"b"},
            {"kind":"outside_kit_added", "about":"z-scope", "after":"x"},
            {"kind":"rule_gone", "about":"m-gone"},
            {"kind":"paths_limited", "about":"a-limited", "after":"a/**, b/**"},
            {"kind":"kits_limited", "about":"a-limited", "after":"b, a"},
            {"kind":"exception_added", "about":"a-limited", "after":"new.py (E3)"},
            {"kind":"expiry_later", "about":"z-scope", "before":"2030-01-01", "after":"2032-01-01"},
            {"kind":"expiry_dropped", "about":"z-scope", "before":"2031-02-03"}
        ]
    }));
    assert!(err.is_empty(), "{err}");
    let (code, out, err) = repo.run(&["rules", "--loosened", "--base", "base"]);
    assert_eq!(code, 1, "stdout: {out}\nstderr: {err}");
    assert_eq!(out, format!(concat!(
        "rule file: {}\nbase: base\nloosened (12):\n",
        "  rule_gone  tm-rust-dead-code\n",
        "  severity_lowered  z-scope  before: error  after: warning\n",
        "  path_removed  z-scope  before: z/**\n",
        "  exclude_added  z-scope  after: gen/**\n",
        "  kit_removed  z-scope  before: b\n",
        "  outside_kit_added  z-scope  after: x\n",
        "  rule_gone  m-gone\n",
        "  paths_limited  a-limited  after: a/**, b/**\n",
        "  kits_limited  a-limited  after: b, a\n",
        "  exception_added  a-limited  after: new.py (E3)\n",
        "  expiry_later  z-scope  before: 2030-01-01  after: 2032-01-01\n",
        "  expiry_dropped  z-scope  before: 2031-02-03\n",
        "next: have each loosening above approved, or undo it in {}\n"
    ), repo.path(), repo.path()));
    assert!(err.is_empty(), "{err}");
}

#[test]
fn dropping_a_set_follows_its_removed_rules() {
    let repo = Repo::new(true);
    let base = repo.commit(Some("version = 1\nbaseline = [\"standard@1\"]\n"));
    repo.write("version = 1\n");
    let (code, out, err) = repo.run(&["rules", "--loosened", "--base", &base, "--json"]);
    assert_eq!(code, 1, "stdout: {out}\nstderr: {err}");
    let value: Value = serde_json::from_str(&out).unwrap();
    let entries = value["loosened"].as_array().unwrap();
    assert_eq!(entries.last(), Some(&json!({"kind":"set_dropped", "about":"standard", "before":"1"})));
    assert!(entries.len() > 1);
    assert!(entries[..entries.len() - 1].iter().all(|entry| entry["kind"] == "rule_gone"));
}

const TIGHT_BASE: &str = r#"version = 1
[[rule]]
id = "r"
engine = "ruff"
check = "E1"
severity = "warning"
fix = "fix"
[[exception]]
rule = "r"
path = "old.py"
reason = "legacy"
approved = "team"
"#;
const TIGHT_NOW: &str = r#"version = 1
[[rule]]
id = "r"
engine = "ruff"
check = "E1"
severity = "error"
fix = "fix"
"#;

fn no_loosening(now: &str) {
    let repo = Repo::new(true);
    let base = repo.commit(Some(TIGHT_BASE));
    repo.write(now);
    for json in [false, true] {
        let mut args = vec!["rules", "--loosened", "--base", &base];
        if json { args.push("--json"); }
        let (code, out, err) = repo.run(&args);
        assert_eq!(code, 0, "stdout: {out}\nstderr: {err}");
        if json {
            assert_eq!(serde_json::from_str::<Value>(&out).unwrap(), json!({
                "rule_file": repo.path(), "base": base, "loosened": [], "count": 0,
                "exit": 0, "next": "nothing to do"
            }));
        } else {
            assert_eq!(out, format!("rule file: {}\nbase: {}\nloosened (0):\nnext: nothing to do\n", repo.path(), base));
        }
        assert!(err.is_empty(), "{err}");
    }
}

macro_rules! row {
    ($name:ident, $function:ident, $value:expr) => {
        #[test]
        fn $name() { $function($value); }
    };
}
row!(unchanged_is_not_loosened, no_loosening, TIGHT_BASE);
row!(tighter_is_not_loosened, no_loosening, TIGHT_NOW);

#[derive(Clone, Copy)]
enum ErrorCase { MissingRevision, MissingFile, InvalidBaseToml, InvalidBaseCheck, InvalidNow, MissingBase, BaseAlone, OutsideGit }

fn error_case(case: ErrorCase) {
    let repo = Repo::new(!matches!(case, ErrorCase::OutsideGit));
    let base = if matches!(case, ErrorCase::OutsideGit) {
        String::new()
    } else {
        repo.commit(match case {
            ErrorCase::MissingFile => None,
            ErrorCase::InvalidBaseToml => Some("version = [\n"),
            ErrorCase::InvalidBaseCheck => Some("version = 2\n"),
            _ => Some("version = 1\n"),
        })
    };
    repo.write(if matches!(case, ErrorCase::InvalidNow) { "version = [\n" } else { "version = 1\n" });
    let args = match case {
        ErrorCase::MissingRevision => vec!["rules", "--loosened", "--base", "nope"],
        ErrorCase::MissingBase => vec!["rules", "--loosened"],
        ErrorCase::BaseAlone => vec!["rules", "--base", &base],
        ErrorCase::OutsideGit => vec!["rules", "--loosened", "--base", "nope", "--rules", "temperare.toml"],
        _ => vec!["rules", "--loosened", "--base", &base],
    };
    let (code, out, err) = repo.run(&args);
    assert_eq!(code, 2, "stdout: {out}\nstderr: {err}");
    match case {
        ErrorCase::MissingRevision => {
            assert_eq!(err, "tm: base 'nope' does not name a commit\nnext: Pass a commit, branch or tag that exists, e.g. --base main.\n");
            let (code, out, err) = repo.run(&["rules", "--loosened", "--base", "nope", "--json"]);
            assert_eq!(code, 2, "stdout: {out}\nstderr: {err}");
            assert_eq!(serde_json::from_str::<Value>(&out).unwrap(), json!({
                "error": "base 'nope' does not name a commit", "exit": 2,
                "next": "Pass a commit, branch or tag that exists, e.g. --base main."
            }));
        }
        ErrorCase::MissingFile => assert_eq!(err, format!("tm: no rule file temperare.toml at base '{base}'\nnext: Pass a --base that has temperare.toml, or leave --loosened out.\n")),
        ErrorCase::InvalidBaseToml | ErrorCase::InvalidBaseCheck => assert!(out.contains(&format!("{} (at {base})", repo.path())), "{out}") ,
        ErrorCase::InvalidNow => assert!(out.contains(&repo.path()), "{out}"),
        ErrorCase::MissingBase => assert_eq!(err, "tm: tm rules --loosened needs --base <rev>\nnext: Pass --base <rev>, the commit to compare with.\n"),
        ErrorCase::BaseAlone => assert_eq!(err, "tm: --base is only for tm rules --loosened\nnext: Add --loosened, or leave --base out.\n"),
        ErrorCase::OutsideGit => assert!(err.starts_with("tm: git could not resolve base 'nope'"), "{err}"),
    }
}
row!(missing_revision_is_an_error, error_case, ErrorCase::MissingRevision);
row!(missing_base_file_is_an_error, error_case, ErrorCase::MissingFile);
row!(invalid_base_toml_is_an_error, error_case, ErrorCase::InvalidBaseToml);
row!(invalid_base_check_is_an_error, error_case, ErrorCase::InvalidBaseCheck);
row!(invalid_now_is_an_error, error_case, ErrorCase::InvalidNow);
row!(loosened_without_base_is_an_error, error_case, ErrorCase::MissingBase);
row!(base_without_loosened_is_an_error, error_case, ErrorCase::BaseAlone);
row!(outside_git_is_an_error, error_case, ErrorCase::OutsideGit);

use std::fs;
use std::path::{Path, PathBuf};

use engines::{availability, run_rules_versioned, Availability};
use procio::Cmd;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-no-file-flag", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

fn git(repo: &Path, args: &[&str]) {
    let out = procio::run(&Cmd {
        args: args.iter().map(|arg| (*arg).into()).collect(),
        cwd: Some(repo.to_path_buf()),
        ..Cmd::new("git")
    }).unwrap();
    assert_eq!(out.code, Some(0), "git {args:?}: {}", String::from_utf8_lossy(&out.stderr));
}

#[test]
fn no_file_flag_counts_untracked_but_not_ignored_files() {
    assert_eq!(availability(Engine::Ruff), Availability::Available);
    let repo = TempFolder::new();
    git(&repo.0, &["init", "-q"]);
    for dir in ["src", "docs", "lib", "build", "inner"] {
        fs::create_dir(repo.0.join(dir)).unwrap();
    }
    let violation = "value = None\nif value == None:\n    pass\n";
    fs::write(repo.0.join("src/a.py"), violation).unwrap();
    fs::write(repo.0.join("docs/readme.md"), "Documentation\n").unwrap();
    git(&repo.0, &["add", "src/a.py", "docs/readme.md"]);
    fs::write(repo.0.join("lib/b.py"), violation).unwrap();
    fs::write(repo.0.join(".gitignore"), "build/\n").unwrap();
    fs::write(repo.0.join("build/c.py"), violation).unwrap();
    git(&repo.0.join("inner"), &["init", "-q"]);
    fs::write(repo.0.join("inner/b.py"), violation).unwrap();
    fs::write(repo.0.join("temperare.toml"), r#"version = 1
[[rule]]
id = "untracked"
engine = "ruff"
check = "E711"
paths = ["lib/**"]
fix = "Use is None."
[[rule]]
id = "ignored"
engine = "ruff"
check = "E711"
paths = ["build/**"]
fix = "Use is None."
[[rule]]
id = "here"
engine = "ruff"
check = "E711"
paths = ["src/**"]
fix = "Use is None."
[[rule]]
id = "nested"
engine = "ruff"
check = "E711"
paths = ["inner"]
fix = "Use is None."
"#).unwrap();

    let file = RuleFile::read(&repo.0.join("temperare.toml")).unwrap();
    let runs = run_rules_versioned(&file, &repo.0, &[]).runs;
    assert_eq!(runs.len(), 4);
    assert_eq!(runs[0].rule, "untracked");
    assert_eq!(runs[0].no_files, None);
    let RuleOutcome::Findings(findings) = &runs[0].outcome else {
        panic!("expected E711 finding: {:?}", runs[0]);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, "untracked");
    assert_eq!(findings[0].path, RelPath::new("lib/b.py"));
    assert_eq!(findings[0].line(), Some(2));
    assert!(findings[0].what.contains("None"), "{:?}", findings[0]);

    assert_eq!(runs[1].rule, "ignored");
    assert_eq!(runs[1].no_files, Some("its paths match no file in the rule file's folder".to_string()));
    assert_eq!(runs[1].outcome, RuleOutcome::Findings(vec![]));
    assert_eq!(runs[1].raw_chars, None);

    assert_eq!(runs[2].rule, "here");
    assert_eq!(runs[2].no_files, None);
    let RuleOutcome::Findings(findings) = &runs[2].outcome else {
        panic!("expected E711 finding: {:?}", runs[2]);
    };
    assert_eq!(findings.len(), 1, "{findings:?}");
    assert_eq!(findings[0].rule, "here");
    assert_eq!(findings[0].path, RelPath::new("src/a.py"));
    assert_eq!(findings[0].line(), Some(2));
    assert!(findings[0].what.contains("None"), "{:?}", findings[0]);

    assert_eq!(runs[3].rule, "nested");
    assert_eq!(runs[3].no_files, Some("its paths match no file in the rule file's folder".to_string()));
    assert_eq!(runs[3].outcome, RuleOutcome::Findings(vec![]));
    assert_eq!(runs[3].raw_chars, None);
}

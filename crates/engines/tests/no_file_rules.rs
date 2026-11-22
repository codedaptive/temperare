//! ZIT for no_file_rules: in a throwaway git repository, the rules tm check settles as no-file, without running an
//! engine; outside a repository, none.

use engines::{no_file_rules, NoFileRule};
use rules::{Engine, RuleFile};
use std::fs;
use std::path::{Path, PathBuf};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-no-file-rules-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn no_file_rules_lists_the_rule_folder_without_running_engines() {
    let repo = TempFolder::new("repo");
    git_support::git(&repo.0, &["init", "-q"], b"");
    for dir in ["kit", "kit/docs", "kit/build", "kit/scripts", "kit/new", "src"] {
        fs::create_dir(repo.0.join(dir)).unwrap();
    }
    fs::write(repo.0.join(".gitignore"), "kit/build/\n").unwrap();
    fs::write(repo.0.join("kit/build/ignored.py"), "ignored\n").unwrap();
    fs::write(repo.0.join("kit/docs/readme.md"), "docs\n").unwrap();
    fs::write(repo.0.join("kit/scripts/check.sh"), "#!/bin/sh\n").unwrap();
    fs::write(repo.0.join("kit/new/untracked.py"), "pass\n").unwrap();
    fs::write(repo.0.join("src/decoy.py"), "pass\n").unwrap();
    git_support::git(&repo.0, &["add", ".gitignore", "kit/docs/readme.md", "kit/scripts/check.sh", "src/decoy.py"], b"");
    let text = r#"version = 1
[[rule]]
id = "no-script"
engine = "shellcheck"
check = "SC2086"
paths = ["docs/**"]
fix = "f"
[[rule]]
id = "ignored"
engine = "ruff"
check = "E711"
paths = ["build/**"]
fix = "f"
[[rule]]
id = "refused-no-file"
engine = "ruff"
check = "E711"
paths = ["missing/**"]
params = { nope = ["x"] }
fix = "f"
[[rule]]
id = "refused-no-script"
engine = "shellcheck"
check = "SC2086"
paths = ["docs/**"]
params = { x = ["y"] }
fix = "f"
[[rule]]
id = "base-no-file"
engine = "ruff"
check = "E711"
paths = ["src/**"]
only_with_base = true
fix = "f"
[[rule]]
id = "untracked-file"
engine = "ruff"
check = "E711"
paths = ["new/**"]
fix = "f"
[[rule]]
id = "has-script"
engine = "shellcheck"
check = "SC2086"
paths = ["scripts/**"]
fix = "f"
"#;
    let file = RuleFile::parse(text, Path::new("kit")).unwrap();
    assert_eq!(no_file_rules(&file, &repo.0), vec![
        NoFileRule { rule: "no-script".into(), engine: Engine::ShellCheck, why: "no file its engine checks".into() },
        NoFileRule { rule: "ignored".into(), engine: Engine::Ruff, why: "its paths match no file in the rule file's folder".into() },
        NoFileRule { rule: "base-no-file".into(), engine: Engine::Ruff, why: "its paths match no file in the rule file's folder".into() },
    ]);
}

#[test]
fn no_file_rules_outside_a_repository_settles_nothing() {
    let folder = TempFolder::new("outside");
    let file = RuleFile::parse(r#"version = 1
[[rule]]
id = "missing"
engine = "ruff"
check = "E711"
paths = ["missing/**"]
fix = "f"
[[rule]]
id = "shell"
engine = "shellcheck"
check = "SC2086"
fix = "f"
"#, Path::new(".")).unwrap();
    assert_eq!(no_file_rules(&file, &folder.0), Vec::<NoFileRule>::new());
}

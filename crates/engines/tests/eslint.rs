#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-eslint-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

#[test]
fn eslint_rule_reports_findings_suppressions_errors_and_absence() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let violation = "var x = 1;\nconsole.log(x);\n";
    let clean = "const x = 1;\nconsole.log(x);\n";
    let config = "export default [];\n";
    let cases = [
        ("violation", "example.js", violation, config, "no-var", 1),
        ("clean", "example.js", clean, config, "no-var", 0),
        ("inline-disable", "example.js", "// eslint-disable-next-line no-var\nvar x = 1;\nconsole.log(x);\n", config, "no-var", 1),
        ("bulk-suppression", "example.js", violation, config, "no-var", 1),
        ("config-ignores", "skip/s.js", violation, "export default [{ ignores: [\"skip/**\"] }];\n", "no-var", 1),
        ("git-ignored", "build/b.js", violation, config, "no-var", 0),
        ("unknown-core", "example.js", clean, config, "no-such-rule-xyz", 0),
        ("plugin-not-defined", "example.js", clean, config, "@typescript-eslint/no-explicit-any", 0),
        ("import-fails", "example.js", clean, "import x from \"no-such-eslint-plugin-xyz\";\nexport default [x];\n", "no-var", 0),
        ("parse-error", "broken.js", "var = ;\n", config, "no-var", 0),
        ("no-files", "notes.txt", "no JavaScript here\n", config, "no-var", 0),
        ("typescript", "a.ts", "export const x: any = 1;\n", "import tseslint from \"/opt/homebrew/lib/node_modules/typescript-eslint/dist/index.js\";\nexport default [...tseslint.configs.recommended];\n", "@typescript-eslint/no-explicit-any", 1),
        ("absent", "example.js", violation, config, "no-var", 0),
        ("absent-no-js", "notes.txt", "no JavaScript here\n", config, "no-var", 0),
        ("param", "example.js", violation, config, "no-var", 0),
    ];
    let mut failed = Vec::new();
    for (name, path, source, config, check, count) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            if name == "git-ignored" || name == "no-files" || name == "absent-no-js" {
                git_support::git(&folder.0, &["init", "-q"], b"");
                fs::write(folder.0.join(".gitignore"), if name == "git-ignored" { "build/\n" } else { "eslint.config.mjs\n" }).unwrap();
                if name == "git-ignored" {
                    fs::write(folder.0.join("tracked.js"), clean).unwrap();
                    git_support::git(&folder.0, &["add", "tracked.js"], b"");
                }
            }
            let source_path = folder.0.join(path);
            fs::create_dir_all(source_path.parent().unwrap()).unwrap();
            fs::write(source_path, source).unwrap();
            fs::write(folder.0.join("eslint.config.mjs"), config).unwrap();
            if name == "bulk-suppression" {
                let output = procio::run(&procio::Cmd {
                    args: ["--suppress-rule", "no-var", "--rule", "no-var:error", "."].into_iter().map(Into::into).collect(),
                    cwd: Some(folder.0.clone()),
                    ..procio::Cmd::new("eslint")
                }).unwrap();
                assert_eq!(output.code, Some(0), "{name}: {}", String::from_utf8_lossy(&output.stderr));
                assert!(folder.0.join("eslint-suppressions.json").is_file(), "{name}");
            }
            fs::write(folder.0.join("tao.toml"), format!(
                "version = 1\n[[rule]]\nid = \"eslint-test\"\nengine = \"eslint\"\ncheck = \"{check}\"\nfix = \"Use the recommended form.\"\n{}",
                if name == "param" { "params = { option = 1 }\n" } else if name == "absent-no-js" { "paths = [\"notes.txt\"]\n" } else { "" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("tao.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "absent-no-js" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_eslint = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("eslint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_eslint);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            match name {
                "absent" | "absent-no-js" => {
                    assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                        rule: "eslint-test".into(), engine: Engine::Eslint, why: "eslint not found on PATH".into(),
                    }, "{name}");
                    assert_eq!(runs[0].no_files, None, "{name}");
                    return;
                }
                "unknown-core" | "plugin-not-defined" => {
                    let RuleOutcome::NotAvailable { why, .. } = &runs[0].outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", runs[0].outcome);
                    };
                    assert!(why.contains(check), "{name}: {why}");
                    if name == "unknown-core" { assert!(why.contains("Could not find \"no-such-rule-xyz\" in plugin \"@\""), "{name}: {why}"); }
                    if name == "plugin-not-defined" { assert!(why.contains("could not find plugin"), "{name}: {why}"); }
                    return;
                }
                "import-fails" | "parse-error" | "param" => {
                    let RuleOutcome::Failed { code, stderr, .. } = &runs[0].outcome else {
                        panic!("{name}: expected Failed, got {:?}", runs[0].outcome);
                    };
                    if name == "import-fails" { assert!(stderr.contains("Cannot find package 'no-such-eslint-plugin-xyz'"), "{name}: {stderr}"); }
                    if name == "parse-error" { assert_eq!(stderr, "eslint could not parse broken.js: Parsing error: Unexpected token =", "{name}"); }
                    if name == "param" {
                        assert_eq!(*code, None, "{name}");
                        assert_eq!(stderr, "eslint param option: an eslint rule takes no params", "{name}");
                    }
                    return;
                }
                _ => {}
            }
            let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                panic!("{name}: expected findings, got {:?}", runs[0].outcome);
            };
            assert_eq!(findings.len(), count, "{name}: {findings:?}");
            if name == "no-files" { assert!(runs[0].no_files.is_some(), "{name}: {:?}", runs[0]); }
            if name == "clean" || name == "git-ignored" { assert_eq!(runs[0].no_files, None, "{name}"); }
            if count == 1 {
                let finding = &findings[0];
                assert_eq!(finding.rule, "eslint-test", "{name}");
                assert_eq!(finding.path, RelPath::new(path), "{name}");
                assert_eq!(finding.line(), Some(if name == "inline-disable" { 2 } else { 1 }), "{name}");
                assert_eq!(finding.fix, "Use the recommended form.", "{name}");
                if name == "typescript" {
                    assert_eq!(finding.col, Some(17), "{name}");
                    assert_eq!(finding.what, "Unexpected any. Specify a different type.", "{name}");
                } else {
                    assert_eq!(finding.col, Some(1), "{name}");
                    assert_eq!(finding.what, "Unexpected var, use let or const instead.", "{name}");
                }
            }
        });
        if result.is_err() { failed.push(name); } else { eprintln!("{name}: PASS"); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

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
        let path = std::env::temp_dir().join(format!("engines-zit-{}-stylelint-{name}", std::process::id()));
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
fn stylelint_rule_reports_findings_suppressions_errors_and_absence() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let violation = "a { color: #fff1az; }\n";
    let fixable = "a { color: #FFF; }";
    let standard = r#"{"extends":"/opt/homebrew/lib/node_modules/stylelint-config-standard/index.js","rules":{"color-no-invalid-hex":true}}"#;
    let cases = [
        ("violation", "a.css", violation, standard, "color-no-invalid-hex", 1),
        ("clean", "a.css", "a { color: #fff; }\n", standard, "color-no-invalid-hex", 0),
        ("inline-disable", "a.css", "/* stylelint-disable color-no-invalid-hex */\na { color: #fff1az; }\n", standard, "color-no-invalid-hex", 1),
        ("stylelintignore", "a.css", violation, standard, "color-no-invalid-hex", 1),
        ("suppressions-file", "a.css", violation, standard, "color-no-invalid-hex", 1),
        ("ignore-files", "a.css", violation, r#"{"ignoreFiles":["a.css"],"rules":{"color-no-invalid-hex":true}}"#, "color-no-invalid-hex", 0),
        ("not-on", "a.css", violation, r#"{"rules":{"color-no-invalid-hex":true}}"#, "color-named", 0),
        ("off-for-one", "a.css", violation, r#"{"rules":{"color-no-invalid-hex":true},"overrides":[{"files":["b.css"],"rules":{"color-no-invalid-hex":null}}]}"#, "color-no-invalid-hex", 0),
        ("nested-fix", "a.css", fixable, r#"{"rules":{"color-hex-length":"long"}}"#, "color-hex-length", 0),
        ("quiet", "a.css", violation, r#"{"quiet":true,"rules":{"color-no-invalid-hex":[true,{"severity":"warning"}]}}"#, "color-no-invalid-hex", 0),
        ("unknown", "a.css", violation, r#"{"rules":{"color-no-invalid-hexx":true}}"#, "color-no-invalid-hexx", 0),
        ("fix-config", "a.css", fixable, r#"{"fix":true,"rules":{"color-hex-length":"long"}}"#, "color-hex-length", 0),
        ("config-fails", "a.css", violation, r#"{"extends":"nope-config"}"#, "color-no-invalid-hex", 0),
        ("parse-error", "broken.css", "a { color: red;", standard, "color-no-invalid-hex", 0),
        ("no-files", "notes.txt", "no CSS here\n", standard, "color-no-invalid-hex", 0),
        ("absent-no-files", "notes.txt", "no CSS here\n", standard, "color-no-invalid-hex", 0),
        ("absent", "a.css", violation, standard, "color-no-invalid-hex", 0),
        ("param", "a.css", violation, standard, "color-no-invalid-hex", 0),
    ];
    let mut failed = Vec::new();
    for (name, path, source, config, check, count) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            fs::write(folder.0.join(path), source).unwrap();
            fs::write(folder.0.join(".stylelintrc.json"), config).unwrap();
            if name == "stylelintignore" { fs::write(folder.0.join(".stylelintignore"), "a.css\n").unwrap(); }
            if name == "off-for-one" { fs::write(folder.0.join("b.css"), violation).unwrap(); }
            if name == "nested-fix" {
                fs::create_dir(folder.0.join("sub")).unwrap();
                fs::write(folder.0.join("sub/c.css"), fixable).unwrap();
                fs::write(folder.0.join("sub/.stylelintrc.json"), r#"{"fix":true,"rules":{"color-hex-length":"long"}}"#).unwrap();
            }
            if name == "suppressions-file" {
                let output = procio::run(&procio::Cmd {
                    args: ["--suppress", "a.css"].into_iter().map(Into::into).collect(),
                    cwd: Some(folder.0.clone()),
                    ..procio::Cmd::new("stylelint")
                }).unwrap();
                assert_eq!(output.code, Some(0), "{name}: {}", String::from_utf8_lossy(&output.stderr));
                assert!(folder.0.join("stylelint-suppressions.json").is_file(), "{name}");
            }
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"stylelint-test\"\nengine = \"stylelint\"\ncheck = \"{check}\"\nfix = \"Use a valid value.\"\n{}",
                if name == "param" { "params = { option = 1 }\n" } else { "" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "absent-no-files" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_stylelint = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("stylelint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_stylelint);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            if name == "nested-fix" || name == "fix-config" {
                assert_eq!(fs::read(folder.0.join("a.css")).unwrap(), fixable.as_bytes(), "{name}");
                if name == "nested-fix" { assert_eq!(fs::read(folder.0.join("sub/c.css")).unwrap(), fixable.as_bytes(), "{name}"); }
            }
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            match name {
                "absent" => {
                    assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                        rule: "stylelint-test".into(), engine: Engine::Stylelint, why: "stylelint not found on PATH".into(),
                    }, "{name}");
                    return;
                }
                "not-on" | "unknown" => {
                    let RuleOutcome::NotAvailable { why, .. } = &runs[0].outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", runs[0].outcome);
                    };
                    assert!(why.contains(check), "{name}: {why}");
                    if name == "unknown" { assert!(why.contains("Unknown rule color-no-invalid-hexx"), "{name}: {why}"); }
                    return;
                }
                "ignore-files" | "off-for-one" | "nested-fix" | "quiet" | "fix-config" | "config-fails" | "parse-error" | "param" => {
                    let RuleOutcome::Failed { code, stderr, .. } = &runs[0].outcome else {
                        panic!("{name}: expected Failed, got {:?}", runs[0].outcome);
                    };
                    match name {
                        "ignore-files" => for token in ["a.css", "exceptions go in the rule file"] { assert!(stderr.contains(token), "{name}: {stderr}"); },
                        "off-for-one" => for token in ["b.css", "color-no-invalid-hex", "exceptions go in the rule file"] { assert!(stderr.contains(token), "{name}: {stderr}"); },
                        "nested-fix" => for token in ["sub/c.css", "turns on fix"] { assert!(stderr.contains(token), "{name}: {stderr}"); },
                        "quiet" => assert!(stderr.contains("turns on quiet"),"{name}: {stderr}"),
                        "fix-config" => assert!(stderr.contains("turns on fix"), "{name}: {stderr}"),
                        "config-fails" => assert!(stderr.contains("Could not find \"nope-config\""), "{name}: {stderr}"),
                        "parse-error" => assert_eq!(stderr, "stylelint could not parse broken.css: Unclosed block (CssSyntaxError)", "{name}"),
                        "param" => {
                            assert_eq!(*code, None, "{name}");
                            assert_eq!(stderr, "stylelint param option: a stylelint rule takes no params", "{name}");
                        }
                        _ => unreachable!(),
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
            if name == "absent-no-files" { assert_eq!(runs[0].no_files.as_deref(), Some("no file its engine checks"), "{name}"); }
            if name == "clean" { assert_eq!(runs[0].no_files, None, "{name}"); }
            if count == 1 {
                let finding = &findings[0];
                assert_eq!(finding.rule, "stylelint-test", "{name}");
                assert_eq!(finding.path, RelPath::new(path), "{name}");
                assert_eq!(finding.line(), Some(if name == "inline-disable" { 2 } else { 1 }), "{name}");
                assert_eq!(finding.col, Some(12), "{name}");
                assert_eq!(finding.what, "Invalid hex color \"#fff1az\" (color-no-invalid-hex)", "{name}");
                assert_eq!(finding.fix, "Use a valid value.", "{name}");
            }
        });
        if result.is_err() { failed.push(name); } else { eprintln!("{name}: PASS"); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-gitleaks", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
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
fn gitleaks_rule_reports_secret_clean_unknown_and_absence() {
    let folder = TempFolder::new();
    let original_path = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original_path.0.as_deref().unwrap_or_default()))
    ).unwrap());
    let key = format!("AKIA{}", "Q7Z2M4X6W3B6N5KD");
    let secret_source = format!("aws_key = \"{key}\"\n");
    let allowed_source = format!("aws_key = \"{key}\" # gitleaks:allow\n");
    let token = format!("{}{}", "TMTEST", "735946208115");
    let token_regex = format!("{}[0-9]{{12}}", "TMTEST");
    let token_source = format!("token = \"{token}\"\n");
    let cases = [
        ("secret", secret_source.as_str(), "aws-access-token", false),
        ("clean", "VALUE = 1\n", "aws-access-token", false),
        ("unknown rule", secret_source.as_str(), "no-such-rule", false),
        ("allow comment", allowed_source.as_str(), "aws-access-token", false),
        ("ignore file", secret_source.as_str(), "aws-access-token", false),
        ("config allowlist", secret_source.as_str(), "aws-access-token", false),
        ("extended allowlist", secret_source.as_str(), "aws-access-token", false),
        ("config own rule", token_source.as_str(), "cctao-test-token", false),
        ("inline extended allowlist", secret_source.as_str(), "aws-access-token", false),
        ("mixed-case allowlist", secret_source.as_str(), "aws-access-token", false),
        ("config not TOML", secret_source.as_str(), "aws-access-token", false),
        ("dotted allowlist key", secret_source.as_str(), "aws-access-token", false),
        ("gitleaks absent", secret_source.as_str(), "aws-access-token", true),
    ];
    let mut failed = Vec::new();
    for (name, source, check, absent) in cases {
        let row = std::panic::catch_unwind(|| {
            for entry in fs::read_dir(&folder.0).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    fs::remove_dir_all(path).unwrap();
                } else {
                    fs::remove_file(path).unwrap();
                }
            }
            match name {
                "ignore file" => fs::write(folder.0.join(".gitleaksignore"), "config.py:aws-access-token:1\n").unwrap(),
                "config allowlist" => fs::write(folder.0.join(".gitleaks.toml"),
                    "[extend]\nuseDefault = true\n[[allowlists]]\npaths = [\"config.py\"]\n").unwrap(),
                "extended allowlist" => {
                    fs::create_dir(folder.0.join("cfg")).unwrap();
                    fs::write(folder.0.join(".gitleaks.toml"), "[extend]\npath = \"cfg/base.toml\"\n").unwrap();
                    fs::write(folder.0.join("cfg/base.toml"),
                        "[extend]\nuseDefault = true\n[allowlist]\npaths = [\"config.py\"]\n").unwrap();
                }
                "config own rule" => fs::write(folder.0.join(".gitleaks.toml"), format!(
                    "[extend]\nuseDefault = true\n[[rules]]\nid = \"cctao-test-token\"\nregex = '''{token_regex}'''\n"
                )).unwrap(),
                "inline extended allowlist" => {
                    fs::create_dir(folder.0.join("base")).unwrap();
                    fs::write(folder.0.join(".gitleaks.toml"), "extend = { path = \"base/x.toml\" }\n").unwrap();
                    fs::write(folder.0.join("base/x.toml"), format!(
                        "rules = [{{ id = \"aws-access-token\", regex = '''{}[A-Z0-9]{{16}}''', allowlist = {{ paths = [\"config.py\"] }} }}]\n[extend]\nuseDefault = true\n", "AKIA"
                    )).unwrap();
                }
                "mixed-case allowlist" => fs::write(folder.0.join(".gitleaks.toml"),
                    "[extend]\nuseDefault = true\n[AllowList]\npaths = [\"config.py\"]\n").unwrap(),
                "config not TOML" => fs::write(folder.0.join(".gitleaks.toml"), "[allowlist\n").unwrap(),
                "dotted allowlist key" => fs::write(folder.0.join(".gitleaks.toml"),
                    "\"allowlist.paths\" = [\"config.py\"]\n[extend]\nuseDefault = true\n").unwrap(),
                _ => {}
            }
            fs::write(folder.0.join("config.py"), source).unwrap();
            fs::write(folder.0.join("tao.toml"), format!(
                "version = 1\nowner = \"security-team\"\n[[rule]]\nid = \"aws-secret\"\nengine = \"gitleaks\"\nkind = \"security\"\ncheck = \"{check}\"\nfix = \"Remove the exposed key.\"\n"
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("tao.toml")).unwrap();
            let _path_guard = if absent {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_gitleaks = std::env::join_paths(
                    std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                        !dir.join("gitleaks").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                    })
                ).unwrap();
                std::env::set_var("PATH", without_gitleaks);
                Some(guard)
            } else {
                None
            };
            let result = run_rules_versioned_at(&file, &folder.0, &[], None);
            let debug = format!("{result:?}");
            assert!(!debug.contains(&key), "{name}: check result exposed the key");
            assert!(!debug.contains(&token), "{name}: check result exposed the token");
            assert_eq!(result.runs.len(), 1, "{name}: expected one run");
            let outcome = &result.runs[0].outcome;
            match name {
                "secret" | "clean" => {
                    let RuleOutcome::Findings(findings) = outcome else {
                        panic!("{name}: expected Findings, got {}", format!("{outcome:?}").replace(&key, "[redacted]"));
                    };
                    let count = if name == "secret" { 1 } else { 0 };
                    assert_eq!(findings.len(), count, "{name}: finding count");
                    if name == "secret" {
                        let finding = &findings[0];
                        assert!(!format!("{finding:?}").contains(&key), "{name}: finding exposed the key");
                        assert!(finding.rule == "aws-secret", "{name}: rule id");
                        assert!(finding.path == RelPath::new("config.py"), "{name}: path");
                        assert_eq!(finding.line(), Some(1), "{name}: line");
                        assert_eq!(finding.col, Some(12), "{name}: column");
                        assert!(finding.fix == "Remove the exposed key.", "{name}: fix");
                    }
                }
                "allow comment" | "config own rule" => {
                    let RuleOutcome::Findings(findings) = outcome else {
                        panic!("{name}: expected Findings, got {}", format!("{outcome:?}").replace(&key, "[redacted]").replace(&token, "[redacted]"));
                    };
                    assert_eq!(findings.len(), 1, "{name}: finding count");
                    assert_eq!(findings[0].path, RelPath::new("config.py"), "{name}: path");
                    assert_eq!(findings[0].line(), Some(1), "{name}: line");
                }
                "ignore file" | "config allowlist" | "extended allowlist" | "inline extended allowlist"
                | "mixed-case allowlist" | "config not TOML" | "dotted allowlist key" => {
                    let RuleOutcome::Failed { stderr, .. } = outcome else {
                        panic!("{name}: expected Failed, got {}", format!("{outcome:?}").replace(&key, "[redacted]").replace(&token, "[redacted]"));
                    };
                    let why = match name {
                        "ignore file" => "`.gitleaksignore` in the rule's folder lists findings to ignore; the rule file is the only home for an exception, so cctao reads no findings under it",
                        "config allowlist" => "gitleaks config `.gitleaks.toml` declares an allowlist (`allowlists`); the rule file is the only home for an exception, so cctao reads no findings under it",
                        "extended allowlist" => "gitleaks config `cfg/base.toml` declares an allowlist (`allowlist`); the rule file is the only home for an exception, so cctao reads no findings under it",
                        "inline extended allowlist" => "gitleaks config `base/x.toml` declares an allowlist (`rules[0].allowlist`); the rule file is the only home for an exception, so cctao reads no findings under it",
                        "mixed-case allowlist" => "gitleaks config `.gitleaks.toml` declares an allowlist (`AllowList`); the rule file is the only home for an exception, so cctao reads no findings under it",
                        "config not TOML" => "gitleaks config `.gitleaks.toml` does not parse as TOML, so cctao cannot tell whether it declares an allowlist and reads no findings under it",
                        "dotted allowlist key" => "gitleaks config `.gitleaks.toml` declares an allowlist (`allowlist`); the rule file is the only home for an exception, so cctao reads no findings under it",
                        _ => unreachable!(),
                    };
                    assert_eq!(stderr, why, "{name}: failure reason");
                }
                "unknown rule" | "gitleaks absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = outcome else {
                        panic!("{name}: expected NotAvailable, got {}", format!("{outcome:?}").replace(&key, "[redacted]"));
                    };
                    assert!(rule == "aws-secret", "{name}: rule id");
                    assert_eq!(*engine, Engine::Gitleaks, "{name}: engine");
                    if absent {
                        assert!(why == "gitleaks not found on PATH", "{name}: {}", why.replace(&key, "[redacted]"));
                    } else {
                        assert!(why.contains("Requested rule no-such-rule not found in rules"), "{name}: {}", why.replace(&key, "[redacted]"));
                    }
                }
                _ => unreachable!(),
            }
        });
        if row.is_err() {
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

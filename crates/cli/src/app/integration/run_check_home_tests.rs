//! The check's config, append-only log and telemetry file under a test-owned HOME.

use super::super::run_check_savings_tests::{assert_clean_body, check, checked, output_tokens, percent, printed_body, run, world};
use super::super::CheckWorld;
use crate::CheckArgs;
use ccvcslib::VcsError;
use report::Oid;
use rules::{Engine, Kind, RelPath, RuleFile};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);
struct Home(PathBuf);
impl Home {
    fn new() -> Self {
        loop {
            let path = std::env::temp_dir().join(format!("cctao-savings-home-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("create HOME: {e}"),
            }
        }
    }
    fn config(&self, text: &str) {
        let dir = self.0.join(".config/cctao");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("config.toml"), text).unwrap();
    }
    fn log(&self) -> PathBuf { self.0.join(".local/state/cctao/savings.jsonl") }
}
impl Drop for Home {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

/// A first-parent line of three commits: HEAD, HEAD~1 and HEAD~2 name one; past it, none.
fn three_commits(_: &Path, rev: &str) -> Result<Option<Oid>, VcsError> {
    let hex = match rev {
        "HEAD" => '1',
        "HEAD~1" => '2',
        "HEAD~2" => '3',
        _ => return Ok(None),
    };
    Ok(Oid::parse(&hex.to_string().repeat(40)))
}

#[test]
fn disabled_line_leaves_text_json_and_logging_available() {
    let home = Home::new();
    home.config("savings_line = false\n");
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() };
    let (code, text, err) = check(&CheckArgs::default(), &w);
    assert_eq!((code, err.as_str()), (0, ""));
    assert_clean_body(&text);
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (0, ""));
    let (body, saved) = printed_body(&out);
    assert_eq!(saved["engine_runs"], 1);
    assert_eq!(saved["baseline_tokens"], 100);
    let chars = body.chars().count();
    assert_eq!(saved["output_chars"], chars);
    let tokens = output_tokens(chars);
    let tokens_saved = 100usize.saturating_sub(tokens);
    assert_eq!(output_tokens(231), 58);
    assert_eq!(saved["output_tokens"], tokens);
    assert_eq!(saved["tokens_saved"], tokens_saved);
    assert_eq!(saved["percent"], tokens_saved);
    assert_eq!(fs::read_to_string(home.log()).unwrap(), format!(concat!(
        "{{\"time\":\"2026-11-26T08:00:00Z\",\"folder\":\"/repo\",\"verb\":\"check\",\"output_tokens\":{},\"baseline_tokens\":100}}\n",
        "{{\"time\":\"2026-11-26T08:00:00Z\",\"folder\":\"/repo\",\"verb\":\"check\",\"output_tokens\":{},\"baseline_tokens\":100}}\n"
    ), output_tokens(text.chars().count()), tokens));
}

#[test]
fn disabled_switch_leaves_the_entire_saved_object_unchanged() {
    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() };
    let args = CheckArgs { json: true, ..CheckArgs::default() };
    let (on_code, on_out, on_err) = check(&args, &w);
    home.config("savings_line = false\n");
    let (off_code, off_out, off_err) = check(&args, &w);
    assert_eq!((on_code, on_err.as_str(), off_code, off_err.as_str()), (0, "", 0, ""));
    let on: serde_json::Value = serde_json::from_str(&on_out).unwrap();
    let off: serde_json::Value = serde_json::from_str(&off_out).unwrap();
    assert!(on["saved"].is_object());
    assert_eq!(off["saved"], on["saved"]);
}

#[test]
fn unknown_config_key_warns_once_and_keeps_the_default_line() {
    let home = Home::new();
    home.config("extra = true\n");
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let (code, out, err) = check(&CheckArgs::default(), &CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() });
    assert_eq!(code, 0);
    assert_eq!(err, "cctao: ignoring ~/.config/cctao/config.toml: unknown key `extra`\n");
    let (line, body) = out.split_once('\n').unwrap();
    assert_clean_body(body);
    let saved = 100 - output_tokens(body.chars().count());
    assert!(line.starts_with(&format!("[cctao] tokens saved ≈ {saved} ({}%)", percent(saved, 100))), "{out}");
}

#[test]
fn json_unknown_config_key_warns_once_without_changing_the_report_or_exit() {
    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() };
    let args = CheckArgs { json: true, ..CheckArgs::default() };
    let (baseline_code, baseline_out, baseline_err) = check(&args, &w);
    assert_eq!(baseline_err, "");
    home.config("colour = true\n");
    let (code, out, err) = check(&args, &w);
    assert_eq!(code, baseline_code);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(value.is_object());
    assert!(value["saved"].is_object());
    assert_eq!(value, serde_json::from_str::<serde_json::Value>(&baseline_out).unwrap());
    assert_eq!(err, "cctao: ignoring ~/.config/cctao/config.toml: unknown key `colour`\n");
}

#[test]
fn json_invalid_toml_warns_once_when_an_engine_ran() {
    let home = Home::new();
    home.config("not = [toml\n");
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() };
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!(code, 0);
    assert!(serde_json::from_str::<serde_json::Value>(&out).unwrap()["saved"].is_object());
    let reason = err.strip_prefix("cctao: ignoring ~/.config/cctao/config.toml: ").unwrap();
    assert!(!reason.trim().is_empty(), "{err:?}");
    assert_eq!(err.lines().count(), 1, "{err:?}");
    assert!(err.ends_with('\n'));
}

#[test]
fn json_bad_config_is_not_read_when_no_engine_ran() {
    let home = Home::new();
    home.config("colour = true\n");
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, None)]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() };
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!(code, 0);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&out).unwrap()["saved"]["engine_runs"], 0);
    assert_eq!(err, "");
}

#[test]
fn failed_report_after_counted_runs_has_no_savings_in_either_format() {
    let home = Home::new();
    let cwd = Home::new();
    let home_path = || Some(home.0.clone());
    let calls = std::cell::Cell::new(0);
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
        calls.set(calls.get() + 1);
        checked(vec![run("q", Engine::Ruff, Some(400))])
    };
    let fail = |_: &RuleFile, _: Vec<(String, rules::RuleOutcome)>, _: Option<(&Path, &report::Oid)>, _: Vec<(String, String)>| {
        Err(report::ReportError::Vcs(ccvcslib::VcsError::NotAWorkingCopy))
    };
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, run_rules: &runs, build_report: &fail, ..world() };
    for json in [false, true] {
        let mut args = CheckArgs::default();
        args.json = json;
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let code = super::super::run_check(&args, &cwd.0, &w, &mut out, &mut err);
        let out = String::from_utf8(out).unwrap();
        assert_eq!(code, 2);
        assert!(!out.contains("tokens saved"), "{out}");
        if json {
            let value: serde_json::Value = serde_json::from_str(&out).unwrap();
            assert!(value.get("saved").is_none(), "{out}");
        }
        assert!(!home.log().exists());
    }
    assert_eq!(calls.get(), 2);
}

#[test]
fn savings_log_names_rule_file_folder_not_cwd() {
    let home = Home::new();
    let cwd = Home::new();
    let rules_dir = Home::new();
    let rules_path = rules_dir.0.join("tao.toml");
    fs::write(&rules_path, "version = 1\n[[rule]]\nid = \"q\"\nengine = \"ruff\"\ncheck = \"F401\"\nfix = \"Remove import.\"\n").unwrap();
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, Some(400))]);
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, read: &RuleFile::read, run_rules: &runs, ..world() };
    let mut args = CheckArgs::default();
    args.rules = Some(rules_path);
    let (mut out, mut err) = (Vec::new(), Vec::new());
    assert_eq!(super::super::run_check(&args, &cwd.0, &w, &mut out, &mut err), 0);
    let log = fs::read_to_string(home.log()).unwrap();
    let value: serde_json::Value = serde_json::from_str(&log).unwrap();
    assert!(rules_dir.0.is_absolute());
    assert_ne!(rules_dir.0, cwd.0);
    assert_eq!(value["folder"].as_str().unwrap(), rules_dir.0.to_str().unwrap());
    assert_eq!(log.lines().count(), 1);
}

#[test]
fn no_log_is_created_when_no_engine_ran() {
    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| checked(vec![run("q", Engine::Ruff, None)]);
    let (code, out, _) = check(&CheckArgs::default(), &CheckWorld { home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, run_rules: &runs, ..world() });
    assert_eq!(code, 0);
    assert_clean_body(&out);
    assert!(!home.log().exists());
}

#[test]
fn usage_error_creates_no_log() {
    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let w = CheckWorld { home: &home_path, now: &|| 1795680000, toplevel: &|_| Err(ccvcslib::VcsError::NotAWorkingCopy), ..world() };
    let (code, out, _) = check(&CheckArgs::default(), &w);
    assert_eq!((code, out.as_str()), (2, ""));
    assert!(!home.log().exists());
}

#[test]
fn watch_finding_reaches_telemetry_but_not_output_or_savings() {
    const TEXT: &str = "version = 1\n[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F401\"\nseverity = \"watch\"\nfix = \"Remove import.\"\n";
    let read = |_: &Path| Ok(RuleFile::parse(TEXT, Path::new("/repo")).unwrap());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
        checked(vec![engines::RuleRun::new(
            "w".into(), Engine::Ruff,
            rules::RuleOutcome::Findings(vec![rules::Finding {
                rule: "w".into(), engine: Engine::Ruff, kind: Kind::Quality, severity: rules::Severity::Watch,
                owner: None, path: RelPath::new("src/watch.py"), line: Some(7), col: None,
                what: "Unused watch import".into(), fix: "Remove import.".into(),
            }]),
            Some(400),
        )])
    };
    let w = CheckWorld { read: &read, run_rules: &runs, build_report: &report::build_report_with_no_files, ..world() };
    let (code, text, err) = check(&CheckArgs::default(), &w);
    assert_eq!((code, err.as_str()), (0, ""));
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (0, ""));
    let (body, saved) = printed_body(&out);
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(json["findings"], serde_json::json!([]));
    assert!(!out.contains("Unused watch import"));
    assert!(!out.contains("src/watch.py"));

    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let (code, home_text, err) = check(&CheckArgs::default(), &CheckWorld {
        home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, ..w
    });
    assert_eq!((code, err.as_str()), (0, ""));
    let telemetry = fs::read_to_string(home.0.join(
        ".local/state/cctao/telemetry/3333333333333333333333333333333333333333.jsonl"
    )).unwrap();
    assert_eq!(telemetry.lines().count(), 1);
    let record: serde_json::Value = serde_json::from_str(&telemetry).unwrap();
    let rules = record["rules"].as_array().unwrap();
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0]["rule"], "w");
    assert_eq!(rules[0]["findings"].as_array().unwrap().len(), 1);
    assert_eq!(rules[0]["findings"][0]["what"], "Unused watch import");
    assert_eq!(rules[0]["findings"][0]["path"], "src/watch.py");

    assert_eq!(saved["engine_runs"], 0);
    assert_eq!(saved["baseline_tokens"], 0);
    assert_clean_body(&text);
    assert_clean_body(&home_text);
    assert!(!home.log().exists());
}

#[test]
fn shared_watch_runs_are_excluded_from_savings() {
    let rows = [
        (
            "watch first, error second",
            "version = 1\n[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F841\"\nseverity = \"watch\"\nfix = \"Remove variable.\"\n\
             [[rule]]\nid = \"e\"\nengine = \"ruff\"\ncheck = \"F401\"\nfix = \"Remove import.\"\n",
            "e", Some(400), 1, 100,
            serde_json::json!([{"rule":"e", "engine":"ruff", "chars":400}]),
        ),
        (
            "watch rules only",
            "version = 1\n[[rule]]\nid = \"w\"\nengine = \"ruff\"\ncheck = \"F841\"\nseverity = \"watch\"\nfix = \"Remove variable.\"\n\
             [[rule]]\nid = \"v\"\nengine = \"ruff\"\ncheck = \"F401\"\nseverity = \"watch\"\nfix = \"Remove import.\"\n",
            "v", None, 0, 0, serde_json::json!([]),
        ),
    ];
    for (name, text, second, chars, engine_runs, baseline_tokens, counted) in rows {
        let home = Home::new();
        let home_path = || Some(home.0.clone());
        let read = |_: &Path| Ok(RuleFile::parse(text, Path::new("/repo")).unwrap());
        let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
            checked(vec![run("w", Engine::Ruff, None), run(second, Engine::Ruff, chars)])
        };
        let w = CheckWorld {
            home: &home_path, now: &|| 1795680000, rev_parse: &three_commits, read: &read,
            run_rules: &runs, build_report: &report::build_report_with_no_files, ..world()
        };
        let (code, out, err) = check(&CheckArgs::default(), &w);
        assert_eq!((code, err.as_str()), (0, ""), "{name}");
        assert_eq!(out.starts_with("[cctao] tokens saved"), engine_runs == 1, "{name}: {out}");
        if engine_runs == 1 {
            assert_eq!(fs::read_to_string(home.log()).unwrap().lines().count(), 1, "{name}");
        } else {
            assert!(!home.log().exists(), "{name}");
        }
        let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
        assert_eq!((code, err.as_str()), (0, ""), "{name}");
        let (_, saved) = printed_body(&out);
        assert_eq!(saved["engine_runs"], engine_runs, "{name}");
        assert_eq!(saved["baseline_tokens"], baseline_tokens, "{name}");
        assert_eq!(saved["counted"], counted, "{name}");
        if engine_runs == 1 {
            assert_eq!(fs::read_to_string(home.log()).unwrap().lines().count(), 2, "{name}");
        } else {
            assert!(!home.log().exists(), "{name}");
        }
    }
}

#[test]
fn a_check_appends_one_telemetry_line_named_after_the_root_and_prints_the_same() {
    let home = Home::new();
    let home_path = || Some(home.0.clone());
    let git = |top: &Path, rev: &str| {
        assert_eq!(top, Path::new("/repo"));
        let id = match rev {
            "HEAD" => "a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1",
            "HEAD~1" => "b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2b2",
            "HEAD~2" => "c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3",
            "HEAD~3" => "d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4",
            "main" => "e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5",
            _ => return Ok(None),
        };
        Ok(Oid::parse(id))
    };
    let args = CheckArgs {
        base: Some("main".to_string()),
        kind: Some(Kind::Security),
        paths: vec![PathBuf::from("src/a.py")],
        ..CheckArgs::default()
    };
    let w = CheckWorld { now: &|| 1795680000, rev_parse: &git, ..world() };
    let without_home = check(&args, &w);
    let with_home = check(&args, &CheckWorld { home: &home_path, ..w });
    assert_eq!(with_home, without_home);
    let dir = home.0.join(".local/state/cctao/telemetry");
    let names: Vec<_> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().file_name()).collect();
    assert_eq!(names, ["d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4d4.jsonl"]);
    let version = crate::output::version_after_name(env!("CARGO_PKG_VERSION"), env!("CCTAO_COMMIT"));
    assert_eq!(fs::read_to_string(dir.join(&names[0])).unwrap(), format!(concat!(
        "{{\"format\":1,\"time\":\"2026-11-26T08:00:00Z\",\"version\":\"{}\",\"folder\":\"/repo\",",
        "\"head\":\"a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1\",\"base\":\"e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5\",",
        "\"kind\":\"security\",\"paths\":[\"src/a.py\"],\"exit\":0,\"engines\":[],\"rules\":[]}}\n"
    ), version));
}

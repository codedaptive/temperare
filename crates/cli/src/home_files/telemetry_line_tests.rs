//! Unit test for telemetry_line: the whole line from given run fields and a given report, keys in the contract's order.

use super::{TelemetryRun, telemetry_line};
use report::{EngineVersion, RecordFinding, Report, RuleRecord, RuleState};
use rules::{Engine, Kind, RelPath};
use std::path::Path;

#[test]
fn telemetry_line_writes_the_run_and_the_report_in_order() {
    let two_paths = [RelPath::new("src/a.py"), RelPath::new("src/b.py")];
    let one_rule = Report {
        engines: vec![EngineVersion { engine: Engine::Ruff, version: Some("0.6.0".to_owned()) }],
        record: vec![RuleRecord {
            rule: "no-eval".to_owned(),
            engine: Engine::Ruff,
            severity: "error".to_owned(),
            state: RuleState::Ran,
            why: None,
            findings: vec![RecordFinding { path: RelPath::new("src/a.py"), line: Some(3), what: "eval used".to_owned() }],
            silenced: 0,
        }],
        ..Report::default()
    };
    let whole_tree = TelemetryRun { folder: Path::new("/work/repo"), base: None, kind: None, paths: &[], exit: 0 };
    let rows: [(&str, TelemetryRun, &Report, &str); 3] = [
        (
            "whole tree, no base or kind, empty report",
            whole_tree,
            &Report::default(),
            r#"{"format":1,"time":"1970-01-01T00:00:00Z","version":"0.5.0","folder":"/work/repo","head":"def456","base":null,"kind":null,"paths":[],"exit":0,"engines":[],"rules":[]}"#,
        ),
        (
            "base, kind security, two paths, one engine and one rule",
            TelemetryRun {
                folder: Path::new("/work/repo"),
                base: Some("abc123"),
                kind: Some(Kind::Security),
                paths: &two_paths,
                exit: 1,
            },
            &one_rule,
            r#"{"format":1,"time":"1970-01-01T00:00:00Z","version":"0.5.0","folder":"/work/repo","head":"def456","base":"abc123","kind":"security","paths":["src/a.py","src/b.py"],"exit":1,"engines":[{"engine":"ruff","version":"0.6.0"}],"rules":[{"rule":"no-eval","engine":"ruff","severity":"error","state":"ran","why":null,"findings":[{"path":"src/a.py","line":3,"what":"eval used"}],"silenced":0}]}"#,
        ),
        (
            "a quote in the folder is escaped",
            TelemetryRun { folder: Path::new("/work/a\"b"), base: None, kind: None, paths: &[], exit: 0 },
            &Report::default(),
            r#"{"format":1,"time":"1970-01-01T00:00:00Z","version":"0.5.0","folder":"/work/a\"b","head":"def456","base":null,"kind":null,"paths":[],"exit":0,"engines":[],"rules":[]}"#,
        ),
    ];
    for (case, run, report, expected) in rows {
        assert_eq!(telemetry_line(&run, 0, "0.5.0", "def456", report), expected, "{case}");
    }
}

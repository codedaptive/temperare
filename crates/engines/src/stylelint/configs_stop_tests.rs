//! ZUT for configs_stop.rs: a fake probe answering each file with real stylelint 18.1.0 `--print-config` output
//! (fixtures/print_config_*; see fixtures/fixtures.txt), except `OFF`, written to the shape
//! fixtures/print_config_rules.json shows for a null rule (`color-named`): the check on for all, for none, off for one,
//! a config turning on fix, and a probe that could not run; which files were probed, in order.

use super::configs_stop;
use crate::types::Stop;
use procio::Output;
use std::cell::RefCell;

const RULES: &[u8] = include_bytes!("fixtures/print_config_rules.json");
const FIX: &[u8] = include_bytes!("fixtures/print_config_fix.json");
const OFF: &[u8] = b"{\n  \"rules\": {\n    \"color-no-invalid-hex\": null\n  }\n}\n";

type Answer = Result<&'static [u8], Stop>;

#[test]
fn configs_stop_settles_on_every_files_config() {
    let not_on_path = || Stop::NotOnPath { program: "stylelint".into() };
    let rows: [(&str, &str, &[(&str, Answer)], Option<Stop>, &[&str]); 6] = [
        ("on for all", "color-no-invalid-hex", &[("a.css", Ok(RULES)), ("sub/b.css", Ok(RULES))], None, &["a.css", "sub/b.css"]),
        ("on for none", "color-named", &[("a.css", Ok(RULES)), ("sub/b.css", Ok(RULES))],
            Some(Stop::NotAvailable { why: "stylelint's config does not turn on color-named for any file the rule covers".into() }), &["a.css", "sub/b.css"]),
        ("off for one", "color-no-invalid-hex", &[("a.css", Ok(RULES)), ("b.css", Ok(OFF)), ("c.css", Ok(OFF))],
            Some(Stop::Failed { code: None, stderr: "stylelint's config turns color-no-invalid-hex off for b.css; exceptions go in the rule file".into() }),
            &["a.css", "b.css", "c.css"]),
        ("off first, on later", "color-no-invalid-hex", &[("a.css", Ok(OFF)), ("b.css", Ok(RULES))],
            Some(Stop::Failed { code: None, stderr: "stylelint's config turns color-no-invalid-hex off for a.css; exceptions go in the rule file".into() }),
            &["a.css", "b.css"]),
        ("fix stops the probing", "color-hex-length", &[("a.css", Ok(RULES)), ("sub/c.css", Ok(FIX)), ("d.css", Ok(RULES))],
            Some(Stop::Failed { code: None, stderr: "stylelint's config for sub/c.css turns on fix, which rewrites the checked files; tm never edits the code it checks".into() }),
            &["a.css", "sub/c.css"]),
        ("probe could not run", "color-no-invalid-hex", &[("a.css", Err(not_on_path())), ("b.css", Ok(RULES))], Some(not_on_path()), &["a.css"]),
    ];
    let mut failed = Vec::new();
    for (name, check, answers, want, want_probed) in rows {
        let files: Vec<String> = answers.iter().map(|(file, _)| file.to_string()).collect();
        let probed = RefCell::new(Vec::new());
        let probe = |file: &str| {
            probed.borrow_mut().push(file.to_string());
            let (_, answer) = answers.iter().find(|(listed, _)| *listed == file).expect("probe of a listed file");
            answer.clone().map(|stdout| Output { code: Some(0), stdout: stdout.to_vec(), stderr: Vec::new() })
        };
        let got = configs_stop(&files, check, &probe);
        let probed = probed.into_inner();
        if got != want || probed != want_probed {
            failed.push((name, got, probed));
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

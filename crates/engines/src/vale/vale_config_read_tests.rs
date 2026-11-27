//! Unit test for vale_config_read.rs: Vale 3.28.1's real `ls-config` output (fixtures/fixtures.txt says how each was
//! captured) read into a `ValeConfig`; no config is `NotAvailable` naming the config names; a load error and output
//! that is not the JSON are `Failed` with Vale's text. The stdout-only row is hand-built.

use super::ValeConfig;
use crate::types::Stop;
use procio::Output;
use std::path::PathBuf;

#[test]
fn vale_config_read_cases() {
    let config = |styles: &[&str], vocab: &[&str], paths: &[&str], files: &[&str]| {
        Ok(ValeConfig {
            styles: styles.iter().map(|style| style.to_string()).collect(),
            vocab: vocab.iter().map(|name| name.to_string()).collect(),
            paths: paths.iter().map(PathBuf::from).collect(),
            files: files.iter().map(PathBuf::from).collect(),
        })
    };
    let no_config = "Vale finds no config (.vale.ini, _vale.ini, vale.ini, .vale or _vale) at or above the rule file's \
        folder: cctao runs Vale only with the repository's own config";
    let load_error = "E201 Invalid value [/private/tmp/tm-vale-capture/bad/.vale.ini:1:14]:\n\n   1* StylesPath = styles\n   2  BasedOnStyles = Vale\n\nThe StylesPath '/private/tmp/tm-vale-capture/bad/styles' doesn't exist.\n\nExecution stopped with code 2.";
    let rows: [(&str, i32, &[u8], &[u8], Result<ValeConfig, Stop>); 5] = [
        (
            "global and section styles, Vocab",
            0,
            include_bytes!("fixtures/ls_config.json"),
            b"",
            config(&["Vale", "Other"], &["Proj"], &["/private/tmp/tm-vale-capture/styles"], &["/private/tmp/tm-vale-capture/.vale.ini"]),
        ),
        (
            "section styles only, no Vocab",
            0,
            include_bytes!("fixtures/ls_config_sections.json"),
            b"",
            config(&["Vale", "Gone"], &[], &["/private/tmp/tm-vale-capture/gone/styles"], &["/private/tmp/tm-vale-capture/gone/.vale.ini"]),
        ),
        ("no config", 2, b"", include_bytes!("fixtures/no_config.txt"), Err(Stop::NotAvailable { why: no_config.to_string() })),
        ("load error", 2, b"", include_bytes!("fixtures/load_error.txt"), Err(Stop::Failed { code: Some(2), stderr: load_error.to_string() })),
        ("not JSON, stderr empty", 0, b"  not json\n", b"", Err(Stop::Failed { code: Some(0), stderr: "not json".to_string() })),
    ];
    for (label, code, stdout, stderr, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(ValeConfig::read(&out), expected, "{label}");
    }
}

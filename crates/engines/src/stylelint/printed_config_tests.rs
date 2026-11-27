//! Unit test for printed_config.rs: real stylelint 18.1.0 `--print-config` output (fixtures/print_config_*; commands and
//! configs in fixtures/fixtures.txt) read for a check: a rule on, null, absent or unknown, a config turning on fix or
//! quiet, and a probe that failed. The "no config" row's stdout (`[]`, stdout that is no config) is written by hand.

use super::printed_config;
use crate::types::Stop;
use procio::Output;

#[test]
fn printed_config_reads_the_check_fix_quiet_and_failures() {
    let rules: &[u8] = include_bytes!("fixtures/print_config_rules.json");
    let error: &[u8] = include_bytes!("fixtures/print_config_error.txt");
    let failed = |code, stderr: &str| Err(Stop::Failed { code, stderr: stderr.to_string() });
    let rows: [(&str, i32, &[u8], &[u8], &str, Result<bool, Stop>); 9] = [
        ("rule on", 0, rules, b"", "color-no-invalid-hex", Ok(true)),
        ("rule with an option", 0, rules, b"", "color-hex-length", Ok(true)),
        ("unknown rule printed []", 0, rules, b"", "color-no-invalid-hexx", Ok(true)),
        ("rule null", 0, rules, b"", "color-named", Ok(false)),
        ("rule absent", 0, rules, b"", "block-no-empty", Ok(false)),
        ("fix", 0, include_bytes!("fixtures/print_config_fix.json"), b"", "color-hex-length", failed(None,
            "stylelint's config for sub/a.css turns on fix, which rewrites the checked files; cctao never edits the code it checks")),
        ("quiet", 0, include_bytes!("fixtures/print_config_quiet.json"), b"", "color-no-invalid-hex", failed(None,
            "stylelint's config for sub/a.css turns on quiet, which drops warning-severity findings; exceptions go in the rule file")),
        ("config error", 78, b"", error, "color-no-invalid-hex", failed(Some(78), "ConfigurationError: Could not find \"nope-config\". Do you need to install the package, use the \"configBasedir\" option or add the filename and extension to the path?\n    at getModulePath (file:///opt/homebrew/lib/node_modules/stylelint/lib/utils/getModulePath.mjs:31:9)\n    at loadExtendedConfig (file:///opt/homebrew/lib/node_modules/stylelint/lib/augmentConfig.mjs:281:21)\n    at extendConfig (file:///opt/homebrew/lib/node_modules/stylelint/lib/augmentConfig.mjs:248:25)\n    at augmentConfigBasic (file:///opt/homebrew/lib/node_modules/stylelint/lib/augmentConfig.mjs:79:26)\n    at augmentConfigFull (file:///opt/homebrew/lib/node_modules/stylelint/lib/augmentConfig.mjs:132:30)\n    at getConfigForFile (file:///opt/homebrew/lib/node_modules/stylelint/lib/getConfigForFile.mjs:102:32)\n    at async resolveConfig (file:///opt/homebrew/lib/node_modules/stylelint/lib/resolveConfig.mjs:32:19)\n    at async printConfig (file:///opt/homebrew/lib/node_modules/stylelint/lib/printConfig.mjs:35:4)\n    at async main (file:///opt/homebrew/lib/node_modules/stylelint/lib/cli.mjs:667:3)\n")),
        ("no config", 0, b"[]", b"", "color-no-invalid-hex", failed(Some(0), "stylelint --print-config printed no config for sub/a.css: []")),
    ];
    let failed: Vec<(&str, Result<bool, Stop>)> = rows
        .iter()
        .filter_map(|(name, code, stdout, stderr, check, want)| {
            let out = Output { code: Some(*code), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            let got = printed_config(&out, check, "sub/a.css");
            (got != *want).then_some((*name, got))
        })
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:#?}");
}

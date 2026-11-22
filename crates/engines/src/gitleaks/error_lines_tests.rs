//! ZUT for error_lines.rs: gitleaks 8.30.1's real stderr for an unknown rule id (fixtures/unknown_rule.txt) and
//! literal log lines cut to their error lines.

use super::error_lines;

#[test]
fn error_lines_keeps_only_err_and_ftl_text() {
    let key = format!("AKIA{}", "Q7Z2M4X6W3B6N5KD");
    let real: &[u8] = include_bytes!("fixtures/unknown_rule.txt");
    let coloured = "\u{1b}[90m10:04PM\u{1b}[0m \u{1b}[31mINF\u{1b}[0m Overriding enabled rules: no-such-rule\n\u{1b}[90m10:04PM\u{1b}[0m \u{1b}[1m\u{1b}[31mFTL\u{1b}[0m Requested rule no-such-rule not found in rules\n";
    let verbose = format!("Finding:     aws_key = \"{key}\"\nSecret:      {key}\nRuleID:      aws-access-token\nFile:        config.py\n");
    let rows: [(&str, Vec<u8>, &str); 8] = [
        ("the real unknown-id stderr", real.to_vec(), "Requested rule no-such-rule not found in rules"),
        ("the same, coloured", coloured.into(), "Requested rule no-such-rule not found in rules"),
        ("INF and WRN lines", b"10:04PM INF no leaks found\n10:04PM WRN leaks found: 1\n".to_vec(), ""),
        ("an ERR and an FTL line", b"1:00AM ERR bad path\n1:00AM DBG x\n1:00AM FTL  stop \n".to_vec(), "bad path\nstop"),
        ("a verbose finding block", verbose.into_bytes(), ""),
        ("a finding line whose second word is ERR", format!("Finding:     ERR = \"{key}\"\n").into_bytes(), ""),
        ("an FTL line with no time first", b"config FTL bad path\n".to_vec(), ""),
        ("nothing", Vec::new(), ""),
    ];
    for (name, stderr, want) in rows {
        let got = String::from_utf8(error_lines(&stderr)).unwrap();
        assert!(!got.contains(&key), "{name}: the key survived");
        assert_eq!(got, want, "{name}");
    }
}

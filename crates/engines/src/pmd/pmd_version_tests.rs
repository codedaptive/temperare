//! ZUT for pmd_version.rs: the version read from PMD 7.29.0's real `--version` stdout (fixtures/version.txt).

use super::pmd_version;
use procio::Output;

#[test]
fn pmd_version_reads_the_pmd_line_past_the_banner() {
    let real: &[u8] = include_bytes!("fixtures/version.txt");
    let rows: [(&str, Option<i32>, &[u8], Option<&str>); 3] = [
        ("the real output", Some(0), real, Some("7.29.0")),
        ("a failed probe", Some(1), real, None),
        ("no PMD line", Some(0), b"Java version: 27.0.1.1\n", None),
    ];
    for (name, code, stdout, want) in rows {
        let out = Output { code, stdout: stdout.to_vec(), stderr: Vec::new() };
        assert_eq!(pmd_version(&out), want.map(str::to_string), "{name}");
    }
}

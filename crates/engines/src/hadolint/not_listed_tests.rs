//! ZUT for not_listed.rs, against temperare's real list (hadolint_codes.txt): a listed code is Ok; DL1000, an unknown
//! DL code and an SC code are NotListed naming hadolint 2.15.1, the SC code with why no SC code is listed; a list
//! with no version line is Failed.

use super::not_listed;
use crate::types::Stop;

const CODES: &str = include_str!("hadolint_codes.txt");

#[test]
fn not_listed_cases() {
    let listed = |code: &str| format!("{code} is not in temperare's list of hadolint codes (taken from hadolint 2.15.1)");
    let rows: [(&str, &str, &str, Result<(), Stop>); 6] = [
        ("a listed code", CODES, "DL3006", Ok(())),
        ("a rule off by default", CODES, "DL1001", Ok(())),
        ("the parse failure", CODES, "DL1000", Err(Stop::NotListed { said: listed("DL1000") })),
        ("an unknown DL code", CODES, "DL9999", Err(Stop::NotListed { said: listed("DL9999") })),
        (
            "an SC code",
            CODES,
            "SC2086",
            Err(Stop::NotListed {
                said: format!(
                    "{}: hadolint runs ShellCheck without its optional checks and leaves out codes of its own, so tm lists no SC code",
                    listed("SC2086")
                ),
            }),
        ),
        (
            "a list tm cannot read",
            "# source: test\nDL3006\n",
            "DL3006",
            Err(Stop::Failed {
                code: None,
                stderr: "temperare's list of hadolint codes cannot be read: no \"# version:\" line".to_string(),
            }),
        ),
    ];
    for (label, codes, check, expected) in rows {
        assert_eq!(not_listed(codes, check), expected, "{label}");
    }
}

//! Unit test for detekt_args.rs: the files, config layers and report path as detekt's arguments, and the paths and the
//! length it refuses.

use super::detekt_args;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const LAYER: &str = "/tmp/cctao-detekt-1-0.yml";
const REPORT: &str = "/tmp/cctao-detekt-report-1-1.sarif";

/// What a row expects: the whole argument list, only the `--input` value's length in bytes, or the why.
enum Want {
    Args(Vec<String>),
    InputBytes(usize),
    Why(&'static str),
}

#[test]
fn detekt_args_cases() {
    let comma = "cannot give detekt the path src/a,b.kt: detekt splits its paths on `,` and `;`";
    let semicolon = "cannot give detekt the path /w/kt/lint;old.yml: detekt splits its paths on `,` and `;`";
    let too_long = "too many Kotlin files for detekt's --input (more than 98304 bytes in one argument); narrowing the \
                    rule's paths keeps the list short";
    // 8191 files of 11 bytes (`sr/F0000.kt`), one of 12 (`sr/Last0.kts`) and 8191 commas: 98304 bytes, the bound.
    let at_bound: Vec<String> =
        (0..8191).map(|n| format!("sr/F{n:04}.kt")).chain(["sr/Last0.kts".to_string()]).collect();
    let over_bound: Vec<String> = at_bound.iter().cloned().chain(["a.kt".to_string()]).collect();
    let rows: Vec<(&str, Vec<String>, Option<&str>, Want)> = vec![
        (
            "two files and the repository's config",
            vec!["src/Demo.kt".into(), "build.gradle.kts".into()],
            Some("/w/kt/detekt.yml"),
            Want::Args(vec![
                "--input".into(),
                "src/Demo.kt,build.gradle.kts".into(),
                "--config".into(),
                format!("/w/kt/detekt.yml,{LAYER}"),
                "--report".into(),
                format!("sarif:{REPORT}"),
            ]),
        ),
        (
            "one file and no config",
            vec!["Main.kt".into()],
            None,
            Want::Args(vec!["--input".into(), "Main.kt".into(), "--config".into(), LAYER.into(), "--report".into(), format!("sarif:{REPORT}")]),
        ),
        ("a file holding a comma", vec!["src/Ok.kt".into(), "src/a,b.kt".into()], None, Want::Why(comma)),
        ("a config holding a semicolon", vec!["Main.kt".into()], Some("/w/kt/lint;old.yml"), Want::Why(semicolon)),
        ("an input of exactly 98304 bytes", at_bound, None, Want::InputBytes(98304)),
        ("an input over 98304 bytes", over_bound, None, Want::Why(too_long)),
    ];
    for (name, files, config, want) in rows {
        let files: Vec<PathBuf> = files.into_iter().map(PathBuf::from).collect();
        let got = detekt_args(&files, config.map(Path::new), Path::new(LAYER), Path::new(REPORT));
        match want {
            Want::Args(args) => assert_eq!(got, Ok(args.into_iter().map(OsString::from).collect()), "{name}"),
            Want::InputBytes(bytes) => assert_eq!(got.map(|args| args[1].len()), Ok(bytes), "{name}"),
            Want::Why(why) => assert_eq!(got, Err(why.to_string()), "{name}"),
        }
    }
}

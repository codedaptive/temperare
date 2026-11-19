use std::fs;

use engines::{availability, clippy::{clippy_cmd, parse_clippy}, Availability};
use rules::{Engine, RelPath};

#[test]
fn clippy_reports_only_the_requested_lint() {
    assert_eq!(availability(Engine::Clippy), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-clippy", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::create_dir(folder.join("src")).unwrap();
    fs::write(folder.join("Cargo.toml"), "[package]\nname = \"clippy_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n").unwrap();
    fs::write(folder.join("src/lib.rs"), "pub fn compare(value: bool) -> bool {\n    let unused = 1;\n    return value == true;\n}\npub fn clean(value: bool) -> bool { value }\n").unwrap();
    let mut cmd = clippy_cmd("clippy::bool_comparison", &[], &folder);
    cmd.env.push(("CARGO_TARGET_DIR".into(), folder.join("target").into_os_string()));
    cmd.env.push(("CARGO_NET_OFFLINE".into(), "true".into()));
    let result = procio::run(&cmd).map(|out| parse_clippy(&out, "clippy::bool_comparison", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("src/lib.rs"));
    assert_eq!(hits[0].line, 3);
    assert!(hits[0].what.contains("true"), "{:?}", hits[0]);
}

#[test]
fn dashed_unknown_lint_from_real_clippy_is_an_error() {
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-clippy-dashed-unknown", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::create_dir(folder.join("src")).unwrap();
    fs::write(folder.join("Cargo.toml"), "[package]\nname = \"clippy_dashed_unknown_zit\"\nversion = \"0.1.0\"\nedition = \"2021\"\n").unwrap();
    fs::write(folder.join("src/lib.rs"), "pub fn compare(x: bool) -> bool { x == true }\n").unwrap();
    let mut cmd = clippy_cmd("clippy::bool-comparisn", &[], &folder);
    cmd.env.push(("CARGO_TARGET_DIR".into(), folder.join("target").into_os_string()));
    cmd.env.push(("CARGO_NET_OFFLINE".into(), "true".into()));
    let result = procio::run(&cmd).map(|out| parse_clippy(&out, "clippy::bool-comparisn", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let got = result.unwrap();
    assert_eq!(got.unwrap_err().why, "clippy did not take check `clippy::bool-comparisn` whole: unknown lint: `clippy::bool_comparisn`");
}

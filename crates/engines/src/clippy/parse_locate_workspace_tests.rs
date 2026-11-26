//! ZUT for parse_locate_workspace.rs. `locate_workspace.txt` is the stdout of `cargo locate-project --workspace
//! --message-format plain`, captured once (exit 0) from the `member/` folder of a throwaway workspace; `CAPTURED` is
//! the root path it names; no test reads that folder. The other outputs are hand-built.
//! The capture's home-folder path is replaced by `/work/project`, so it and `CAPTURED` start with it.

use super::parse_locate_workspace;
use procio::Output;
use std::path::PathBuf;

// The capture's path is a unix absolute path, not absolute on Windows.
#[cfg(unix)]
const CAPTURED: &str = "/work/project/locate-capture";
#[cfg(unix)]
const LOCATED: &[u8] = include_bytes!("fixtures/locate_workspace.txt");
/// An absolute folder on this platform: `/w` is not absolute on Windows.
const W: &str = if cfg!(windows) { "C:/w" } else { "/w" };

fn output(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

// The capture's path is a unix absolute path, not absolute on Windows.
#[cfg(unix)]
#[test]
fn the_captured_output_names_the_workspace_root_folder() {
    assert_eq!(parse_locate_workspace(&output(0, LOCATED, b"")), Ok(PathBuf::from(CAPTURED)));
}

#[test]
fn surrounding_whitespace_is_trimmed() {
    let got = parse_locate_workspace(&output(0, format!("  {W}/Cargo.toml \n\n").as_bytes(), b""));
    assert_eq!(got, Ok(PathBuf::from(W)));
}

#[test]
fn a_non_zero_exit_is_a_parse_error_quoting_stderr() {
    let stderr = b"error: could not find `Cargo.toml` in `/w` or any parent directory\n";
    let why = parse_locate_workspace(&output(101, b"", stderr)).unwrap_err().why;
    assert!(why.starts_with("cargo locate-project"), "{why}");
    assert!(why.contains("error: could not find `Cargo.toml` in `/w` or any parent directory"), "{why}");
}

#[test]
fn a_non_zero_exit_with_a_path_printed_is_still_a_parse_error() {
    assert!(parse_locate_workspace(&output(1, b"/w/Cargo.toml\n", b"")).is_err());
}

#[test]
fn empty_output_is_a_parse_error() {
    let why = parse_locate_workspace(&output(0, b"", b"")).unwrap_err().why;
    assert!(why.starts_with("cargo locate-project"), "{why}");
}

#[test]
fn a_relative_path_is_a_parse_error_quoting_it() {
    let why = parse_locate_workspace(&output(0, b"w/Cargo.toml\n", b"")).unwrap_err().why;
    assert!(why.contains("w/Cargo.toml"), "{why}");
}

#[test]
fn a_path_not_ending_in_cargo_toml_is_a_parse_error() {
    assert!(parse_locate_workspace(&output(0, format!("{W}/Cargo.lock\n").as_bytes(), b"")).is_err());
    assert!(parse_locate_workspace(&output(0, format!("{W}/member\n").as_bytes(), b"")).is_err());
}

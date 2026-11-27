//! Unit test for parse_hadolint.rs: real hadolint 2.15.1 output (fixtures/, see commands.txt) read into the hits of one
//! check; a config hadolint could not read, a Dockerfile it could not parse, and stdout that is not one SARIF log are
//! no answer. `DIR` stands for the folder hadolint ran in; no folder is read.

use super::parse_hadolint;
use crate::types::{Hit, ParseError};
use procio::Output;
use rules::RelPath;
use std::path::Path;

const DIR: &str = "/w/docker";
const FINDINGS: &[u8] = include_bytes!("fixtures/findings.sarif");
const NOT_SARIF: &str = "SARIF output is not a SARIF log: ";

fn hit(line: u32, what: &str) -> Hit {
    Hit { path: RelPath::new("Dockerfile"), line: Some(line), col: Some(1), what: what.to_string() }
}

/// What a row expects: these hits, this exact error, or `parse_sarif`'s error for stdout that is not a SARIF log.
enum Want {
    Hits(Vec<Hit>),
    Error(&'static str),
    NotSarif,
}

#[test]
fn parse_hadolint_cases() {
    use Want::{Error, Hits, NotSarif};
    let dl3006 = hit(1, "Always tag the version of an image explicitly");
    let dl3008 = hit(
        2,
        "Pin versions in apt get install. Instead of `apt-get install <package>` use `apt-get install <package>=<version>`",
    );
    let unparsed = "hadolint could not parse Dockerfile:1, so its other findings there are missing: unexpected 'F'\n\
        expecting '#', ADD, ARG, CMD, COPY, ENTRYPOINT, ENV, EXPOSE, FROM, HEALTHCHECK, LABEL, MAINTAINER, ONBUILD, RUN, \
        SHELL, STOPSIGNAL, USER, VOLUME, WORKDIR, a pragma, end of input, or whitespaces";
    let config = "hadolint could not read its config and would run on its defaults";
    let rows: [(&str, &[u8], &[u8], &str, Want); 9] = [
        ("findings, DL3006", FINDINGS, b"", "DL3006", Hits(vec![dl3006])),
        ("findings, DL3008", FINDINGS, b"", "DL3008", Hits(vec![dl3008])),
        ("findings, a code with none", FINDINGS, b"", "DL9999", Hits(vec![])),
        ("clean", include_bytes!("fixtures/clean.sarif"), b"", "DL3006", Hits(vec![])),
        ("a Dockerfile hadolint could not parse", include_bytes!("fixtures/parse_error.sarif"), b"", "DL3006", Error(unparsed)),
        ("a config hadolint could not read", FINDINGS, include_bytes!("fixtures/config_error.stderr"), "DL3006", Error(config)),
        ("a second format", include_bytes!("fixtures/double_format.out"), b"", "DL3006", NotSarif),
        ("empty stdout", b"", b"", "DL3006", NotSarif),
        ("empty stdout with the config error", b"", include_bytes!("fixtures/config_error.stderr"), "DL3006", Error(config)),
    ];
    let mut failed = Vec::new();
    for (label, stdout, stderr, check, want) in rows {
        let out = Output { code: Some(1), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        let got = parse_hadolint(&out, check, Path::new(DIR), Path::new(DIR));
        let ok = match (&got, want) {
            (Ok(hits), Hits(want)) => hits == &want,
            (Err(ParseError { why }), Error(want)) => why == want,
            (Err(ParseError { why }), NotSarif) => why.starts_with(NOT_SARIF),
            _ => false,
        };
        if !ok {
            eprintln!("{label}: got {got:?}");
            failed.push(label);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

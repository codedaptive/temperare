//! ZUT for stop.rs: plain data; equality covers the variant and every field, `NotOnPath`,
//! `UnknownCheck`, `NotListed`, `NoFiles` and `NoCargoProject` included.

use super::Stop;

fn failed(code: Option<i32>, stderr: &str) -> Stop {
    Stop::Failed { code, stderr: stderr.to_string() }
}

#[test]
fn stops_compare_by_variant_and_every_field() {
    let absent = Stop::NotOnPath { program: "ruff".to_string() };
    assert_eq!(absent, Stop::NotOnPath { program: "ruff".to_string() });
    assert_ne!(absent, Stop::NotOnPath { program: "swiftlint".to_string() });
    assert_ne!(absent, Stop::NotAvailable { why: "ruff not found on PATH".to_string() });
    assert_eq!(failed(Some(2), "bad"), failed(Some(2), "bad"));
    assert_ne!(failed(Some(2), "bad"), failed(None, "bad"));
    assert_ne!(failed(Some(2), "bad"), failed(Some(2), "worse"));
    assert_ne!(absent, failed(None, "ruff not found on PATH"));
}

#[test]
fn an_unknown_check_compares_by_what_the_engine_said_and_is_no_other_stop() {
    let unknown = |said: &str| Stop::UnknownCheck { said: said.to_string() };
    assert_eq!(unknown("Unknown rule selector: `ZZZ999`"), unknown("Unknown rule selector: `ZZZ999`"));
    assert_ne!(unknown("Unknown rule selector: `ZZZ999`"), unknown("Unknown rule selector: `E711`"));
    assert_ne!(unknown("x"), Stop::NotAvailable { why: "x".to_string() });
    assert_ne!(unknown("x"), failed(None, "x"));
}

#[test]
fn no_files_compares_by_its_reason_and_is_no_other_stop() {
    let none = |why: &str| Stop::NoFiles { why: why.to_string() };
    assert_eq!(none("no file its engine checks"), none("no file its engine checks"));
    assert_ne!(none("no file its engine checks"), none("its paths match no file in the rule file's folder"));
    assert_ne!(none("x"), Stop::NotAvailable { why: "x".to_string() });
    assert_ne!(none("x"), Stop::UnknownCheck { said: "x".to_string() });
    assert_ne!(none("x"), failed(None, "x"));
}

#[test]
fn not_listed_compares_by_what_it_said_and_is_no_other_stop() {
    let unlisted = |said: &str| Stop::NotListed { said: said.to_string() };
    let sc9999 = "SC9999 is not in cctao's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(unlisted(sc9999), unlisted(sc9999));
    assert_ne!(unlisted(sc9999), unlisted("SC9998 is not in cctao's list of shellcheck codes (taken from shellcheck 9.9.9)"));
    assert_ne!(unlisted("x"), Stop::UnknownCheck { said: "x".to_string() });
    assert_ne!(unlisted("x"), Stop::NotAvailable { why: "x".to_string() });
    assert_ne!(unlisted("x"), Stop::NoFiles { why: "x".to_string() });
    assert_ne!(unlisted("x"), failed(None, "x"));
}

#[test]
fn no_cargo_project_compares_by_its_why_and_is_no_other_stop() {
    let no_cargo = |why: &str| Stop::NoCargoProject { why: why.to_string() };
    let why = "no Cargo.toml at or above the rule file's folder";
    assert_eq!(no_cargo(why), no_cargo(why));
    assert_ne!(no_cargo(why), no_cargo("no Cargo.toml"));
    assert_ne!(no_cargo("x"), Stop::NotAvailable { why: "x".to_string() });
    assert_ne!(no_cargo("x"), Stop::NoFiles { why: "x".to_string() });
    assert_ne!(no_cargo("x"), Stop::UnknownCheck { said: "x".to_string() });
    assert_ne!(no_cargo("x"), Stop::NotListed { said: "x".to_string() });
    assert_ne!(no_cargo("x"), failed(None, "x"));
}

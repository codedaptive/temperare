//! ZUT for kind.rs: each variant serializes to its rule-file name and reads back from it.

use super::Kind;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Wrap {
    kind: Kind,
}

#[test]
fn quality_serializes_as_quality() {
    assert_eq!(toml::to_string(&Wrap { kind: Kind::Quality }).unwrap(), "kind = \"quality\"\n");
}

#[test]
fn security_serializes_as_security() {
    assert_eq!(toml::to_string(&Wrap { kind: Kind::Security }).unwrap(), "kind = \"security\"\n");
}

#[test]
fn both_names_read_back() {
    assert_eq!(toml::from_str::<Wrap>("kind = \"quality\"\n").unwrap().kind, Kind::Quality);
    assert_eq!(toml::from_str::<Wrap>("kind = \"security\"\n").unwrap().kind, Kind::Security);
}

#[test]
fn another_name_does_not_read() {
    assert!(toml::from_str::<Wrap>("kind = \"Security\"\n").is_err());
}

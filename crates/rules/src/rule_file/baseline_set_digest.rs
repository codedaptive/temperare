//! `baseline_set_digest`: a fixed 64-bit FNV-1a hash of a baseline set's rule-file text and carried files, for tests
//! that pin a released set so it cannot change.
//!
//! The hash is written out here rather than taken from std, because std's `DefaultHasher` may change between Rust
//! releases and a pinned literal must stay the same across them. It feeds the set's rule-file text, then each YAML
//! file's path and text, then each case file's path and text, in the set's order. A 0xFF byte ends every field and
//! one 0xFE byte stands between the YAML files and the case files; neither byte occurs in UTF-8 text, so no field
//! runs into the next, and moving a file from the YAML files to the case files changes the digest. The set's name,
//! version and readiness are not fed.

use super::baseline_set::BaselineSet;

const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;
const FIELD_END: u8 = 0xFF;
const YAML_END: u8 = 0xFE;

/// The FNV-1a 64 digest of `set`'s text, YAML files and case files.
pub(super) fn baseline_set_digest(set: &BaselineSet) -> u64 {
    let mut h = feed_field(OFFSET_BASIS, set.text);
    h = feed_files(h, set.yaml);
    h = feed(h, &[YAML_END]);
    feed_files(h, set.cases)
}

/// `h` after each (path, text) in `files`, each field ended by `FIELD_END`.
fn feed_files(h: u64, files: &[(&str, &str)]) -> u64 {
    files.iter().fold(h, |h, (path, text)| feed_field(feed_field(h, path), text))
}

/// `h` after `field`'s bytes and `FIELD_END`.
fn feed_field(h: u64, field: &str) -> u64 {
    feed(feed(h, field.as_bytes()), &[FIELD_END])
}

/// `h` after FNV-1a's step over each of `bytes`.
fn feed(h: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(h, |h, b| (h ^ u64::from(*b)).wrapping_mul(PRIME))
}

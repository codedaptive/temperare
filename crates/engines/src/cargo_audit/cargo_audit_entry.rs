//! `CARGO_AUDIT`: the cargo-audit engine as an entry.

use super::{cargo_audit_refusal, parse_cargo_audit, run_cargo_audit};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// cargo-audit run by its own flow, `run_cargo_audit`, with the command `cargo_audit_cmd` builds:
/// `cargo-audit audit --no-fetch --no-yanked --json --db <db> --file <folder>/Cargo.lock`, run in the rule file's
/// folder and read by `parse_cargo_audit`. The database is cargo-audit's default, `$CARGO_HOME/advisory-db`, else
/// `$HOME/.cargo/advisory-db`, and cctao never fetches it: `--no-fetch` reads it as it is and `--no-yanked` skips the
/// crates.io index, so nothing reaches the network. No environment is set; CARGO_HOME and HOME are only read to find
/// the database. A database with no `crates` folder makes the rule not available, naming
/// `cargo audit update-db --db <db>` to fetch it. A check is an advisory id (`RUSTSEC-YYYY-NNNN`) or a kind: `all`,
/// `vulnerability`, `unmaintained`, `unsound` or `notice`; anything else, `yanked` included, is refused before anything
/// runs (`cargo_audit_refusal`). The check is never passed to cargo-audit: it reports every advisory and the reader
/// keeps the rule's. An id the database does not hold is `UnknownCheck`, settled by the flow, so `unknown` gives none.
/// A folder with no Cargo.lock, or a rule whose paths do not cover it, is `NoFiles` before availability
/// (`lock_listed`). A report showing an audit.toml applied (settings other than the defaults) is a parse failure:
/// the rule file is the only home for an exception. Every finding is on Cargo.lock, with no line.
pub static CARGO_AUDIT: Entry = Entry {
    name: "cargo-audit",
    engine: Engine::CargoAudit,
    program: "cargo-audit",
    needs: &["cargo-audit"],
    never: None,
    args: &["audit", "--no-fetch", "--no-yanked", "--json"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: cargo_audit_refusal,
    unknown: |_, _| None,
    read: parse_cargo_audit,
    run: run_cargo_audit,
};

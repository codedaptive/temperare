//! `TESSERA`: the tessera engine as an entry, never available.

use super::hits_tessera;
use crate::entry::{Entry, Params, Pick};
use crate::ParseError;
use rules::Engine;

/// `ts slop --json` in the rule file's folder, which no rule runs: Tessera's slop records are not defined yet, so the
/// entry is never available, nothing is looked up or run, and its version is never read. Its reader
/// is never reached and says so. It takes no params, so a rule with one is refused before any flow (`refused`, through
/// `param_args`) and is `Failed`.
pub static TESSERA: Entry = Entry {
    name: "tessera",
    engine: Engine::Tessera,
    program: "ts",
    needs: &[],
    never: Some("Tessera's slop records are not defined yet"),
    args: &["slop", "--json"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: None,
    refuse: |_| None,
    unknown: |_, _| None,
    read: |_, _, _, _| Err(ParseError { why: "tessera has no reader".to_string() }),
    run: hits_tessera,
};

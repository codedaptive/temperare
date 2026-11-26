//! `PYLINT`: the pylint engine as an entry.

use super::{parse_pylint, pylint_refusal, unknown_message};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `pylint --persistent=n --output-format=json2 --disable=all --enable=unknown-option-value,useless-option-value
/// --enable=<check> [--rcfile=<config>] .` run in the rule file's folder, read by `parse_pylint`. json2 is pylint's
/// current JSON reporter and carries each message's id; the legacy `json` reporter names it `message-id`.
/// `--disable=all` then `--enable=<check>` runs the one message; pylint applies its command line over the rcfile, so
/// a config that disables the check does not hide it. `unknown-option-value` (W0012) and `useless-option-value`
/// (R0022) stay on so pylint says when it does not know the check (`unknown_message`), which makes the rule not
/// available. `--persistent=n` keeps pylint from writing its stats file under the user's home; pylint never
/// reaches the network.
///
/// The check is one pylint message id (`C0121`); anything else, and an old id pylint renamed, is refused before
/// running (`pylint_refusal`), since its findings would carry other ids and read clean. Pylint takes no params: any
/// param makes the rule `Failed` before running. Pylint rules never share a run: `shares_with` has no arm for it.
pub static PYLINT: Entry = Entry {
    name: "pylint",
    engine: Engine::Pylint,
    program: "pylint",
    needs: &["pylint"],
    never: None,
    args: &["--persistent=n", "--output-format=json2", "--disable=all", "--enable=unknown-option-value,useless-option-value"],
    pick: Pick::Arg("--enable="),
    params: Params::Args(&[]),
    config: Some("--rcfile="),
    after: &["."],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: pylint_refusal,
    unknown: unknown_message,
    read: parse_pylint,
    run: run_plain,
};

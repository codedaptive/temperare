//! `run_eslint_with`: ESLint's flow for one rule, keeping the files a given listing holds, and settling a run that
//! linted none of them in the rule's paths as no file.

use super::{linted_files, parse_eslint};
use crate::entry::{run_plain, Entry};
use crate::host::Host;
use crate::no_file_match::no_file_match;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, Rule};
use std::cell::RefCell;
use std::ffi::OsString;
use std::path::Path;

/// Why a rule whose run linted no listed file ran nothing.
const NO_LINTED_FILE_WHY: &str =
    "ESLint linted no file in the rule's paths that git lists (its config decides which files it lints)";

/// `run_plain` over a host whose `run` puts `--rule` and `{"<check>":"error"}` just before the last argument (the
/// `.` ESLint lints) and runs the command with `host.run`. The severity-only `--rule` turns the check on and keeps
/// the options the config gives it. `on_path`, `raw_chars` and `base` are `host`'s.
///
/// The run's stdout names every file ESLint linted. ESLint's config, not cctao, decides which files those are, so the
/// no-file case is settled here, after the run: when stdout is a JSON array and none of its files is both in what
/// `listed` gives for `folder` (any file counts when it gives `None`) and one `rule` applies to, the rule is
/// `NoFiles`, keeping the count `run_plain` set, since ESLint ran. Else its hits are those in a listed file (all of them
/// when `None`), so a hit in a file git ignores is dropped; `findings_from` applies the rule's paths to them.
///
/// A run whose stdout is that array but which `run_plain` calls `Failed` holds a fatal message, or output
/// `parse_eslint` cannot read, and its stderr need not say so (Node's warnings land there, such as ESLint's own
/// warning for an empty config), so its stderr is `parse_eslint`'s why, naming the file ESLint could not parse or
/// what it could not read. Every other stop passes through unchanged.
pub(crate) fn run_eslint_with(
    listed: &dyn Fn(&Path) -> Option<Vec<RelPath>>,
    entry: &Entry,
    rule: &Rule,
    folder: &Path,
    host: &Host,
) -> Result<Vec<Hit>, Stop> {
    let rule_arg = serde_json::json!({ rule.check.as_str(): "error" }).to_string();
    let seen: RefCell<Option<Output>> = RefCell::new(None);
    let run = |cmd: &Cmd| {
        let mut cmd = cmd.clone();
        let last = cmd.args.len().saturating_sub(1);
        cmd.args.splice(last..last, [OsString::from("--rule"), OsString::from(&rule_arg)]);
        let got = (host.run)(&cmd);
        *seen.borrow_mut() = got.as_ref().ok().cloned();
        got
    };
    let got = run_plain(entry, rule, folder, &Host { run: &run, on_path: host.on_path, raw_chars: host.raw_chars, base: host.base });
    let Some(out) = seen.into_inner() else { return got };
    let Some(linted) = linted_files(&out, folder) else { return got };
    match got {
        Ok(hits) => settled(hits, &linted, listed(folder), rule),
        Err(Stop::Failed { code, stderr }) => {
            let stderr = parse_eslint(&out, &rule.check, folder, folder).err().map_or(stderr, |err| err.why);
            Err(Stop::Failed { code, stderr })
        }
        Err(stop) => Err(stop),
    }
}

/// `NoFiles` when `no_file_match` says `rule` applies to none of the `linted` files in `listing`; else the `hits` in
/// `listing`. `None` lists every file.
fn settled(hits: Vec<Hit>, linted: &[RelPath], listing: Option<Vec<RelPath>>, rule: &Rule) -> Result<Vec<Hit>, Stop> {
    let listed = |path: &RelPath| listing.as_ref().is_none_or(|files| files.contains(path));
    if no_file_match(rule, &linted.iter().filter(|path| listed(path)).cloned().collect::<Vec<_>>()) {
        return Err(Stop::NoFiles { why: NO_LINTED_FILE_WHY.to_string() });
    }
    Ok(hits.into_iter().filter(|hit| listed(&hit.path)).collect())
}

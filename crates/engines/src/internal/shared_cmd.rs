//! `shared_cmd`: a rule's check run turned into the run it shares with the other rules of its group.

use crate::entry::{Entry, Pick};
use procio::Cmd;
use std::ffi::OsString;

/// `(key, run)` for the rule's check run `cmd`, whose pick argument is `cmd.args[at]` (`pick_at`'s index): `run` is
/// the command to run, `key` what the run is replayed under, equal for every rule of the group. `Pick::Arg`: `run` is
/// `cmd` with that one argument replaced by `union` (`union_args`), and `key` is `run`. `Pick::File`: `run` is `cmd`
/// unchanged, as the rules share one check file; `key` is `cmd` with that argument emptied, since it is the path of
/// the rule's own filled copy, different for each rule. `pick_at` gives no index for `Pick::None`, so it never comes
/// here; it shares the File arm only so the match is whole.
pub(crate) fn shared_cmd(entry: &Entry, cmd: &Cmd, at: usize, union: &[OsString]) -> (Cmd, Cmd) {
    let mut key = cmd.clone();
    match entry.pick {
        Pick::Arg(_) => {
            key.args.splice(at..=at, union.iter().cloned());
            (key.clone(), key)
        }
        Pick::File(_) | Pick::None => {
            if let Some(path) = key.args.get_mut(at) {
                *path = OsString::new();
            }
            (key, cmd.clone())
        }
    }
}

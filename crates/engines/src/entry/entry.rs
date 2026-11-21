//! `Entry`: everything tm knows about one engine, as data.

use super::{Params, Pick, ReadFn, RunFn, VersionProbe};
use procio::Output;
use rules::Engine;

/// One engine: how a rule naming it becomes a command, and how the command's output becomes hits. An adapter is one
/// `static` of this type in the engine's folder; a rule file names the engine, never a command line.
///
/// Every field is public except `run`: the flow takes the crate's own host and stop types, so an entry is built
/// inside this crate (one folder per engine).
#[derive(Debug)]
pub struct Entry {
    /// The engine's name in the rule file ("clippy"); messages name the engine by it.
    pub name: &'static str,
    /// The engine this entry serves.
    pub engine: Engine,
    /// The program the check run runs ("cargo", "ruff").
    pub program: &'static str,
    /// The programs PATH must hold for the engine to be available, looked up in order; the first missing one is
    /// named in the why (clippy: `["cargo-clippy", "clippy-driver"]`).
    pub needs: &'static [&'static str],
    /// `Some(why)` when the engine is never available: nothing is looked up or run, and every rule naming it is not
    /// available with this why (tessera). `None` for an engine tm can run.
    pub never: Option<&'static str>,
    /// The fixed arguments that come first, before the check.
    pub args: &'static [&'static str],
    /// How a rule's `check` reaches the engine.
    pub pick: Pick,
    /// Which of a rule's `params` the engine accepts, and how each reaches it.
    pub params: Params,
    /// The option a rule's `config` is joined to, with its `=` (`"--config="`), giving one argument after the
    /// params; `None` when the engine is not given the rule's config. On a threshold run the config argument comes
    /// from `threshold_run`, ahead of the threshold, and `entry_cmd` gets none.
    pub config: Option<&'static str>,
    /// The fixed arguments that come last (ruff's `"."`).
    pub after: &'static [&'static str],
    /// How the engine's version is read; `None` when it is never read (an engine that never runs).
    pub version: Option<VersionProbe>,
    /// Why tm refuses a check before anything runs (a check whose findings tm could not match back to it); `None`
    /// to run it. An engine that refuses nothing gives a function returning `None`.
    pub refuse: fn(&str) -> Option<String>,
    /// `(out, check)`: the engine's own words, from the check run's output, saying it does not know `check`, the
    /// rule's check; `None` when it said no such thing, or said it of another name (one in the rule's config file,
    /// say), so the rule is never blamed for a name it did not give. Such a rule is not available, as an engine too
    /// old for the check. An engine that never says so gives a function returning `None`.
    pub unknown: fn(&Output, &str) -> Option<String>,
    /// The reader: the check run's output into hits.
    pub read: ReadFn,
    /// The flow that runs one rule: `run_plain` for an engine whose check run is one command built by `entry_cmd`;
    /// an engine with steps of its own (finding a workspace, filling a check file) gives its own function.
    pub(crate) run: RunFn,
}

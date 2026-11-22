//! `run_group`: the rules of one shared group run through one check run, each rule read as if it had run alone.

use crate::entry_for::entry_for;
use crate::host::Host;
use crate::pick_at::pick_at;
use crate::replayed_run::replayed_run;
use crate::rule_host::rule_host;
use crate::rule_run_of::rule_run_of;
use crate::shared_cmd::shared_cmd;
use crate::types::{EngineVersion, Hit, HitsFn, RuleRun, Stop};
use crate::union_args::union_args;
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, Rule, RuleFile};
use std::cell::{Cell, RefCell};
use std::io;
use std::path::Path;

/// The commands already run, each with its result, replayed to every later rule that runs the same one.
type Seen = RefCell<Vec<(Cmd, Result<Output, ProcError>)>>;

/// One rule's flow: what `hits` gave, and the characters its check run counted (`check_run`), if it read one.
type Flowed = (Result<Vec<Hit>, Stop>, Option<usize>);

/// One `RuleRun` per rule of `rules`, in the order given, for rules that pairwise share a run (`shares_with`, which
/// is not checked here), none of them refused (`run_rules_listed` settles those first); a rule with `only_with_base`
/// and no base stops in its flow (`engine_hits`) before its check run, so step 1 keeps it. Every rule goes through its
/// flow (`hits`, `engine_hits` in production) with a host of its own, made by `rule_host` in both passes, so it keeps
/// the check's `on_path` and `base`, and its unknown-check test and reader (which keeps only its own check's hits)
/// apply as when it runs alone; only its `run` is shared. Any other command (clippy's `clippy-driver -W help`,
/// `cargo locate-project`) runs through `host.run` once per distinct command, replayed to the rest in every pass.
/// 1. Probe pass: each rule's flow runs until its check run (`pick_at`), which is not run. A rule that stops before
///    it (no script or Cargo.lock in scope, engine not available, folder unresolved, a clippy lint group or unlisted lint) keeps
///    that outcome, with no count, exactly as alone. Only the rules that reach their check run put their check
///    into the shared run, so a check that would stop before running never changes what the run does for the others.
/// 2. Run pass: each rule that reached runs its flow again; its check run is replaced by one run over the distinct
///    checks of those rules (`union_args`, `shared_cmd`), run for real for the first and replayed to the rest.
/// 3. A rule whose flow ended in `Stop::UnknownCheck` after reading the run is settled as `stopped` says. If any was,
///    the others are run again by step 2 without those checks, as a new real run: ruff names only the first selector
///    it does not know and fails the whole run, so the others had no answer. Repeat until a pass finds no unknown
///    check; then every rule of that pass is settled with its outcome.
/// 4. Counts: a shared run is one counted run, its characters counted once. Each real check run's characters go on
///    the first rule, in the given order, that it settled, as `Some(n)`; every other rule gets `None`, never
///    `Some(0)`, since a count of rules with `Some` counts the runs. Each rerun after an unknown check is one more
///    real run, counted on the first rule it settles.
///
/// A shared run that gives no output (program not found, timed out) reaches every rule that shares it as `ran`'s
/// `Stop`, with no count; output no reader can read is each rule's own `Failed`. Each rule is settled from its flow by
/// `rule_run_of`, in either pass, so a flow that ends in `Stop::NoFiles` carries its `no_files` flag.
pub(crate) fn run_group(
    file: &RuleFile,
    rules: &[&Rule],
    folder: Result<&Path, &Stop>,
    only: &[RelPath],
    host: &Host,
    versions: &[EngineVersion],
    hits: &HitsFn,
) -> Vec<RuleRun> {
    let helpers: Seen = RefCell::new(Vec::new());
    let run_of = |at: usize, got: Result<Vec<Hit>, Stop>, raw_chars: Option<usize>| {
        (at, rule_run_of(file, rules[at], got, only, versions, raw_chars))
    };
    let mut done = Vec::new();
    let mut pending = Vec::new();
    for (at, rule) in rules.iter().enumerate() {
        match probed(rule, folder, host, &helpers, hits) {
            (_, true) => pending.push(at),
            (got, false) => done.push(run_of(at, got, None)),
        }
    }
    while !pending.is_empty() {
        let ran = run_pass(rules, &pending, folder, host, &helpers, hits);
        let unknown = |(got, chars): &Flowed| matches!(got, Err(Stop::UnknownCheck { .. })) && chars.is_some();
        let some_unknown = ran.iter().any(unknown);
        let mut chars = ran.iter().find_map(|(_, chars)| *chars);
        let mut rest = Vec::new();
        for (at, flowed) in pending.into_iter().zip(ran) {
            if some_unknown && !unknown(&flowed) {
                rest.push(at);
                continue;
            }
            done.push(run_of(at, flowed.0, chars.take()));
        }
        pending = rest;
    }
    done.sort_by_key(|(at, _)| *at);
    done.into_iter().map(|(_, run)| run).collect()
}

/// The probe flow of `rule`, and whether it reached its check run, which answers as a run that could not start.
fn probed(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    helpers: &Seen,
    hits: &HitsFn,
) -> (Result<Vec<Hit>, Stop>, bool) {
    let entry = entry_for(rule.engine);
    let reached = Cell::new(false);
    let run = |cmd: &Cmd| match entry.and_then(|entry| pick_at(entry, &rule.check, cmd)) {
        Some(_) => {
            reached.set(true);
            Err(ProcError::Spawn { program: cmd.program.clone(), io: io::Error::other("the probe pass runs no check") })
        }
        None => replayed_run(helpers, cmd, cmd, host.run),
    };
    let (got, _) = flowed(rule, folder, host, &run, hits);
    (got, reached.get())
}

/// The flows of the rules at `pending`, in order, their check runs shared as one run over all their checks.
fn run_pass(
    rules: &[&Rule],
    pending: &[usize],
    folder: Result<&Path, &Stop>,
    host: &Host,
    helpers: &Seen,
    hits: &HitsFn,
) -> Vec<Flowed> {
    let checks: Vec<&str> = pending.iter().map(|at| rules[*at].check.as_str()).collect();
    let shared: Seen = RefCell::new(Vec::new());
    pending
        .iter()
        .map(|at| {
            let rule = rules[*at];
            let entry = entry_for(rule.engine);
            let run = |cmd: &Cmd| match entry.and_then(|entry| Some((entry, pick_at(entry, &rule.check, cmd)?))) {
                Some((entry, pick)) => {
                    let (key, union) = shared_cmd(entry, cmd, pick, &union_args(entry, &checks));
                    replayed_run(&shared, &key, &union, host.run)
                }
                None => replayed_run(helpers, cmd, cmd, host.run),
            };
            flowed(rule, folder, host, &run, hits)
        })
        .collect()
}

/// `hits` for `rule` through a host of its own (`rule_host` of the check's `host`, with `run`), with a fresh
/// count.
fn flowed(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    run: &dyn Fn(&Cmd) -> Result<Output, ProcError>,
    hits: &HitsFn,
) -> Flowed {
    let raw_chars = Cell::new(None);
    let got = hits(rule, folder, &rule_host(host, run, &raw_chars));
    (got, raw_chars.get())
}

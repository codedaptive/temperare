//! `shares_with`: whether two rules can be answered by one shared run of their engine.

use crate::entry::{Params, Reach};
use crate::entry_of::entry_of;
use crate::ruff::ruff_selects;
use rules::{Engine, Rule};

/// True when one run of the engine over both checks gives each rule exactly the hits it would get alone. Both rules
/// must name the same engine, `config` and `params` (clippy's `allow` is a param and changes what the run reports),
/// and their params must not reach the engine through a temporary file or folder (`writes_temp_params`); then by
/// engine:
/// - Clippy: the checks are equal (one check, one run, the same reading), or neither is `warnings`, which reads every
///   warning, so another rule's `--warn` would add to it.
/// - Ruff: the checks are equal, or neither selects RUF100, they do not overlap (`ruff_overlap`) and they do not split
///   an incompatible pair (`ruff_splits_a_pair`). RUF100 (unused noqa) calls a `# noqa: E711` unused or
///   "non-enabled" by whether E711 is selected, so its answer depends on the other checks in the run.
/// - SwiftLint: the checks are equal, or neither is `superfluous_disable_command`, which judges a
///   `swiftlint:disable` comment against the rules that are running.
/// - AstGrep: the checks name the same YAML, the rules carry the same YAML text (`embedded_text`: a set rule's YAML,
///   None for a repository rule) and it takes no params. A hit carries no rule file, so different YAMLs cannot be
///   told apart in one output, and a YAML that takes params is filled per rule.
/// - ShellCheck: never. The shared run finds each rule's own check run by its pick argument (`pick_at`), and
///   shellcheck's check run carries none (the check is never passed), so a group would run it as a helper command
///   and count it on no rule.
/// - Git: never. A git rule makes one count of the change for itself, so there is no run to share.
/// - Tessera, and every engine with no arm of its own: never, so a new engine runs alone until it is named here.
pub(crate) fn shares_with(a: &Rule, b: &Rule) -> bool {
    if a.engine != b.engine || a.config != b.config || a.params != b.params || writes_temp_params(a) {
        return false;
    }
    match a.engine {
        Engine::Clippy => a.check == b.check || (a.check != "warnings" && b.check != "warnings"),
        Engine::Ruff => {
            a.check == b.check
                || !(ruff_selects(&a.check, "RUF100")
                    || ruff_selects(&b.check, "RUF100")
                    || ruff_overlap(&a.check, &b.check)
                    || ruff_splits_a_pair(&a.check, &b.check))
        }
        Engine::SwiftLint => {
            a.check == b.check || (a.check != "superfluous_disable_command" && b.check != "superfluous_disable_command")
        }
        Engine::AstGrep => a.check == b.check && a.embedded_text == b.embedded_text && a.params.is_empty(),
        Engine::ShellCheck => false,
        Engine::Git => false,
        Engine::Tessera => false,
        // An engine with no arm here runs alone.
        _ => false,
    }
}

/// True when either `--select` list selects a part of the other: some part is `ALL`, or some non-empty part is a
/// string prefix of a part of the other list (`ruff_selects` on each part). Overlapping selections could make one
/// rule see a code, or a remap warning, that only the other rule's check turned on.
fn ruff_overlap(a: &str, b: &str) -> bool {
    b.split(',').any(|part| ruff_selects(a, part)) || a.split(',').any(|part| ruff_selects(b, part))
}

/// The rule pairs ruff 0.17.4 calls incompatible. Selecting both, it keeps one and drops the other:
/// "`incorrect-blank-line-before-class` (D203) and `blank-line-before-class` (D211) are incompatible. Ignoring
/// `incorrect-blank-line-before-class`." and "`multi-line-summary-first-line` (D212) and
/// `multi-line-summary-second-line` (D213) are incompatible. Ignoring `multi-line-summary-second-line`." So a run
/// whose union selects both can drop findings one rule gets alone.
const RUFF_INCOMPATIBLE_PAIRS: [(&str, &str); 2] = [("D203", "D211"), ("D212", "D213")];

/// True when, for some incompatible pair, the union of the two `--select` lists selects both codes and at least one
/// list selects exactly one of them. A list that selects both alone (`D`) already gets ruff's own pick in its own run,
/// so a union adding nothing to the pair changes nothing.
fn ruff_splits_a_pair(a: &str, b: &str) -> bool {
    RUFF_INCOMPATIBLE_PAIRS.iter().any(|&(x, y)| {
        let union_selects = |code| ruff_selects(a, code) || ruff_selects(b, code);
        let splits = |check| ruff_selects(check, x) != ruff_selects(check, y);
        union_selects(x) && union_selects(y) && (splits(a) || splits(b))
    })
}

/// True when tm writes the rule's params into a temporary file or folder for its run: the entry's thresholds reach
/// it by `Reach::File` or `Reach::Dir` and the rule names one of them. Each call makes a new path
/// (`tm-<name>-<pid>-<n>`, or clippy's `CLIPPY_CONF_DIR`), so two rules' check commands never match as one shared run.
/// A `Reach::Arg` threshold is the same argument for every rule and still shares.
fn writes_temp_params(rule: &Rule) -> bool {
    match entry_of(rule.engine).params {
        Params::Thresholds { thresholds, reach: Reach::File { .. } | Reach::Dir { .. }, .. } => {
            thresholds.iter().any(|threshold| rule.params.contains_key(threshold.name))
        }
        _ => false,
    }
}

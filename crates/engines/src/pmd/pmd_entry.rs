//! `PMD`: the PMD engine as an entry.

use super::{parse_pmd, pmd_refusal, pmd_version, run_pmd, unknown_ruleset};
use crate::entry::{Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// PMD's suppression marker, set to a text no real code carries, so a `// NOPMD` comment hides nothing: the rule file
/// is the only home for an exception. PMD has no switch that turns the marker off, only this option that renames it.
const NO_NOPMD: &str = "--suppress-marker=TM-NOPMD-OFF";

/// `pmd check --no-cache --no-progress --format=sarif --suppress-marker=TM-NOPMD-OFF --rulesets=<check>
/// --file-list=<list>` run by `run_pmd` in the rule file's folder, its SARIF log on stdout read by `parse_pmd`. The
/// list is a temporary file naming, one per line, the `.java` files under the folder the rule applies to (tracked, then
/// untracked and not ignored; never a symlink: `java_files`); a path holding `,`, which PMD reads there as a
/// separator (fixtures/fixtures.txt), makes the rule `Failed` before running (`run_pmd_with`). PMD reads Java only
/// here: its other languages (Apex, Kotlin, JavaScript and more) are out, since the check is refused for them and the
/// file list holds only `.java` files.
///
/// The check picks PMD's rules: one rule reference `category/java/<category>.xml/<Rule>`, a whole category
/// `category/java/<category>.xml`, or a comma list of them. Anything else is refused before running (`pmd_refusal`):
/// a ruleset file or URL, since the rule file is the only home for a rule and its exceptions (a ruleset file can carry
/// exclude patterns and `violationSuppressXPath`/`Regex`; a URL would reach the network), another language, an empty
/// part. PMD's `[WARN] No rules found in ruleset <part>` (exit 0) or `[ERROR] Cannot load ruleset <part>:` (exit 1)
/// for a part of the check makes the rule not available (`unknown_ruleset`). PMD takes no params: any param makes the
/// rule `Failed` before running. A rule's config never reaches PMD, since a ruleset file it named would carry
/// exceptions outside the rule file.
///
/// Suppressions: `// NOPMD` is off through `NO_NOPMD`; `@SuppressWarnings` with `"PMD"`, `"PMD.<Rule>"`, `"all"` or
/// `"unused"` in the code is honoured (seen on UnusedLocalVariable: fixtures/fixtures.txt) and PMD cannot be told to
/// ignore it; a ruleset's exclude patterns cannot apply, since no ruleset file is read.
///
/// Exit codes: 0 no violation, 4 violations (an answer, not an error), 5 a file PMD could not process (the rule is
/// `Failed`: such a file must never read clean), 1 or any other an error. With no `.java` file the rule applies to,
/// the rule is `NoFiles` before availability is looked at (`engine_listed`), so it never reads clean with nothing
/// checked. `run_pmd` removes CLASSPATH from the environment, as PMD's launcher puts it ahead of PMD's own rulesets;
/// `--no-cache` writes no analysis cache, no `--aux-classpath` is given, and nothing reaches the network. PMD finds its
/// own Java; none need be on PATH. PMD rules never share a run: `shares_with` has no arm for it.
pub static PMD: Entry = Entry {
    name: "pmd",
    engine: Engine::Pmd,
    program: "pmd",
    needs: &["pmd"],
    never: None,
    args: &["check", "--no-cache", "--no-progress", "--format=sarif", NO_NOPMD],
    pick: Pick::Arg("--rulesets="),
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: pmd_version, wrap: None }),
    refuse: pmd_refusal,
    unknown: unknown_ruleset,
    read: parse_pmd,
    run: run_pmd,
};

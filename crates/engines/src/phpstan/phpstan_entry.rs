//! `PHPSTAN`: the PHPStan engine as an entry.

use super::{parse_phpstan, phpstan_refusal, run_phpstan, unknown_level};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `phpstan analyse --no-progress --no-interaction --no-ansi --error-format=json --level=<check>
/// --configuration=<wrapper> -- <files>`, run in the rule file's folder by its own flow, `run_phpstan`, and read by
/// `parse_phpstan`. PHP must be on PATH as well, since the phpstan launcher starts with `#!/usr/bin/env php`.
///
/// The check is a PHPStan level, `0` to `10` or `max`: the rule reports every finding PHPStan gives at that
/// level, narrowed by the rule's paths and the rule file's exceptions. A level the installed PHPStan lacks is not
/// available, in PHPStan's own words (`unknown_level`); a check that is neither digits nor `max` is `Failed` before
/// anything runs (`phpstan_refusal`).
///
/// The config is the repository's own: the rule's config, else the first file in the folder of those PHPStan itself
/// discovers, in its order (`repo_config`); with none the rule is not available. PHPStan is pointed at a wrapper
/// config cctao writes in a temporary folder (`phpstan_config`), which includes the repository's, puts `tmpDir` beside
/// it, and empties `ignoreErrors`, which drops the config's entries and any baseline neon's, and `excludePaths`, so no
/// exclusion in the config drops a file passed. The wrapper, its folder and the cache PHPStan writes there
/// are removed after the run.
///
/// What still keeps a finding from the report outside the rule file: inline `@phpstan-ignore`, `@phpstan-ignore-line`
/// and `@phpstan-ignore-next-line` comments, since no option turns them off. `reportUnmatchedIgnoredErrors`
/// (on by default) adds a finding for an inline ignore that matches nothing; the wrapper leaves it to the repository's
/// config, since it silences nothing: it only reports more. A config's `stubFiles`
/// naming a passed file drops that file from analysis, so its findings are not reported (when it drops every file
/// passed, the rule is `NoFiles`). A service the repository's config tags `phpstan.ignoreErrorExtension` can drop
/// errors. The wrapper cannot empty either setting without dropping PHPStan's own stubs and services, so cctao leaves
/// both as stated limits, not `Failed`.
///
/// Only the rule's in-scope `.php` files are passed (`source_files` with `is_php_source`); PHPStan sees the rest of the code only as the
/// config's own bootstrap and scan settings let it. With no such file the rule is `NoFiles`, settled before
/// availability (`engine_listed`, in `engine_hits_with`'s step 1). A run whose config's `stubFiles` named every file
/// passed is `NoFiles` too, found after the run from PHPStan's words (`no_php_analysed`, `run_phpstan` step 7).
/// PHPStan exits 0 when clean and 1 both for findings and for failures, so cctao reads the JSON, not the exit code, and
/// a top-level error in it (a crashed worker) is `Failed`. Nothing reaches the network: no composer, no update check;
/// its parallel workers talk to the main process over loopback only. PHPStan takes no params, and its rules never
/// share a run: `shares_with` has no arm for it.
pub static PHPSTAN: Entry = Entry {
    name: "phpstan",
    engine: Engine::Phpstan,
    program: "phpstan",
    needs: &["phpstan", "php"],
    never: None,
    args: &["analyse", "--no-progress", "--no-interaction", "--no-ansi", "--error-format=json"],
    pick: Pick::Arg("--level="),
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: phpstan_refusal,
    unknown: unknown_level,
    read: parse_phpstan,
    run: run_phpstan,
};

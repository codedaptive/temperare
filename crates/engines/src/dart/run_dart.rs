//! `run_dart`: dart's flow for one rule. It reads the disk and runs git and the engine, so its test is the zone
//! integration test tests/dart.rs and it has no unit-test sibling.

use super::{
    kind_of, lint_enabled, missing_packages, options_files, options_suppression, probe_files, pubspec_root, CheckKind,
    ProbeDir,
};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, Entry};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::repo_top::repo_top;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `DART`. The central `refused` has already
/// settled the rule's refusals and params (dart takes none), and `engine_hits_with`'s `engine_listed` a folder with no
/// Dart source. Each step can stop the rule:
/// 1. the package is the nearest `pubspec.yaml` at or above `folder` and no higher than the repository top
///    (`repo_top`) (`pubspec_root`); none is `NotAvailable` naming pubspec.yaml, and a `folder` in no repository is
///    `Failed` with ccvcslib's error;
/// 2. the analysis options that govern `folder`, each with what it includes (`options_files`), which stops the rule
///    on an options file read outside the repository top that no `package:` include reached, an include it cannot
///    resolve or read, a folder it cannot list, or options text the YAML parser cannot read (`yaml_entries`);
/// 3. each chain's `options_suppression`: a key `exclude` or a value `ignore` in the options is `Failed` naming the
///    file and the line, since dart would hide findings cctao cannot get back;
/// 4. the probe: `entry_cmd` (no params, no config) run through `host.run` in a new `ProbeDir` holding `probe_files`,
///    not counted, a run error read as `ran` reads it; `kind_of` reads it as a lint, a diagnostic, or an
///    `UnknownCheck`;
/// 5. for a lint, every chain must enable it (`lint_enabled`); else `NotAvailable` naming the first governing file
///    that does not, or saying no analysis_options.yaml governs, and the lint: dart runs a lint only where
///    analysis_options.yaml enables it, so a lint it does not run would read as clean;
/// 6. the check run of `entry_cmd` in `folder`, through `check_run` so it is counted;
/// 7. `missing_packages`, given whether `<package>/.dart_tool/package_config.json` is a file, is `NotAvailable` when
///    dart could not resolve a `package:` URI because the dependencies were never fetched;
/// 8. otherwise the output read by `entry.read` against `folder`; output it cannot read is `parse_failed`'s `Failed`.
pub(crate) fn run_dart(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let top = repo_top(folder)?;
    let root = pubspec_root(folder, &top).ok_or_else(|| Stop::NotAvailable {
        why: "no pubspec.yaml at or above the rule file's folder: dart analyze checks a Dart package, and cctao runs it only in one".into(),
    })?;
    let chains = options_files(folder, &top, &root)?;
    if let Some(stop) = chains.iter().find_map(|chain| options_suppression(chain)) {
        return Err(stop);
    }
    if probed(entry, &rule.check, host)? == CheckKind::Lint {
        if let Some(stop) = lint_stop(&chains, &rule.check) {
            return Err(stop);
        }
    }
    let out = check_run(&entry_cmd(entry, OsStr::new(&rule.check), &[], None, folder), host)?;
    if let Some(stop) = missing_packages(&out, root.join(".dart_tool").join("package_config.json").is_file()) {
        return Err(stop);
    }
    (entry.read)(&out, &rule.check, folder, folder).map_err(|err| parse_failed(&out, err))
}

/// The kind of `check`, from the probe run in a new `ProbeDir`, removed once the probe has run.
fn probed(entry: &Entry, check: &str, host: &Host) -> Result<CheckKind, Stop> {
    let probe = ProbeDir::new(&probe_files(check)).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot make a folder for dart's probe: {err}"),
    })?;
    kind_of(&ran(&entry_cmd(entry, OsStr::new(""), &[], None, &probe.path), host)?, check)
}

/// `NotAvailable` when `lint` is not enabled by every chain of `chains`: naming the governing file (a chain's last)
/// of the first that does not, or saying none governs when there is no chain.
fn lint_stop(chains: &[Vec<(PathBuf, String)>], lint: &str) -> Option<Stop> {
    let place = match chains.iter().find(|chain| !lint_enabled(chain, lint)) {
        Some(chain) => chain.last().map_or(String::new(), |(file, _)| format!("{} does not enable", file.display())),
        None if chains.is_empty() => "no analysis_options.yaml governs the rule's folder, so nothing enables".into(),
        None => return None,
    };
    Some(Stop::NotAvailable {
        why: format!("{place} the lint `{lint}`: dart runs a lint only where analysis_options.yaml enables it"),
    })
}

//! `threshold_run`: what a rule's threshold adds to its check run, and the temporary file or folder it needs.

use super::{param_why, threshold_of, Entry, Params, Reach, TempDir, TempFile, ThresholdRun};
use rules::Rule;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::Path;

/// For a rule that sets one of its `Params::Thresholds` entry's thresholds (`threshold_of`) to `n`: the arguments,
/// in order, `{entry.config}{rule.config}` when both are given; for `Reach::File` with no rule config, the engine's
/// own config `{option}{folder/own}` when that file is in `folder`; then the threshold: `Reach::Arg(flag)` one
/// argument `{flag}{setting(n)}`, `Reach::File` the setting in a `TempFile` passed as `{option}{path}`, `Reach::Dir`
/// the setting in a `TempDir` whose path is set in env `env`. `None` for a rule that sets no threshold, and for every
/// other `Params` kind. The caller passes `args` after the param arguments and gives `entry_cmd` no config: the
/// rule's config is already in `args`, ahead of the threshold, so the threshold wins over a key the config also sets.
/// The error is the stderr of the rule's `Failed`: `param_why`'s text for a threshold `threshold_of` refuses, and
/// `cannot write the <name> threshold config: <error>` when the file or folder cannot be written.
pub(crate) fn threshold_run(entry: &Entry, rule: &Rule, folder: &Path) -> Result<Option<ThresholdRun>, String> {
    let Params::Thresholds { reach, .. } = entry.params else { return Ok(None) };
    let found = threshold_of(entry, &rule.check, &rule.params).map_err(|err| param_why(entry.name, err))?;
    let Some((threshold, n)) = found else { return Ok(None) };
    let setting = (threshold.setting)(n);
    let cannot = |err: io::Error| format!("cannot write the {} threshold config: {err}", entry.name);
    let mut run = ThresholdRun { args: Vec::new(), env: Vec::new(), _file: None, _dir: None };
    if let (Some(option), Some(config)) = (entry.config, &rule.config) {
        run.args.push(joined(option, config));
    }
    match reach {
        Reach::Arg(flag) => run.args.push(joined(flag, &setting)),
        Reach::File { option, ext, own } => {
            let own = own.map(|own| folder.join(own)).filter(|own| rule.config.is_none() && own.is_file());
            run.args.extend(own.map(|own| joined(option, own)));
            let file = TempFile::write(entry.name, ext, &setting).map_err(cannot)?;
            run.args.push(joined(option, &file.path));
            run._file = Some(file);
        }
        Reach::Dir { env, file } => {
            let dir = TempDir::write(entry.name, file, &setting).map_err(cannot)?;
            run.env.push((env.into(), dir.path.clone().into()));
            run._dir = Some(dir);
        }
    }
    Ok(Some(run))
}

/// `option` and `value` as one argument.
fn joined(option: &str, value: impl AsRef<OsStr>) -> OsString {
    let mut arg = OsString::from(option);
    arg.push(value);
    arg
}

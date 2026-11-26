//! `YAMLLINT`: the yamllint engine as an entry.

use super::{no_such_rule, parse_yamllint, run_yamllint, yamllint_refusal};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `yamllint -f parsable -d <data> -- <files>`, run in the rule file's folder by its own flow, `run_yamllint`, and read
/// by `parse_yamllint`.
///
/// The check is one yamllint rule id (`line-length`, `truthy`): the rule reports that rule's findings, at `error` or
/// `warning` level alike, narrowed by the rule's paths and the rule file's exceptions. A check of any other shape (a
/// capital, a comma, a space) is `Failed` before anything runs (`yamllint_refusal`), since it is written into the config
/// text. An id the installed yamllint lacks is not available, in yamllint's own words (`no_such_rule`).
///
/// The config is the rule's, else the nearest `.yamllint`, `.yamllint.yaml` or `.yamllint.yml` from the rule's folder
/// up to the repository top (`yamllint_config`), else yamllint's `default`. tm hands yamllint, with `-d`, a config that
/// extends it and turns the check on (`config_data`); `-d` replaces yamllint's own discovery, so the user-global config
/// and the project config yamllint would find are never read. A rule config that is not a file is `Failed` naming it.
///
/// The files passed are the rule's in-scope YAML files (`source_files` with `is_yaml_source`): yamllint's stock
/// `yaml-files`, `*.yaml`, `*.yml` and `.yamllint`. A config's own `yaml-files` is not read: it steers only yamllint's
/// walk of a folder, and tm passes files, so a file it adds under another name is not checked (a stated limit).
///
/// tm reads none of the repository's YAML: yamllint reads its own config, so `extends` chains and `ignore-from-file`
/// files are yamllint's, and gitignore-style patterns in its own config are matched by yamllint; which files it is
/// given is git's (`source_files`). The `-d` text tm writes double-quotes the check key and the base path, since
/// yamllint reads it with PyYAML, YAML 1.1, where a bare `yes` or `on` is a boolean; the quoted key `"yes"` was measured
/// to reach yamllint as a string (fixtures/fixtures.txt).
///
/// What keeps a finding from the report outside the rule file: the config's top-level `ignore` and `ignore-from-file`,
/// and the check's own, are turned off by the `-d` text, so exceptions go in the rule file (S-67 (1)).
/// `# yamllint disable`, `disable-line` and `disable-file` comments are left on, since no option turns them off
/// (S-67 (2)).
///
/// Limits: yamllint runs in the rule's folder, so a relative `extends` or `ignore-from-file` in the config resolves from
/// there; a relative `extends` or `ignore-from-file` that does not resolve there is `Failed`, never clean (the base's own
/// `ignore-from-file` is still read, though the `-d` text's `ignore: []` replaces what it gives). A config above the
/// rule's folder whose relative `extends` names a path that also exists in the rule's folder loads that file as its
/// base, not the one beside the config, so its options apply instead.
///
/// yamllint exits 0 for nothing found or warnings only, 1 for an error-level problem and 255 for a config it refuses.
/// A missing extended config exits 1 with a Python traceback, the same code as findings, so tm reads the output, not
/// the code: any stderr is `Failed`. With no YAML file the rule applies to it is `NoFiles`, settled before availability
/// (`engine_listed`, in `engine_hits_with`'s step 1). Nothing reaches the network: yamllint fetches nothing. yamllint
/// takes no params, and its rules never share a run: `shares_with` has no arm for it.
pub static YAMLLINT: Entry = Entry {
    name: "yamllint",
    engine: Engine::Yamllint,
    program: "yamllint",
    needs: &["yamllint"],
    never: None,
    args: &["-f", "parsable"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: yamllint_refusal,
    unknown: no_such_rule,
    read: parse_yamllint,
    run: run_yamllint,
};

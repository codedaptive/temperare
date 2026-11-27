//! `SQLFLUFF`: the sqlfluff engine as an entry.

use super::{parse_sqlfluff, run_sqlfluff, sqlfluff_refusal, unknown_rule};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// sqlfluff run by its own flow, `run_sqlfluff`, with the command `sqlfluff_cmd` builds:
/// `sqlfluff lint --format json --nocolor --disable-progress-bar --disable-noqa --disregard-sqlfluffignores
/// --exclude-rules= --ignore= --rules=<check> [--config=<config>] -- <files>`, run in the rule file's folder on the SQL
/// files in the rule's scope (`source_files` with `is_sql_source`) and read by `parse_sqlfluff`.
///
/// The SQL files are those matching sqlfluff's stock `sql_file_exts` (`.sql`, `.sql.j2`, `.dml`, `.ddl`, `.pkb`, any
/// case). A stated limit: a repository whose config adds an extension has those files left out. A repository whose
/// config removes one has those files skipped by sqlfluff, which reads `Failed` through `unreported_files`, never clean.
///
/// The check is one sqlfluff rule reference: a code (`LT01`), a name (`layout.spacing`), a group (`layout`) or an
/// alias (`L039`); `sqlfluff_refusal` refuses any other shape before anything runs. It reaches sqlfluff as `--rules=`,
/// so sqlfluff runs that rule alone. sqlfluff lints with no rule and exits 0 for a reference it does not know (case
/// matters: `lt01` is unknown), so its stderr warning "Tried to allowlist unknown rule references" naming the check is
/// `UnknownCheck` (`unknown_rule`). The rule's config is passed as `--config=`, sqlfluff's additional config file,
/// layered on top of the ones it finds.
///
/// The environment: `HOME` is the repository's top. sqlfluff reads its user and `~` configs and walks folder configs
/// from the common path of `~` and the file down to the file's folder, so every config it reads is the repository's
/// own and none above the top is read (measured); `XDG_CONFIG_HOME`, `PYTHONPATH` and `PYTHONUSERBASE` are removed.
/// Nothing reaches the network: sqlfluff fetches nothing and installs no plugin. The templater is the repository's,
/// one of sqlfluff's own (`raw`, `jinja`, `python`, `placeholder`); dbt is a plugin sqlfluff does not ship; where it is
/// not installed, a config naming it is `Failed` with sqlfluff's stderr (exit 2; it says to `pip install
/// sqlfluff-templater-dbt`; the "Requested templater 'dbt' ..." line is on stdout). The dialect comes from the
/// repository's own config: cctao passes no `--dialect`, and none set is `NotAvailable` with a why naming `.sqlfluff`
/// (`no_dialect`), as a missing project file is.
///
/// Suppressions: (1) turned off: `-- noqa` comments (`--disable-noqa`), `.sqlfluffignore`
/// (`--disregard-sqlfluffignores`), and a config's `exclude_rules` and `ignore` (overridden empty, in nested folder
/// configs too, so a parse error a config ignores is still reported); a violation a config lists under `warnings` is
/// still a finding. (2) Left on: inline `-- sqlfluff:` file directives such as `-- sqlfluff:exclude_rules:` and
/// `-- sqlfluff:rules:`, which sqlfluff applies after every command-line option with no switch to stop them. They are
/// a gap for the next standard version: no standard@1 suppression rule reports them.
///
/// No SQL file in the rule's scope is `NoFiles`, settled before availability (`engine_listed`). Exit 0 (clean, or only
/// config warnings) and 1 (violations) are answers; 2 is a user error and anything else a crash, `Failed`. A file
/// sqlfluff could not parse, template or lex is `Failed`, as is a file it was handed and did not report on (over
/// `large_file_skip_byte_limit`, say: `unreported_files`). A sqlfluff rule takes no params and shares no run with
/// another rule.
pub static SQLFLUFF: Entry = Entry {
    name: "sqlfluff",
    engine: Engine::Sqlfluff,
    program: "sqlfluff",
    needs: &["sqlfluff"],
    never: None,
    args: &[
        "lint",
        "--format",
        "json",
        "--nocolor",
        "--disable-progress-bar",
        "--disable-noqa",
        "--disregard-sqlfluffignores",
        "--exclude-rules=",
        "--ignore=",
    ],
    pick: Pick::Arg("--rules="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: sqlfluff_refusal,
    unknown: unknown_rule,
    read: parse_sqlfluff,
    run: run_sqlfluff,
};

# The rule file

`temperare.toml` holds every rule a repository enforces and every exception to one. It is the only place a rule or
an exception lives: neither is ever a comment in the code.

## Where it is

- `tm` reads `temperare.toml` at the top of the git repository that holds the current folder.
- `--rules <path>` names another file, anywhere. A relative path is taken from the current folder.
- Every path inside the file (a rule's `paths`, a kit's `paths`, an exception's `path`, an ast-grep `check`, a
  `config`) is relative to the folder the rule file is in. The engines check that folder, and every finding's path
  in a report is relative to it.

## A whole file

```toml
version = 1
owner = "platform"

[kits.app]
paths = ["app/**"]
owner = "app-team"

[kits.deploy]
paths = ["deploy/**"]
owner = "deploy-team"

[[rule]]
id = "no-bool-compare"
engine = "clippy"
check = "clippy::bool_comparison"
fix = "Use the bool itself: `if x`, not `if x == true`."

[[rule]]
id = "clippy-warnings"
engine = "clippy"
check = "warnings"
severity = "warning"
fix = "Fix what clippy names."
params = { allow = ["clippy::module_inception"] }

[[rule]]
id = "no-force-cast"
engine = "swiftlint"
check = "force_cast"
config = ".swiftlint.yml"
paths = ["Sources/**/*.swift"]
exclude = ["**/Generated/**"]
fix = "Use `as?` and handle nil."

[[rule]]
id = "no-hardcoded-password"
engine = "ruff"
check = "S105"
kind = "security"
fix = "Read the secret from the environment or the keychain."

[[rule]]
id = "no-stored-bool"
engine = "ast-grep"
check = "rules/no-stored-bool.yml"
paths = ["**/*.swift", "**/*.rs"]
kits = ["app"]
fix = "Store a bitmap field; expose the Bool as a computed accessor."
params = { entity_types = ["Order", "Invoice"] }

[[rule]]
id = "git-only-in-deploy"
engine = "ast-grep"
check = "rules/git-only-in-deploy.yml"
paths = ["**/*.rs"]
outside_kits = ["deploy"]
fix = "Run git through the deploy kit; only deploy starts git."

[[exception]]
rule = "no-force-cast"
path = "Sources/Legacy/Bridge.swift"
what = "Force casts"
reason = "the bridge is rewritten in 2.0"
approved = "who approved it"
```

## The top level

| Key | Required | Meaning |
|---|---|---|
| `version` | yes | The integer `1` |
| `owner` | when any rule has `kind = "security"` | Who owns a finding in a file no kit holds. A blank string counts as none |
| `[kits.<name>]` | no | A named set of paths |
| `[[rule]]` | no | One rule; as many as you need |
| `[[exception]]` | no | One approved exception; as many as you need |

Any other key is a problem.

## A kit: `[kits.<name>]`

| Key | Meaning |
|---|---|
| `paths` | Globs. The kit holds every file that matches one. A kit with no paths holds nothing |
| `owner` | Who owns a finding in this kit's files. A blank string counts as none |

A kit does two jobs. A rule can be limited to kits (`kits`) or kept out of them (`outside_kits`), and a kit can name
the owner of the findings in its files.

## A rule: `[[rule]]`

| Key | Required | Meaning |
|---|---|---|
| `id` | yes | The rule's name in every report. One or more of `a-z`, `0-9` and `-`. No two rules share an id |
| `engine` | yes | `clippy`, `swiftlint`, `ruff`, `ast-grep` or `tessera`, exactly as written here |
| `check` | yes | What the engine is asked. See below |
| `fix` | yes | The text shown with each finding of this rule |
| `severity` | no | `error` (the default) or `warning` |
| `kind` | no | `quality` (the default) or `security` |
| `paths` | no | Globs. The rule covers files that match one. Absent or empty: every file |
| `exclude` | no | Globs. The rule never covers a file that matches one |
| `kits` | no | Kit names. The rule covers only files held by one of these kits |
| `outside_kits` | no | Kit names. The rule never covers a file held by one of these kits |
| `config` | no | The engine's own config file. Passed to `swiftlint` and `ruff` as written; the other engines do not take one |
| `params` | no | A table of values for the engine. See below |

Any other key is a problem.

### `check`

| Engine | `check` is |
|---|---|
| `clippy` | One lint the toolchain lists, such as `clippy::bool_comparison` (`-` and `_` are read alike), or `warnings` for every warning `cargo clippy` reports with a lint code. Not a lint group such as `clippy::pedantic`, and not a `rustdoc::` lint |
| `swiftlint` | One SwiftLint rule identifier, such as `force_cast`. For a custom rule, its own identifier, not `custom_rules` |
| `ruff` | A rule code (`E711`), a code prefix (`E7`, `F`), `ALL`, or several of these joined by commas with no spaces (`E711,F401`). Not a rule name |
| `ast-grep` | The path of an ast-grep rule file (YAML) inside the rule file's folder. Every match of that file is a finding |
| `tessera` | Required like any other, and not used today: nothing is run |

A finding is kept only when it can be matched back to `check`: for clippy and SwiftLint the engine reports exactly
that lint or identifier, and for ruff the diagnostic's code starts with one of the parts of `check`.

These are checked when the rule runs, not when the rule file is read. A `check` that cannot be matched (a clippy
lint group or a name the toolchain does not list, `custom_rules`, a ruff rule name, an ast-grep file outside the
rule file's folder) makes the rule "failed", exit 5, with the reason in the report. It never reads as clean, and
`tm rules` does not report it. [`ENGINES.md`](ENGINES.md) has each case.

### `params`

- **clippy:** `allow`, a list of lints left out, such as `{ allow = ["clippy::module_inception"] }`. It is the only
  param a clippy rule reads; any other name makes the rule fail, so a misspelt `allow` cannot pass unnoticed.
- **ast-grep:** names for the rule file's placeholders. Each `{{name}}` in the YAML is replaced by the param of that
  name before ast-grep runs. A string goes in as written. A list of strings goes in as its items joined by `|`, each
  escaped so it matches literally, which drops into a `regex:` line as "any of these". A `{{name}}` with no param of
  that name makes the rule fail.
- **swiftlint, ruff, tessera:** not read.

### Which files a rule covers

A rule covers a file when all of these hold:

1. the file matches one of `paths`, or `paths` is absent;
2. the file matches none of `exclude`;
3. the file is in none of the `outside_kits` kits;
4. `kits` is absent, or the file is in one of those kits.

An engine runs over the rule file's whole folder. Findings in files the rule does not cover are dropped, and so are
findings outside the rule file's folder.

### Globs

- `*` stays within one folder: `*.py` matches `app.py` and not `src/app.py`.
- `**` crosses folders: `**/*.py` matches both.
- A glob is matched against the file's path relative to the rule file's folder, with `/` between its parts.

## An exception: `[[exception]]`

| Key | Required | Meaning |
|---|---|---|
| `rule` | yes | The `id` of a rule in this file |
| `path` | yes | One file, exactly, relative to the rule file's folder. Not a glob |
| `reason` | yes | Why the exception stands |
| `approved` | yes | Who approved it |
| `what` | no | Limits the exception to findings whose message contains this text. Leave it out to cover every finding of the rule in the file; an empty `what` is a problem |

An exception has no line number, because line numbers drift. A finding is covered by the first exception in the
file with its rule and its path whose `what`, when given, is in the finding's message.

A covered finding is silenced: it is not among the findings, it does not change the exit code, and it is counted.
`--json` lists it under `silenced` with the exception that covered it. Every report also gives the number of
exceptions in force, which is every `[[exception]]` in the file, whether or not one was used in this run.

`approved` is text. Temperare records it and does not check who wrote it, so review a change to `temperare.toml` as
you review code.

## Who owns a finding

Each finding carries an `owner` when the rule file gives one for its file:

1. Take the first kit in the file, in the order the kits are written, that holds the finding's file. If that kit
   has an `owner`, that is the owner.
2. Otherwise the owner is the top-level `owner`. This is also the answer when the first kit that holds the file
   has no owner of its own, even if a later kit that holds it has one.
3. With no top-level `owner` either, the finding has no owner.

A rule file with a `security` rule must have a top-level `owner`, so a security finding always names one.

## Problems

`tm` checks the whole file before it runs anything, and reports every problem it finds at once, in file order:

```
/path/to/temperare.toml:3:1  [[rule]] is missing `fix`  Add `fix = "..."` to this [[rule]].
/path/to/temperare.toml:4:6  malformed id "No_None": an id is one or more of a-z, 0-9 and '-'  Write the id in lower-case letters, digits and dashes, e.g. "no-stored-bool".
/path/to/temperare.toml:5:10  unknown engine "pylint"  Use one of: clippy, swiftlint, ruff, ast-grep, tessera.
next: fix each problem listed for /path/to/temperare.toml
```

Each line is `file:line:col`, what is wrong, and what to write instead. With any problem, no engine runs and the
exit code is 2. `tm rules` reports the same problems, so it is the quick way to check a rule file.

What is checked:

- the TOML syntax (a syntax error is reported alone, since nothing after it can be read);
- unknown keys at the top level, in a kit, in a rule and in an exception;
- `version` missing or not `1`;
- a value of the wrong type, such as `paths` that is not a list of strings or `params` that is not a table;
- a rule missing `id`, `engine`, `check` or `fix`;
- an `id` that is malformed or used twice;
- an `engine`, `kind` or `severity` that is not one of the names above;
- a glob that does not compile;
- a kit named in `kits` or `outside_kits` that is not defined under `[kits]`;
- a `security` rule in a file with no top-level `owner`;
- a `check`, `config` or `params.allow` value that begins with `-`, since it would reach the engine as an option;
- an exception missing `rule`, `path`, `reason` or `approved`, naming a rule that is not in the file, or with an
  empty `what`;
- a rule file that cannot be read, reported at line 0, column 0.

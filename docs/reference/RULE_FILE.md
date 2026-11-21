# The rule file

`temperare.toml` holds every rule a repository enforces and every exception to one. It is the only place a rule or
an exception lives: neither is ever a comment in the code. A rule is written there, or comes from a baseline set
built into `tm` that the file takes ([Baseline sets](#baseline-sets)).

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
| `baseline` | no | The baseline sets the file takes, a list of `"name@version"` strings such as `["standard@1"]` ([Baseline sets](#baseline-sets)) |
| `[[override]]` | no | One change to one rule of a taken set; at most one per rule ([`[[override]]`](#a-change-to-a-sets-rule-override)) |
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
| `id` | yes | The rule's name in every report. One or more of `a-z`, `0-9` and `-`. No two rules share an id. It may not start with `tm-`, which is kept for the rules of baseline sets |
| `engine` | yes | `clippy`, `swiftlint`, `ruff`, `ast-grep`, `shellcheck`, `git`, `tessera`, `golangci-lint`, `pylint`, `clang-tidy` or `oclint`, exactly as written here. The last four are accepted, and their rules are "not available" until Temperare has an adapter for each ([`ENGINES.md`](ENGINES.md#engines-with-no-adapter-yet)). `shellcheck` and `git` have adapters, and their rules are not available in this build for other reasons ([`ENGINES.md`](ENGINES.md#shellcheck)) |
| `check` | yes | What the engine is asked. See below |
| `fix` | yes | The text shown with each finding of this rule |
| `severity` | no | `error` (the default) or `warning` |
| `kind` | no | `quality` (the default) or `security` |
| `paths` | no | Globs. The rule covers files that match one. Absent or empty: every file |
| `exclude` | no | Globs. The rule never covers a file that matches one |
| `kits` | no | Kit names. The rule covers only files held by one of these kits |
| `outside_kits` | no | Kit names. The rule never covers a file held by one of these kits |
| `config` | no | The engine's own config file. Passed to `swiftlint` and `ruff` as written, and it is a ShellCheck rule's `--rcfile` ([`ENGINES.md`](ENGINES.md#shellcheck)); the other engines do not take one |
| `params` | no | A table of values for the engine. See below |
| `only_with_base` | no | `true` or `false` (the default). With `true` the rule reports only what a change adds, so it needs `--base`. Without `--base` the rule reports nothing, not even a failure, and is listed under `not_applicable` ([`OUTPUT.md`](OUTPUT.md#new-findings-only---base)) |
| `enforces` | no | Text: the practice the rule enforces. Read and checked to be a string; no report prints it |
| `leaves` | no | Text: what the rule's check does not cover. Read and checked to be a string; no report prints it |

Any other key is a problem.

### `check`

| Engine | `check` is |
|---|---|
| `clippy` | One lint the toolchain lists, such as `clippy::bool_comparison` (`-` and `_` are read alike), or `warnings` for every warning `cargo clippy` reports with a lint code. Not a lint group such as `clippy::pedantic`, and not a `rustdoc::` lint |
| `swiftlint` | One SwiftLint rule identifier, such as `force_cast`. For a custom rule, its own identifier, not `custom_rules` |
| `ruff` | A rule code (`E711`), a code prefix (`E7`, `F`), `ALL`, or several of these joined by commas with no spaces (`E711,F401`). Not a rule name |
| `ast-grep` | The path of an ast-grep rule file (YAML) inside the rule file's folder. Every match of that file is a finding |
| `shellcheck` | One ShellCheck code, `SC` then digits, such as `SC2086`. Not a list of codes |
| `git` | `test_ratio`, the only check git has |
| `tessera` | Required like any other, and not used today: nothing is run |
| `golangci-lint`, `pylint`, `clang-tidy`, `oclint` | Required like any other, and not used yet: nothing is run |

A finding is kept only when it can be matched back to `check`: for clippy and SwiftLint the engine reports exactly
that lint or identifier, and for ruff the diagnostic's code starts with one of the parts of `check`.

These are checked when the rule runs, not when the rule file is read. A `check` that cannot be matched (a clippy
lint group, `custom_rules`, a ruff rule name, an ast-grep file outside the rule file's folder, a ShellCheck `check`
that is not one code, a git `check` other than `test_ratio`) makes the rule
"failed", exit 5, with the reason in the report. A `check` the installed clippy, SwiftLint or ruff does not know
(a misspelt name, or one newer than that engine) makes the rule "not available", exit 3, with a reason naming the
engine, its version and the check. Neither reads as clean, and `tm rules` reports neither, since it runs no
engine. [`ENGINES.md`](ENGINES.md) has each case.

### `params`

- **clippy:** `allow`, a list of lints left out, such as `{ allow = ["clippy::module_inception"] }`; and `max_nesting`, a
  whole number from 1 to 1000, read only with check `clippy::excessive_nesting`. Any other name makes the rule fail,
  so a misspelt `allow` cannot pass unnoticed.
- **ast-grep:** names for the rule file's placeholders. Each `{{name}}` in the YAML is replaced by the param of that
  name before ast-grep runs. A string goes in as written. A list of strings goes in as its items joined by `|`, each
  escaped so it matches literally, which drops into a `regex:` line as "any of these". A `{{name}}` with no param of
  that name makes the rule fail, and so does a param the YAML never names as `{{name}}`, so a misspelt param name
  cannot pass unnoticed. Two names are not placeholders: `count`, whose only value is `"setup-vs-tests"`, and
  `shared_setup`, a list of globs that needs `count`. They turn on count mode, which is not available in this build
  ([`ENGINES.md`](ENGINES.md#ast-grep)); a bad value for either fails the rule.
- **swiftlint:** `max_complexity`, a whole number from 1 to 1000, read only with check `cyclomatic_complexity`; and
  `max_lines`, a whole number from 1 to 100000, read only with check `file_length`. At most one threshold per rule;
  any other param fails the rule.
- **ruff:** `max_complexity`, a whole number from 1 to 1000, read only with check `C901`; any other param fails the
  rule.
- **shellcheck:** none; any param fails the rule.
- **git:** `max_ratio`, a number from 0.1 to 100; `floor`, a whole number from 0 to 100000; and `test_paths`, a list
  of globs with at least one item. All three are required; a missing one, a value out of range and any other param
  fail the rule ([`ENGINES.md`](ENGINES.md#git)).
- **tessera:** none; any param fails the rule.
- **golangci-lint, pylint, clang-tidy, oclint:** not read.

### Which files a rule covers

A rule covers a file when all of these hold:

1. the file matches one of `paths`, or `paths` is absent or empty;
2. the file matches none of `exclude`;
3. the file is in none of the `outside_kits` kits;
4. `kits` is absent or empty, or the file is in one of those kits.

An engine runs over the rule file's whole folder. Findings in files the rule does not cover are dropped, and so are
findings outside the rule file's folder.

A rule that covers none of the files git lists in the rule file's folder (the tracked files in the work tree and
the untracked files git does not ignore) runs nothing, and the report does not say so: the exit code is what it
would be without that rule ([`ENGINES.md`](ENGINES.md#a-rule-that-covers-no-file)).

### Globs

- `*` stays within one folder: `*.py` matches `app.py` and not `src/app.py`.
- `**` crosses folders: `**/*.py` matches both.
- A glob is matched against the file's path relative to the rule file's folder, with `/` between its parts.

## Baseline sets

A baseline set is a list of rules built into `tm`, with a name and a version. A rule file takes a set with the
top-level `baseline` key, and the set's rules are checked beside its own:

```toml
version = 1
baseline = ["standard@1"]

[[override]]
rule = "tm-python-complexity"
params = { max_complexity = 15 }

[[override]]
rule = "tm-python-print"
off = true
```

Each entry is a set's name, `@` and a whole-number version. A set is taken once, at one version. `tm` carries two:

| Set | Rules | State |
|---|---|---|
| `standard@1` | 53, every one `kind = "quality"` and `severity = "warning"` | Ready |
| `house@1` | 36, 30 at `warning` and 6 at `error`, all `kind = "quality"` | Not ready: a rule file that takes it is refused (below) |

A rule file that takes `house@1` has this problem, at the entry, and nothing runs (exit 2):

```
/path/to/temperare.toml:2:13  house@1 is not ready in this build: its rules are not yet proven  remove house@1 from baseline
```

### The rules of `standard@1`

In the set's order:

| Family | Rules | Engine |
|---|---|---|
| Dead code: an unused item, import or variable | 5: Rust 3, Python 2 | clippy, ruff |
| Deprecated calls | 1: Rust | clippy |
| Suppressions written in the code | 11: one per language for Rust, Swift, Python, JavaScript, TypeScript, Go, C and C++; then a Rust `allow` with no reason, a SwiftLint disable left on for the rest of the file, and a Python `noqa` with no code | ast-grep for the first 8; then clippy, swiftlint, ruff |
| The rest: complexity (Swift, Python; `max_complexity = 10`), nesting (Rust; `max_nesting = 6`), `dbg!` (Rust), and in Python `print`, a bare `except`, a blind `except` and commented-out code | 8 | swiftlint, ruff, clippy |
| What a change adds: a sleep, a retry or a skipped test (Rust, Swift, Python, Go, JavaScript, TypeScript), a weaker assertion or a wider catch (the same without Go) | 28, each with `only_with_base = true` | ast-grep |

- Every rule has `paths` for its language's files. A rule for a language the repository does not have covers no
  file and runs nothing ([Which files a rule covers](#which-files-a-rule-covers)).
- The 28 rules of the last family report only what a change adds: without `--base` they report nothing and are
  listed under `not_applicable`.
- A set's ast-grep rules carry their YAML inside `tm`. Their `check` is not a path in your repository
  ([`ENGINES.md`](ENGINES.md#ast-grep)).

The full list: `tm rules` on a rule file that takes the set lists its rules, and each set's own rule file is in the
repository at `crates/rules/baseline/<name>/<version>/set.toml`, with its ast-grep YAML under `ast-grep/` and its
cases under `cases/` beside it.

### How a set's rules join the file's

- The rules of the taken sets come first, in `baseline` order and then each set's own order. The file's own rules
  follow. This is the order `tm rules` lists them in, and the "rule-file order" of a report.
- Every rule of a set has an id that starts with `tm-`, and a rule of the file's own may not, so the two never share
  an id. To change a set's rule, write an `[[override]]`; a second `[[rule]]` cannot replace it.
- An `[[exception]]` may name a rule of a taken set, also one an override turned off.
- Once a set version is released it is frozen: a change to the set is a new version, such as `standard@2`.

### A change to a set's rule: `[[override]]`

| Key | Required | Meaning |
|---|---|---|
| `rule` | yes | The `id` of a rule in a set the file takes |
| `severity` | no | `error` or `warning`, in place of the set's |
| `off` | no | `true` or `false` (the default). With `true` the rule is left out: it is not run, and neither `tm rules` nor a report lists it |
| `paths` | no | Globs, in place of the rule's `paths` |
| `exclude` | no | Globs, in place of the rule's `exclude` |
| `params` | no | A table laid over the rule's `params` key by key: a key given here wins, and the set's other keys stand |

Any other key is a problem, so an override cannot change a rule's `engine`, `check`, `kind`, `config` or `fix`, and
cannot limit it to kits. Each rule takes at most one `[[override]]`.

### Required params

A set's rule can name params the repository must give. The rule file gives each one in the `params` of an
`[[override]]` for that rule; a value in the set's own `params` does not count. Until every one is given the rule
is not applicable: it is not run, `tm rules` does not list it, and `tm check` lists it under `not_applicable` with
the why `required params not given: <names>; give them in the params of an [[override]] for "<rule id>"`.

No rule of `standard@1` requires a param. Five rules of `house@1` do: four require `entity_types` and one
`own_package`.

### Keys only a set's own file has

| Key | Where | Meaning |
|---|---|---|
| `required_params` | a set's `[[rule]]` | The names of the params the repository must give, as above |
| `failing`, `passing` | a set's `[[rule]]` | The rule's two cases, which `tm baseline test` runs ([`OUTPUT.md`](OUTPUT.md#tm-baseline-test)). `failing = { files = [...], path = "...", line = N }` lists the case's files and the one finding it must give; `passing = { files = [...] }` lists files that must give none. Every rule of a set has both |
| `retired` | a set's top level | Rule ids the set has retired. An id is never reused: a retired id is not a rule of the set, nor of another set taken with it. Neither set `tm` carries retires an id |

In a repository's rule file each of these is an unknown key, which is a problem. `enforces`, `leaves` and
`only_with_base` are keys of any rule, a set's or your own.

### Problems of `baseline` and `[[override]]`

| Problem | When |
|---|---|
| `malformed baseline entry "<entry>"` | The entry is not a name, `@` and a whole number |
| `unknown baseline set "<name>"` | `tm` carries no set of that name. The fix names them: `Use one of: house, standard.` |
| `unknown version <n> of baseline set "<name>"` | `tm` carries the set, and not at that version. The fix names the versions it has |
| `baseline set "<name>" is taken twice` | A second entry for a set an earlier entry took, at any version |
| `<name>@<version> is not ready in this build: its rules are not yet proven` | The set is built in and not ready: `house@1`. The fix is `remove <name>@<version> from baseline` |
| `rule "<id>" is in both <set> and <set>` | Two taken sets hold a rule of that id. Reported at the later entry |
| `baseline set <name>@<version> has a problem at <line>:<col>: <what>` | A fault in the set built into `tm`, not in your file. The fix asks you to report it |
| ``[[override]] is missing `rule` `` | An `[[override]]` with no `rule` |
| `override of unknown baseline rule "<id>": no taken set holds it` | `rule` names no rule of a taken set: a misspelt id, one of the file's own rules, or a file with no `baseline` |
| `second [[override]] for "<id>"` | A rule has more than one `[[override]]` |
| `id "<id>" starts with "tm-", which is kept for baseline rules` | A `[[rule]]` of the file's own whose id starts with `tm-` |

An entry gets the first of the first five that applies, in that order. A `baseline` that is not a list of strings,
an unknown key in an `[[override]]`, a value of the wrong type, a glob that does not compile and a `params.allow`
value that begins with `-` are problems as they are in a `[[rule]]`.

## An exception: `[[exception]]`

| Key | Required | Meaning |
|---|---|---|
| `rule` | yes | The `id` of a rule in this file, or of a rule in a baseline set the file takes |
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
/path/to/temperare.toml:5:10  unknown engine "pylnit"  Use one of: clippy, swiftlint, ruff, ast-grep, tessera, golangci-lint, pylint, clang-tidy, oclint, shellcheck, git.
next: fix each problem listed for /path/to/temperare.toml
```

Each line is `file:line:col`, what is wrong, and what to write instead. With any problem, no engine runs and the
exit code is 2. `tm rules` reports the same problems, so it is the quick way to check a rule file.

What is checked:

- the TOML syntax (a syntax error is reported alone, since nothing after it can be read);
- unknown keys at the top level, in a kit, in a rule, in an override and in an exception;
- `version` missing or not `1`;
- a value of the wrong type, such as `paths` that is not a list of strings or `params` that is not a table;
- a rule missing `id`, `engine`, `check` or `fix`;
- an `id` that is malformed, used twice, or starts with `tm-`;
- a `baseline` entry and an `[[override]]`, as listed under
  [Problems of `baseline` and `[[override]]`](#problems-of-baseline-and-override);
- an `engine`, `kind` or `severity` that is not one of the names above;
- a glob that does not compile;
- a kit named in `kits` or `outside_kits` that is not defined under `[kits]`;
- a `security` rule in a file with no top-level `owner`;
- a `check`, `config` or `params.allow` value that begins with `-`, since it would reach the engine as an option;
- an exception missing `rule`, `path`, `reason` or `approved`, naming a rule that is neither in the file nor in a
  taken set, or with an empty `what`;
- a rule file that cannot be read, reported at line 0, column 0.

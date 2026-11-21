# Engines

An engine is a program Temperare runs to do the reading. Temperare parses no source code, links no engine, and
reads only what the engine prints. One read of the code is its own: for a ShellCheck rule it reads the first line of a
file with no extension, to tell a shell script by its `#!` line ([shellcheck](#shellcheck)). This page says exactly
what is run for each one, what `check` means to it, and what is "not available" and what is "failed".

## What holds for every engine

- **Found on `PATH`.** An engine is available when the programs it needs are on the `PATH` that `tm` was started
  with. `tm rules` shows the answer per rule without running anything.
- **Run as a program, never through a shell.** Each argument is passed as one argument, as written.
- **A rule's value is passed as a value, not as an option.** A `check`, `config` or `params.allow` value that
  begins with `-` is a problem in the rule file, and nothing runs. Each value reaches the engine joined to its own
  flag, as in `--select=<check>`, so the engine reads it as that flag's value.
- **One run per rule, or one shared run.** Rules share one run of their engine only when that run gives each of
  them what it would get alone: the same engine, the same `config` and the same `params`, no param Temperare writes
  to a temporary file or folder, and the same check or checks the engine reads together. When the two checks
  differ: for clippy, neither check is `warnings`. For ruff, the codes do not overlap, neither selects `RUF100`, and no pair ruff
  calls incompatible is split. For SwiftLint, neither check is `superfluous_disable_command`. For ast-grep, the
  same rule file with no params. ShellCheck, git, Tessera, and an engine with no adapter yet, never share. Each rule still keeps only its own check's
  findings, in its own paths and with its own exceptions. Every other rule runs alone. A rule that is refused, or
  that covers no file ([below](#the-order-a-rule-settles-in)), runs nothing and joins no shared run.
- **The order of runs.** A rule joins the first group, in rule-file order, whose rules can all share with it. The
  groups run one after another, in the order of each group's first rule, and the rule runs' results are kept in
  rule-file order; the report sorts findings as OUTPUT.md describes.
- **Run over the rule file's folder.** The engine checks the whole folder (clippy: the Cargo package that holds
  it). Temperare then keeps the findings that are inside that folder, in the files the rule covers (its `paths`,
  `exclude`, `kits`, `outside_kits`) and, when you gave paths on the command line, in those paths. Paths on the
  command line narrow the result; they do not make the engines do less work.
- **Its version is read once per check.** Before the rules run, each engine named by the rules this check runs is
  asked its version once, in the rule file's folder, when it is available: `cargo clippy --version`,
  `swiftlint version`, `ruff --version`, `ast-grep --version`, `shellcheck --version`, `git --version`. Tessera, and
  an engine with no adapter yet, is not asked. An engine is asked even when every rule that names it is refused or
  covers no file. The version is named in the reason of a check the engine does not know, and of a ShellCheck code
  that is not on `tm`'s list (below).
- **No time limit.** `tm` waits for an engine as long as it runs.
- **The engine's environment is yours.** Temperare adds nothing to it, except that for a clippy rule with `max_nesting` it
  sets `CLIPPY_CONF_DIR` for that run to a temporary folder holding `clippy.toml`, which clippy reads instead of the
  repository's own `clippy.toml`.
- **The engine finds its own configuration,** as it does when you run it yourself, except where the rule says
  otherwise. A rule's `config` is passed to SwiftLint or ruff as its config file. A SwiftLint threshold passes a
  config file of its own, after the folder's `.swiftlint.yml` when there is one and the rule has no `config` (see
  [swiftlint](#swiftlint)). A clippy `max_nesting` replaces the repository's `clippy.toml` for that run, as above.
  A ruff threshold is layered over the rule's `config`, or over the config ruff finds when the rule has none, and
  turns nothing off.
- **The engine's exit code is not the answer.** Linters exit non-zero when they find something. Temperare reads the
  output; a non-zero exit with output it can read is a normal result.
- **The rule decides severity.** A finding's `severity`, `kind` and `fix` come from the rule in `temperare.toml`.
  The engine's own severity for it is not read.

## Not available, and failed

Neither is ever "clean".

**Not available** (exit 3) means the rule could not be run at all:

| `why` | When |
|---|---|
| `<program> not found on PATH` | The engine's program is not installed, or not on `PATH`. A clippy rule names the first of `cargo-clippy` and `clippy-driver` that is missing. A ShellCheck rule names `git` when ShellCheck is there and git, which lists its scripts, is not |
| `no Cargo.toml at or above the rule file's folder` | A clippy rule in a folder that is not inside a Cargo package |
| ``<engine> <version> does not know the check `<check>`: <what it said>`` | The installed clippy, SwiftLint or ruff does not know the rule's `check`: a name it does not have, often because it is older than the rule. `<version>` is `(version unknown)` when the version could not be read |
| `shellcheck <version>: <check> is not in temperare's list of shellcheck codes (taken from shellcheck <list version>)` | A ShellCheck rule whose code is not on the list built into `tm`. In this build the list holds no code and its version reads `pending`, so this is every ShellCheck rule that has a script to check and finds ShellCheck ([below](#shellcheck)) |
| `reports only what a change adds; run with --base` | A git rule, and an ast-grep rule in count mode, with or without `--base` in this build ([git](#git), [ast-grep](#ast-grep)) |
| `Tessera's slop records are not defined yet` | A rule with `engine = "tessera"` ([below](#tessera)) |
| `no adapter for <engine> in this build` | A rule whose engine the rule file accepts and this `tm` cannot run yet: `golangci-lint`, `pylint`, `clang-tidy` and `oclint` ([below](#engines-with-no-adapter-yet)) |

A rule that is refused, or that covers no file, is settled before any of these is looked at
([the order a rule settles in](#the-order-a-rule-settles-in)).

An engine too old for a rule is not available rather than failed: the fix is a newer engine, or a `check` this one
knows. ast-grep, git and Tessera have no such case: an ast-grep `check` is your own YAML, which ast-grep either reads
or rejects. ShellCheck is not asked whether it knows a code: the code is looked up in `tm`'s own list.

**Failed** (exit 5) means the rule gave no answer that can be trusted. Either the engine gave none: it was found
and could not be started, it printed something that is not its findings format, or a clippy build did not succeed.
Or Temperare refused the rule before running the engine, because the result could not be matched back to the rule
or the rule's own files could not be used:

- a clippy `check` that is a lint group or a `rustdoc::` lint;
- a SwiftLint `check` of `custom_rules`;
- a ruff `check` with a part that is not a rule code, a code prefix or `ALL`;
- an ast-grep `check` that is not a readable file inside the rule file's folder;
- a ShellCheck `check` that is not one code;
- a git `check` other than `test_ratio`;
- `params` that cannot be used.

A rule refused before its engine ran has no exit code. The reason is printed under the counts line of the text
output (`stderr of <rule> (<engine>):`) and carried in `failed[].stderr_tail` of the JSON output: the last 20 lines
of the engine's stderr, or Temperare's own reason when the engine printed none or was not run.

### The order a rule settles in

Each rule is settled by the first of these that applies:

1. **Refused.** `tm` refuses the rule from the rule file alone, reading no other file and running nothing: a clippy
   `rustdoc::` lint, SwiftLint's `custom_rules`, a ruff `check` with a part that is not a code, a ShellCheck `check`
   that is not one code, a git `check` other than `test_ratio`, an ast-grep `check` that is an absolute path, a
   param the engine does not take, a param with a bad value, or a threshold out of range or with another check
   (ast-grep's placeholder params apart: see below). The rule is failed (exit 5).
2. **No file.** The rule covers none of the files git lists in the rule file's folder, or it is a ShellCheck rule
   and covers none of the scripts there. The rule runs nothing ([below](#a-rule-that-covers-no-file)).
3. **Not available.** The engine's program is not on `PATH`, the engine has no adapter in this build, or it is
   Tessera.
4. **The run.** What the engine's run comes to: findings, none, not available or failed.

So a refused rule is failed (exit 5) whether or not its engine is installed, and installing the engine would not help it. Its engine is still asked its version, as above. The one exception is an `only_with_base` rule run without `--base`, which is listed under `not_applicable` instead, as any such rule is.

Three refusals need more than the rule file, so they are made in the run, at step 4, after steps 2 and 3:

- a clippy lint group, which is told from a lint by `clippy-driver -W help`;
- an ast-grep rule file that is missing, cannot be read or lies outside the rule file's folder;
- an ast-grep placeholder param that cannot be used (a `{{name}}` with no param, a param with no `{{name}}`, a
  param that is neither a string nor a list of strings), which needs the YAML read.

Such a rule fails when its engine is installed and it covers a file. With the engine missing it is not available,
and when it covers no file it runs nothing.

Outside a git repository a ShellCheck rule that finds ShellCheck fails on git's listing of its scripts, before its
code is looked up in `tm`'s list ([shellcheck](#shellcheck)).

### A rule that covers no file

Before any rule runs, `tm` asks git for the files of the rule file's folder: the tracked files that are in the work
tree, and the untracked files git does not ignore. A rule that covers none of them, by its `paths`, `exclude`,
`kits` and `outside_kits`, runs nothing. Its engine is not run and not looked for on `PATH`. A ShellCheck rule
that covers a file is settled the same way when it covers none of the scripts in the folder.

These are not among those files, so a rule that covers only such files covers no file: a file git ignores, a
tracked file deleted from the work tree, a skip-worktree file whether or not it is there, and the files of an
untracked nested repository.

The report does not show such a rule. It has no line in the text output, it is in no list of the JSON output, it
adds to no count, and it does not change the exit code. A refused rule is not such a rule: it is settled first and is failed, whatever it covers. (One exception: a rule with `only_with_base = true`, run
without `--base`, is listed as not applicable whatever it covers, as any such rule is.) A check whose only rule
covers no file prints
`0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed, 0 not applicable`
and `next: nothing to do`, and exits 0. (The example line shows no exceptions in force; a rule file with exceptions shows their count.) That is so whether its engine is installed or not. The one difference is
the engines line: an engine found on `PATH` is still asked its version and listed, and a missing one is not. In
`--json`, `saved.counted` holds the rule with `chars` 0.

So exit 0 does not say every rule had a file to check. `tm rules` shows each rule's scope.

When git cannot list the files (the rule file's folder is in no git repository, or git is missing or fails), no
rule is settled this way, and each goes on to steps 3 and 4.

## clippy

- **Available when** `cargo-clippy` and `clippy-driver` are both on `PATH`. clippy is run through `cargo`, so
  `cargo` must be there too.
- **Where:** the nearest folder at or above the rule file's folder that holds a `Cargo.toml`. For a rule file inside
  one member of a workspace, that is the member, so only that crate is linted.
- **What runs,** all three in that folder:

  ```
  clippy-driver -W help
  cargo locate-project --workspace --message-format plain
  cargo clippy --all-targets --message-format=json --quiet -- --warn=<check> [--allow=<lint>]...
  ```

  The first lists the lints and lint groups of the toolchain in use; `check` is looked up in that list before
  anything is built. The second names the workspace root, which is where cargo prints file paths from.
  `--all-targets` lints tests, examples and benches too, so test code is checked like any other. `--warn=<check>`
  turns the lint on even if it is off by default. Each lint in `params.allow` becomes `--allow=<lint>` after it, so
  an allowed lint stays off even under `warnings`.
- **`check`:**
  - one lint that `clippy-driver -W help` lists, such as `clippy::bool_comparison` or the compiler's own
    `unused_variables`. `-` and `_` are read alike, so `clippy::bool-comparison` is the same lint. Messages with
    that code are the findings, at any level;
  - or `warnings`: every warning-level message that has a lint code is a finding, the compiler's own lints
    included. The finding's message starts with the code, such as
    `clippy::bool_comparison: equality checks against true are unnecessary`, so it names the lint.
- **Failed before clippy runs** when `check` is any of these, so a rule that could match nothing never reads as
  clean:
  - a lint group, such as `clippy::all`, `clippy::pedantic` or `unused`. clippy reports each message under its own
    lint's code, so a group's findings cannot be told from other warnings. Name each lint, or use `warnings`;
  - a `rustdoc::` lint, which clippy never runs.

  A name the toolchain does not list (a misspelt lint, one renamed or removed, or one newer than this toolchain)
  makes the rule not available before clippy runs, with the reason
  ``clippy <version> does not know the check `<check>`: it is not a lint this toolchain lists (clippy-driver -W help)``.
  A rule's own `allow` cannot hide it.

  The rule also fails when `clippy-driver -W help` cannot be read or lists no lint or no group, and when clippy
  itself reports that it did not take `check` as named.
- **`params`:** `allow`, a list of strings; and `max_nesting`, a whole number from 1 to 1000, read only with check
  `clippy::excessive_nesting` (tm writes it as `excessive-nesting-threshold` in `clippy.toml` for that run). Any other
  param, or a bad value, fails the rule before clippy runs.
- **`config`:** not used.
- **A finding** is placed at the message's primary span. A message cargo prints once per target is reported once.
- **Failed when** the build does not succeed. A build that failed has not linted everything, so it is not an
  answer.
- **It builds your code.** `cargo clippy` compiles the crate and what it depends on. Temperare sets no target
  folder, so cargo builds where it normally does, the repository's own `target/`. The first run takes as long as a
  build, and a build runs the crate's build scripts and procedural macros.

## swiftlint

- **Available when** `swiftlint` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  swiftlint lint --reporter json --quiet --only-rule=<check> [--config=<config>] [--config=<threshold file>]
  ```
- **`check`:** one SwiftLint rule identifier, such as `force_cast`. Violations with exactly that identifier are the
  findings. For a custom rule, write the custom rule's own identifier.
- **Failed** when `check` is `custom_rules`, before SwiftLint runs: under `--only-rule` that name runs no custom
  rule, so the rule could match nothing. Failed too when SwiftLint finds no Swift file to lint in the folder (it
  exits with an error instead of an empty result).
- **Not available** when SwiftLint says `check` is not a valid rule identifier: it does not know that rule, with the
  rule's `config` applied, so a custom rule the config defines is known. The reason quotes SwiftLint's own warning
  line, which names the identifier. SwiftLint exits 0 with no findings in that case, so its warning is the only sign
  that nothing was checked.
- **Paths under `/private` on macOS.** SwiftLint prints file paths with a leading `/private` removed; a finding is
  placed under the rule file's folder either way, so code under `/tmp` and `/var` is checked as anywhere else. A
  path that cannot be placed under the folder at all makes the rule Failed, never clean.
- **`config`:** passed as written, so a relative path is taken from the rule file's folder. Temperare does not read
  or copy the file.
- **`params`:** `max_complexity`, a whole number from 1 to 1000, read only with check `cyclomatic_complexity`; and
  `max_lines`, a whole number from 1 to 100000, read only with check `file_length`. At most one threshold per rule.
  The threshold is written to a temporary `.yml` file that sets the check's warning and error levels both to it, and
  that file is passed as the last `--config=`, so it wins over the rule's `config`. A rule with a threshold and no
  `config` passes the folder's own `.swiftlint.yml` first, when there is one, since an explicit `--config` would
  otherwise make SwiftLint skip it. No config of yours is changed, and the temporary file is removed after the run.
  A threshold out of range or with another check, and any other param, fail the rule before SwiftLint runs.
- **A finding** has no column when SwiftLint gives none, as for a violation of a whole line or file.

## ruff

- **Available when** `ruff` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  ruff check --no-cache --output-format json --select=<check> [--config=<config>] [--config=lint.mccabe.max-complexity=<n>] .
  ```

  `--no-cache` keeps ruff from leaving a `.ruff_cache` folder in the code it checks.
- **`check`:** a ruff selector made of rule codes:
  - a rule code, such as `E711`;
  - a code prefix, such as `E7` or `F`, for every rule whose code starts with it;
  - `ALL`, for every rule;
  - or several of these with commas and no spaces between them, such as `E711,F401`.

  A diagnostic is a finding when its code starts with one of the parts, or when a part is `ALL`.
- **Failed** when a part of `check` is anything else, such as a rule name (`none-comparison`) or a lower-case code,
  before ruff runs: Temperare could not tell which findings it selects. Failed too when ruff says a code `check`
  selects has been remapped to another, since its findings would carry the new code. Write the new code.
- **Not available** when ruff says a part of `check` is an unknown rule selector: a code this ruff does not have,
  often one newer than it. The reason quotes ruff's own line, which names the code.
- **`config`:** passed as written, so a relative path is taken from the rule file's folder. Temperare does not read
  or copy the file.
- **`params`:** `max_complexity`, a whole number from 1 to 1000, read only with check `C901`, passed as
  `--config=lint.mccabe.max-complexity=<n>` after the rule's `config`. ruff lays that over the config file, so the
  threshold wins over the same key set there, and nothing is written. A threshold out of range or with another check,
  and any other param, fail the rule before ruff runs.

## ast-grep

ast-grep is for rules about the shape of code that no stock linter states: a house rule.

- **Available when** `ast-grep` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  ast-grep scan --rule <a filled copy of your rule file> --json=stream
  ```
- **`check`:** the path of an ast-grep rule file (YAML), relative to the rule file's folder. Every match of that
  file is a finding. One file can hold several rules, separated by `---`, for example one per language.
- **The YAML must be a file inside the rule file's folder.** The rule fails, before the scan runs, when `check`:
  - is an absolute path;
  - resolves to a place outside the rule file's folder, through `..` or through a symlink;
  - names a file that is missing or cannot be read.

  A symlink whose target is inside the rule file's folder is resolved by these checks, so it is read.

  On Unix more checks hold. The file is opened one path component at a time from `/` down through the rule file's
  folder, following no symlink, so the file read is the one inside the folder: if a component has become a symlink
  or something that is not a folder since the path was resolved, or the file is removed while it is read, the rule
  fails with `changed while it was opened`. A folder on that walk that cannot be opened (one you may pass through
  but not read) fails the rule, naming that folder. A YAML path that is not a regular file, such as a FIFO or a
  folder, fails the rule without waiting on it. And a file with more than one hard link fails the rule, wherever
  the other link is, because Temperare cannot tell where a file's other names are. On other systems only the path
  checks above apply.
- **`params`:** the values for the placeholders in the YAML. Temperare reads your rule file, replaces each
  `{{name}}` with the param of that name, writes the result to a temporary file, runs ast-grep with it and removes
  it:
  - a string param goes in as written;
  - a list of strings goes in as its items joined by `|`, each escaped to match literally, ready for a `regex:`
    line.

  The rule fails, before the scan runs, when the YAML has a `{{name}}` with no param of that name, when a param
  is neither a string nor a list of strings, or when a param has no `{{name}}` in the YAML, so a misspelt param
  name cannot pass unnoticed. The reason for the last names the YAML's placeholders: with params `entity_type` and
  `entity_types` and a YAML that names only `{{entity_types}}`, it is
  ``param entity_type in rules/no-stored-bool.yml: the rule YAML has no {{entity_type}}; its placeholders are {{entity_types}}``.
- **Count mode: `count` and `shared_setup`.** These two params are not placeholders; they are taken out before the
  YAML is filled. `count = "setup-vs-tests"`, its only value, turns count mode on. `shared_setup`, a list of globs
  relative to the rule file's folder, names files that are never reported, and needs `count`. Any other `count`, a
  `shared_setup` without `count`, one that is not a list of strings, or a glob in it that does not compile, fails
  the rule before anything runs, whether or not ast-grep is installed.

  In count mode the YAML matches each file's root node and each test function, and a match is not a finding. A file
  the change adds lines to is reported when its non-blank lines outside the test functions outnumber those inside
  them: one finding per file, at its first added line, with no column and the message
  `<n> lines of setup, <m> lines of tests`. A file with no test function is not reported.

  Count mode needs the base commit, and in this build `tm check` does not hand the base to the engines. So a
  count-mode rule that gets as far as its run is not available, with the why
  `reports only what a change adds; run with --base`, with or without `--base`, and ast-grep is not run for it.
  With `only_with_base = true` and no `--base` the rule is listed as not applicable instead, as any such rule is.
- **A YAML that ast-grep rejects** fails the rule. ast-grep has no "not available" case for an unknown `check`: the
  check is your own YAML, which ast-grep either reads or rejects.
- **`config`:** not used.
- **A finding's message** is the `message` of the ast-grep rule that matched, or that rule's `id` when it has no
  message.

A house rule with a list of names:

```toml
[[rule]]
id = "no-stored-bool"
engine = "ast-grep"
check = "rules/no-stored-bool.yml"
paths = ["**/*.rs"]
fix = "Store a bitmap field; expose the Bool as a computed accessor."
params = { entity_types = ["Order", "Invoice"] }
```

`rules/no-stored-bool.yml`:

```yaml
id: no-stored-bool-rust
language: rust
message: stored Bool on a persisted entity
severity: error
rule:
  kind: field_declaration
  has:
    field: type
    regex: ^bool$
  inside:
    kind: struct_item
    stopBy: end
    has:
      field: name
      regex: ^({{entity_types}})$
```

Before ast-grep runs, `{{entity_types}}` becomes `Order|Invoice`. A `bool` field on `Order` or `Invoice` is a
finding; one on any other struct is not.

"Named only inside one kit" is an ast-grep rule that matches the name, with `outside_kits` naming the kit that may
use it:

```toml
[kits.deploy]
paths = ["deploy/**"]

[[rule]]
id = "git-only-in-deploy"
engine = "ast-grep"
check = "rules/git-only-in-deploy.yml"
paths = ["**/*.rs"]
outside_kits = ["deploy"]
fix = "Run git through the deploy kit; only deploy starts git."
```

`rules/git-only-in-deploy.yml`:

```yaml
id: git-only-in-deploy
language: rust
message: runs git outside the deploy kit
severity: error
rule:
  pattern: Command::new("git")
```

## shellcheck

- **Available when** `shellcheck` is on `PATH`. `tm` lists the scripts with git, so git must be there too.
- **In this build no ShellCheck rule reaches ShellCheck.** A rule's code is looked up in a list of ShellCheck's
  codes that is built into `tm`, and the list in this build holds no code; its version reads `pending`. Inside a
  git repository a ShellCheck rule therefore comes to one of these, in the order
  [a rule settles in](#the-order-a-rule-settles-in):
  - failed, when its `check` is not one code or it has a param;
  - nothing run, when it covers no script;
  - not available, with `git not found on PATH`, when git is not installed and so cannot list the scripts; or failed, with git's own reason, when git cannot list them;
  - not available, with `shellcheck not found on PATH`;
  - not available, with
    ``shellcheck <version>: <check> is not in temperare's list of shellcheck codes (taken from shellcheck pending)``.
    `<version>` is the installed ShellCheck's, or `(version unknown)` when it could not be read; the last word is
    the list's version.

  `tm rules` runs nothing, so it shows a ShellCheck rule as available whenever `shellcheck` is on `PATH`.
- **Where:** the rule file's folder, which must be inside a git repository.
- **`check`:** one ShellCheck code, `SC` then digits, such as `SC2086`. Anything else (`sc2086`, `2086`, two codes)
  fails the rule before anything runs, with the reason
  ``shellcheck check `<check>` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects``.
  A well-formed code that is not on the list makes the rule not available, as above; nothing runs for it, so a code
  `tm` does not list is never clean.
- **`params`:** none. Any param fails the rule before anything runs, with the reason
  `shellcheck param <name>: a shellcheck rule takes no params`.
- **Which files it is handed.** `tm` picks the scripts itself, among the files git lists under the rule file's
  folder: the tracked files, then the untracked files git does not ignore. A file is a script when
  - its extension is `sh`, `bash`, `ksh` or `dash`; or
  - it has no extension and its first line is a `#!` line that runs `sh`, `bash`, `dash` or `ksh`, named directly
    (`#!/bin/sh`) or through `env` (`#!/usr/bin/env bash`; env's own flags are skipped). `tm` reads at most the
    file's first 257 bytes for this, and a first line longer than 256 bytes is not read as a shebang. Another shell,
    such as zsh or fish, does not count.

  A file with any other extension is never a script, whatever its first line. Never handed over:
  - a symlink, and any file under a symlinked folder, whatever it points at;
  - anything that is not a regular file, and a file with no extension that cannot be read;
  - a tracked file deleted from the work tree;
  - a skip-worktree file, whether or not it is there.

  A script with an unresolved merge conflict is listed once, as it is in the work tree. The shebang is read on Unix
  only: on another system a file with no extension is never a script.
- **No script, nothing run.** A rule that covers none of those scripts runs nothing, whether or not ShellCheck is
  installed and whether or not its code is on the list ([above](#a-rule-that-covers-no-file)).
- **Outside a git repository** the scripts cannot be listed. A rule that finds ShellCheck fails, with no exit code
  and git's reason (`<folder>: not a git repository`), before its code is looked up in the list. A rule that does
  not find ShellCheck is not available.
- **What is run for a code on the list,** in the rule file's folder:

  ```
  shellcheck -f json1 --enable=all --rcfile=<config> -- <script>...
  ```

  or, for a rule with no `config`, the same with `--norc` in place of `--rcfile=<config>`. Every script under the
  folder is handed over, and the rule's code is not passed: every check runs, `--enable=all` turning the optional
  ones on, and `tm` keeps the comments whose code is the rule's, in the files the rule covers. `--` ends the
  options, so a script whose name begins with `-` is read as a file; one named exactly `-` is passed as `./-`.
  `SHELLCHECK_OPTS` is removed from ShellCheck's environment for this run, since ShellCheck reads more options
  from it.
- **`config`:** passed as `--rcfile=<config>`, as written, so a relative path is taken from the rule file's folder.
  Temperare does not read or copy the file. Without a `config` the run gets `--norc`, so no `.shellcheckrc` in the
  code is read: a `disable=` there cannot silence a rule.
- **Failed,** for a code on the list, when ShellCheck exits with anything but 0 or 1; when it says it was
  `unable to read --rcfile`; when what it prints is not its `json1` output; and when it reports that it could not
  parse a script (an error-level comment with a code from `SC1000` to `SC1999`), unless the rule's own code is one
  of those.
- **A finding** is placed at the comment's line and column, and its message is ShellCheck's.
- **Runs alone.** A ShellCheck rule never shares a run, even with another ShellCheck rule.

## git

`git` as an engine counts what a change adds. It is separate from the git `tm` runs for `--base` and to list files,
which needs no rule.

- **Available when** `git` is on `PATH`.
- **In this build a git rule never comes to an answer.** It counts a change against a base commit, and `tm check`
  does not hand the base to the engines, with or without `--base`. A git rule that is not refused, covers a file
  and finds git is therefore not available, with the why `reports only what a change adds; run with --base`, and
  passing `--base` does not change that. With `only_with_base = true` and no `--base` the rule is listed as not
  applicable instead, as any such rule is.
- **`check`:** `test_ratio`, the only one. Any other fails the rule before anything runs, with the reason
  ``git check `<check>` is not one git counts: the git entry checks only `test_ratio` ``.
- **`params`,** all three required:
  - `max_ratio`, a number from 0.1 to 100: the most lines of test a change may add for each line of code;
  - `floor`, a whole number from 0 to 100000: a change that adds fewer lines of test than this is not reported;
  - `test_paths`, a list of globs with at least one item, relative to the rule file's folder: a file one of them
    matches counts as test, and any other file the rule covers counts as code.

  A missing param, a value out of range, a glob that does not compile, and any other param, fail the rule before
  anything runs.
- **What it counts, given a base:** the lines the change adds in the files the rule covers. For a tracked file
  those are the lines git shows as added against the base; an untracked file git does not ignore counts whole,
  unless it is binary, a symlink or not a regular file. A change that adds at least `floor` lines of test, and more
  than `max_ratio` lines of test for each line of code, gets one finding: at the first added line of the test file
  with the most added lines, with no column, and a message such as
  `this change adds 120 lines of test for 40 lines of code (3.0 to 1; the rule's limit is 2 past 50 lines)`.
- **`config`:** not used.
- **Runs alone.** A git rule never shares a run.

## tessera

A rule may name `tessera` as its engine. The rule file accepts it, and a Tessera rule that covers a file is reported
"not available" with the reason `Tessera's slop records are not defined yet`. This is so whether or not Tessera is
installed: `tm` neither runs it nor looks for it on `PATH`.

Two cases are settled first, as for any engine ([the order](#the-order-a-rule-settles-in)). A Tessera rule takes
no params: one with any param is failed (exit 5), with the reason
`tessera param <name>: a tessera rule takes no params`. And a Tessera rule that covers no file git lists runs
nothing, and the report does not say so ([above](#a-rule-that-covers-no-file)).

A run with a not-available Tessera rule ends with exit 3, unless a quality error (1) or a failed engine (5) comes
first. The `next:` line says `Tessera's checks are not available yet` and does not ask you to install it. To get an
exit 0, leave Tessera rules out of the rule file, or check with a rule file that has none
(`--rules <path>`).

A Tessera rule with `only_with_base = true`, run without `--base`, is not applicable rather than not available, as
any such rule is ([`OUTPUT.md`](OUTPUT.md#new-findings-only---base)).

## Engines with no adapter yet

A rule may name `golangci-lint`, `pylint`, `clang-tidy` or `oclint`. The rule file accepts
these names, so a rule written for one of them needs no change when its adapter lands. Until then this `tm` cannot run
them: a rule that covers a file is reported "not available" with the reason `no adapter for <engine> in this build`,
whether or not the engine is installed, and nothing is run. Its `check` and `params` are not read, so it is never
refused. `tm rules` shows the same reason. A rule with `only_with_base = true`, run without `--base`, is not
applicable instead, and a rule that covers no file git lists runs nothing, and the report does not say so
([above](#a-rule-that-covers-no-file)).

A run with a not-available rule of these engines ends with exit 3, unless a quality error (1) or a failed engine
(5) comes first. The `next:`
line says `<engine>: no adapter for <engine> in this build (or run without those rules)`; installing the engine
changes nothing.

## The engines version 0.1.0 was accepted with

The acceptance checks for 0.1.0 ran against clippy 0.1.95, SwiftLint 0.65.1, ruff 0.17.4 and ast-grep 0.51.1.
Temperare reads each engine's JSON output, so another version of an engine may print something it reads
differently. If a rule stops reporting after an engine upgrade, try it on a file you know breaks it.

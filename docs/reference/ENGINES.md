# Engines

An engine is a program Temperare runs to do the reading. Temperare parses no source code, links no engine, and
reads only what the engine prints. This page says exactly what is run for each one, what `check` means to it, and
what is "not available" and what is "failed".

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
  same rule file with no params. Tessera, and an engine with no adapter yet, never share. Each rule still keeps only its own check's
  findings, in its own paths and with its own exceptions. Every other rule runs alone.
- **The order of runs.** A rule joins the first group, in rule-file order, whose rules can all share with it. The
  groups run one after another, in the order of each group's first rule, and the rule runs' results are kept in
  rule-file order; the report sorts findings as OUTPUT.md describes.
- **Run over the rule file's folder.** The engine checks the whole folder (clippy: the Cargo package that holds
  it). Temperare then keeps the findings that are inside that folder, in the files the rule covers (its `paths`,
  `exclude`, `kits`, `outside_kits`) and, when you gave paths on the command line, in those paths. Paths on the
  command line narrow the result; they do not make the engines do less work.
- **Its version is read once per check.** Before the rules run, each engine named by the rules this check runs is
  asked its version once, in the rule file's folder, when it is available: `cargo clippy --version`,
  `swiftlint version`, `ruff --version`, `ast-grep --version`. Tessera, and an engine with no adapter yet, is not
  asked. The version is named in the reason of a check the engine does not know (below).
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
| `<program> not found on PATH` | The engine's program is not installed, or not on `PATH`. A clippy rule names the first of `cargo-clippy` and `clippy-driver` that is missing |
| `no Cargo.toml at or above the rule file's folder` | A clippy rule in a folder that is not inside a Cargo package |
| ``<engine> <version> does not know the check `<check>`: <what it said>`` | The installed clippy, SwiftLint or ruff does not know the rule's `check`: a name it does not have, often because it is older than the rule. `<version>` is `(version unknown)` when the version could not be read |
| `Tessera's slop records are not defined yet` | Every rule with `engine = "tessera"` |
| `no adapter for <engine> in this build` | Every rule whose engine the rule file accepts and this `tm` cannot run yet: `golangci-lint`, `pylint`, `clang-tidy`, `oclint`, `shellcheck` and `git` ([below](#engines-with-no-adapter-yet)) |

An engine too old for a rule is not available rather than failed: the fix is a newer engine, or a `check` this one
knows. ast-grep and Tessera have no such case: an ast-grep `check` is your own YAML, which ast-grep either reads or
rejects.

**Failed** (exit 5) means the rule gave no answer that can be trusted. Either the engine gave none: it was found
and could not be started, it printed something that is not its findings format, or a clippy build did not succeed.
Or Temperare refused the rule before running the engine, because the result could not be matched back to the rule
or the rule's own files could not be used:

- a clippy `check` that is a lint group or a `rustdoc::` lint;
- a SwiftLint `check` of `custom_rules`;
- a ruff `check` with a part that is not a rule code, a code prefix or `ALL`;
- an ast-grep `check` that is not a readable file inside the rule file's folder;
- `params` that cannot be used.

A rule refused before its engine ran has no exit code. The reason is printed under the counts line of the text
output (`stderr of <rule> (<engine>):`) and carried in `failed[].stderr_tail` of the JSON output: the last 20 lines
of the engine's stderr, or Temperare's own reason when the engine printed none or was not run.

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

## tessera

A rule may name `tessera` as its engine. The rule file accepts it, and the rule is always reported "not available"
with the reason `Tessera's slop records are not defined yet`. This is so whether or not Tessera is installed: `tm`
neither runs it nor looks for it on `PATH`.

A run with a Tessera rule therefore ends with exit 3, unless a quality error (1) or a failed engine (5) comes
first. The `next:` line says `Tessera's checks are not available yet` and does not ask you to install it. To get an
exit 0, leave Tessera rules out of the rule file, or check with a rule file that has none
(`--rules <path>`).

A Tessera rule with `only_with_base = true`, run without `--base`, is not applicable rather than not available, as
any such rule is ([`OUTPUT.md`](OUTPUT.md#new-findings-only---base)).

## Engines with no adapter yet

A rule may name `golangci-lint`, `pylint`, `clang-tidy`, `oclint`, `shellcheck` or `git`. The rule file accepts
these names, so a rule written for one of them needs no change when its adapter lands. `git` is the engine that
will count what a change adds against a base; `tm` already runs git itself for `--base`, and that is not this
engine. Until then this `tm` cannot run
them: the rule is reported "not available" with the reason `no adapter for <engine> in this build`, whether or
not the engine is installed, and nothing is run. `tm rules` shows the same reason. A rule with
`only_with_base = true`, run without `--base`, is not applicable instead.

A run with such a rule ends with exit 3, unless a quality error (1) or a failed engine (5) comes first. The `next:`
line says `<engine>: no adapter for <engine> in this build (or run without those rules)`; installing the engine
changes nothing.

## The engines version 0.2.0 was accepted with

The acceptance checks for 0.2.0 ran against clippy 0.1.95, SwiftLint 0.65.1, ruff 0.17.4 and ast-grep 0.51.1.
Temperare reads each engine's JSON output, so another version of an engine may print something it reads
differently. If a rule stops reporting after an engine upgrade, try it on a file you know breaks it.

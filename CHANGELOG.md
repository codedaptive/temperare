# Changelog

All notable changes to Code Chi Tao (called Temperare in versions up to 0.4.0) are recorded here, newest first.

## 0.5.0

Built from source; there are no release downloads yet. MIT licensed.

### Changed

- The program is now Code Chi Tao ("Tao"). The binary is `cctao` (it was `tm`, with `temperare` as its long name), and
  the rule file is `tao.toml` (it was `temperare.toml`). `cctao` does not fall back to `temperare.toml`: rename the
  file. State lives under `~/.local/state/cctao` and `~/.config/cctao`, and `cctao install` and `cctao uninstall` place
  and remove the `cctao` binary and skill. Programs that read `--json` output or run `tm` must be changed to run `cctao`.
- Every git question goes through the ccvcslib library (v0.2.1); the crate that ran programs no longer holds a git
  module.
- `expires` is required on every exception in the rule file, as `reason` and `approved` already were; it was optional.

### Added

- A telemetry file: each `cctao check` that ends in a report appends one line to
  `~/.local/state/cctao/telemetry/<root>.jsonl`, where `<root>` is the id of the commit HEAD's first-parent line starts from. cctao never reads it back, no output names it, and `cctao uninstall` leaves it. The line
  and its fields are in `docs/reference/OUTPUT.md`.
- The `watch` severity: a `watch` rule runs, but nothing it finds is reported (the telemetry file alone holds it) and it
  never sets an exit code. The savings estimate leaves `watch` rules out.
- `house@2`, a second version of the house baseline set, with a rule that refuses a citation of a register number, a
  fact id or a "(name, date)" label in code comments. A rule file takes `house@1` or `house@2`, not both.

## 0.4.0

Built from source; there are no release downloads yet. MIT licensed.

### Added

- Adapters for seven more engines, each proven by the acceptance checks against the real program: golangci-lint 2.15.0,
  pylint 4.1.4, clang-tidy 23.1.6, vulture 2.16, Periphery 3.8.0, gitleaks 8.30.1 and cargo-audit 0.23.0. Each is
  reported with its rule, place, fix and version, and is "not available" when its program is off `PATH`. Their limits
  are in `docs/reference/ENGINES.md`: vulture leaves git-ignored Python paths out of its scan, Periphery runs on Swift
  packages only and refuses anything it would fetch, gitleaks never prints a secret, and cargo-audit reads the local
  advisory database without fetching.
- Fourteen more engine names are accepted in a rule file: eslint, mypy, pmd, detekt, rubocop, phpstan, dart,
  markdownlint, yamllint, actionlint, hadolint, vale, stylelint and sqlfluff. They have no adapter yet, so a rule that
  names one is reported "not available" when it covers a file.
- `tm rules` lists the rules that do not apply (text, and `not_applicable` in `--json`), judged the way `tm check`
  judges them.

### Changed

- A finding's `line` is `null` in `--json` when the finding has none (cargo-audit's finding on `Cargo.lock`), where it
  was `0`; text output prints its path alone. A finding that has a line is unchanged. Programs that read `line` must
  accept `null`.
- `oclint` is no longer an engine name: a rule that names it is an unknown engine, like any other unknown name.
- A clang-tidy rule with no C-family source in its paths is judged to cover no file before the engine is looked up,
  as a ShellCheck rule with no script already was. A failed git listing stops only the judgement that needs the
  listing, not cargo-audit's check for a `Cargo.lock`.

## 0.3.0

Built from source; there are no release downloads yet. MIT licensed.

### Added

- Baseline sets: `standard@1` (53 rules) and `house@1` (36 rules) are ready, and the acceptance record has all 89 rules
  holding. A rule file takes a set with `baseline = [...]`; `house@1` may be taken alone or with `standard@1`, and its
  five param rules are not applicable until an `[[override]]` gives the params. See `docs/reference/RULE_FILE.md`.
- `tm baseline test [--set <name@version>] [--json]` runs each rule of a built-in set on the failing and passing cases
  the set carries, each case in a throwaway git repository. With no `--set` it tests the ready sets. Exits 0, 1, 2 or 3;
  see `docs/reference/OUTPUT.md`.
- `tm rules --loosened --base <rev> [--rules <path>] [--json]` lists how the rule file was loosened since a commit:
  a rule gone, a severity lowered, paths narrowed, an exception added or its date moved later, a set dropped.
- Exit 6: a rule of the repository's own covers no file. The report lists it as `covers nothing` (`covers_nothing` in
  `--json`) and counts it.
- Exceptions take an optional `expires` day. An exception still covers on that day and silences nothing after it; the
  report lists the expired ones (`expired` in `--json`) and counts them.
- Adapters for ShellCheck and for git (`test_ratio`), and an ast-grep count mode for rules that compare against a base.
  Four engine names still have no adapter: golangci-lint, pylint, clang-tidy and oclint.
- `procio::git_cmd` is public, for programs that run git the way Temperare does.

### Changed

- `tm check` hands `--base` to the engines, so a rule that needs a base runs against it; with no base given it is
  listed as "not applicable".
- An engine's "not available" carries its cause, and `tm check` advises an install only for an engine whose own
  program is missing from `PATH`.
- A rule that covers no file runs no check; its engine is still looked up and asked its version, except for a
  ShellCheck rule with no script, which stops before any lookup.
- The fix for an unknown set names the sets as `name@version`, ready ones only for a rule file.
- `exceptions_in_force` counts only exceptions not expired.

## 0.2.0

Built from source; there are no release downloads yet. MIT licensed.

### Added

- Tokens saved: a `tm check` that ran an engine opens its text with a `tokens saved` line, `--json` carries the
  numbers as `saved`, each such check appends one line to `~/.local/state/temperare/savings.jsonl`, and
  `savings_line = false` in `~/.config/temperare/config.toml` turns the line off. See `docs/reference/OUTPUT.md`.
- The report carries the day of the check (`date`), the findings each rule silenced (`silenced_by_rule`), the version
  of each engine that ran (`engines`) and the rules the check does not apply (`not_applicable`).
- Rules that would get the same answer from one run of their engine share that run, in a fixed order; see
  `docs/reference/ENGINES.md`.
- Threshold params, such as clippy's `max_nesting`.
- The rule file accepts eleven engine names. Six have no adapter yet and are reported "not available": golangci-lint,
  pylint, clang-tidy, oclint, shellcheck and git.

### Changed

- A `check` that clippy, SwiftLint or ruff does not know is reported "not available", naming the engine's version.
- A param that an engine does not take fails the rule.
- A rule that applies only with `--base` is listed as not applicable when no base was given.

## 0.1.0

The first version. Built from source; there are no release downloads yet. MIT licensed.

### Commands

- `tm check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]`: runs every rule's
  engine and prints the result, as text ending in a `next:` line or as one JSON object carrying `exit` and `next`.
- `tm rules [--rules <path>] [--json]`: reads and checks the rule file, lists each rule with its engine's
  availability, and lists the exceptions in force. It runs no engine.
- `temperare install` places the binary in `~/.local/bin`, links `tm` to it unless another program owns that name on
  `PATH`, and offers to wire a skill into Claude Code, Codex and Cursor. It records what it placed in
  `~/.local/state/temperare/installed`, and `temperare uninstall` removes what that record names.
- `tm help` and `tm --version`.

### The rule file

- `temperare.toml`, version 1: a top-level `owner`, kits (`[kits.<name>]` with `paths` and `owner`), rules
  (`[[rule]]`) and exceptions (`[[exception]]`).
- A rule has `id`, `engine`, `check` and `fix`, and may have `severity`, `kind`, `paths`, `exclude`, `kits`,
  `outside_kits`, `config` and `params`.
- An exception names one rule and one file, with a `reason`, who `approved` it, and an optional `what`.
- Every problem in a rule file is reported at once with its line, its column and what to write instead.

### Engines

- clippy (`cargo clippy`), for one lint the toolchain lists or for every warning, with an `allow` list. It needs
  `cargo-clippy` and `clippy-driver`.
- SwiftLint, for one rule identifier, with an optional config file.
- ruff, for a rule code, a code prefix, `ALL` or several joined by commas, with an optional config file.
- ast-grep, for a rule file of your own inside the rule file's folder, with named `params` filled into it.
- Tessera is accepted as an engine name. Its rules are always reported "not available".
- A `check` that cannot be matched back to findings fails its rule and never reads as clean: a clippy lint group,
  a lint the toolchain does not list or a `rustdoc::` lint; SwiftLint's `custom_rules`; a ruff rule name; an
  ast-grep rule file that is outside the rule file's folder, reached through a symlink to outside it, or
  hard-linked.
- A `check`, `config` or `params.allow` value that begins with `-` is a problem in the rule file, and each value
  reaches its engine joined to its flag, as that flag's value.

### The result

- One finding shape for every engine: rule, engine, kind, severity, owner, path, line, column, the engine's message
  and the rule's fix text.
- `--base <rev>` keeps only the findings on lines added since a commit, untracked files included.
- Exceptions silence the findings they cover; every result counts the silenced findings and the exceptions in
  force.
- Fixed exit codes for `tm check`: 0 nothing to refuse, 1 a quality error, 2 used wrongly or a rule-file problem,
  3 a rule not available, 4 security errors only, 5 a rule failed. `tm install` and `tm uninstall` exit 0, 1 when
  a step failed, or 2 when used wrongly.
- A failed rule's reason is printed in the text output and carried in `--json` as `stderr_tail`.
- Security rules (`kind = "security"`) carry an owner, and a security finding never makes the exit code 1.
  `--kind security` runs them alone.

### Known limits

- Rules with `engine = "tessera"` do not run.
- Exit code 1 does not say every rule ran: a run with a quality error ends with 1 even when another rule was not
  available or failed. The counts say so.
- A rule whose `paths` match no file reports nothing and is not flagged.
- A SwiftLint rule fails in a folder that holds no Swift file.
- The ast-grep rule file is opened without following symlinks on Unix only; elsewhere only its path is checked.
- Built and accepted on macOS. Linux and Windows are untested.

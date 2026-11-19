# Changelog

All notable changes to Temperare are recorded here, newest first.

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

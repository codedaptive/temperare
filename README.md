# Temperare

Temperare checks code against the rules a repository has chosen to enforce, and reports what it finds. The command is `tm`.

It runs other tools as engines (clippy, SwiftLint, ruff and ast-grep) and parses no code itself. Every finding comes back in one shape, whichever engine found it, with the fix the rule's author wrote. A rule file may already name Tessera as an engine; such rules are reported "not available" for now (see below).

Temperare checks and reports. It never edits code, and it never decides what a finding blocks: that belongs to whoever calls it.

## Install

```
cargo install --path crates/cli --locked
tm --version
```

Install the engines your rules name. A rule whose engine is missing is reported as "not available"; it is never reported as clean.

## Use

Put a rule file, `temperare.toml`, at the top of a git repository:

```toml
version = 1
owner = "payments"

[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
severity = "error"
fix = "Compare with `is None`."
```

Then:

```
tm rules              # the rules in force, and whether each one's engine is available
tm check              # check the repository
tm check src          # check only the files under src
tm check --base main  # report only what was added since a commit
tm check --json       # the same report as one JSON object
tm check --kind security   # the security rules alone
tm help
```

`--rules <path>` names a rule file anywhere else. `tm check` ends with a `next:` line that says what to do.

## The rule file

| Key | Meaning |
|---|---|
| `id` | The rule's name in every report: lower-case letters, digits and `-` |
| `engine` | `clippy`, `swiftlint`, `ruff`, `ast-grep` or `tessera` |
| `check` | What the engine is asked: a lint or rule code (`clippy::bool_comparison`, `E711`, `force_cast`), `warnings` for every clippy warning, or the path of an ast-grep rule file |
| `config` | The engine's own config file, passed to SwiftLint or ruff as written |
| `paths` | The files the rule covers, as globs; all files when absent |
| `exclude` | Globs the rule leaves out |
| `kits` | The rule applies only inside the named kits |
| `outside_kits` | The rule applies everywhere except inside the named kits: "this is named only inside kit Y" |
| `severity` | `error` (the default) or `warning` |
| `kind` | `quality` (the default) or `security`. A security rule needs a top-level `owner` |
| `fix` | The text shown with each finding |
| `params` | Values passed to the engine: into an ast-grep rule (such as a list of type names), or `allow` for clippy (lints left out) |

`id`, `engine`, `check` and `fix` are required. A `[kits.<name>]` table gives a kit its `paths` and its `owner`. A finding's owner is the first kit that holds its file, else the rule file's top-level `owner`.

An exception is a table in the same file, never a comment in the code:

```toml
[[exception]]
rule = "no-none-compare"
path = "legacy.py"
reason = "the legacy module is rewritten in 2.0"
approved = "who approved it"
```

An optional `what` limits it to the findings whose message contains that text. Every report counts the exceptions in force.

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Nothing to refuse. Warnings alone end here |
| 1 | A quality rule at severity `error` was broken |
| 2 | The command was used wrongly, or the rule file has a problem |
| 3 | A rule's engine was not available |
| 4 | The only errors are security findings. A security finding is reported with its severity, place and owner, and never refuses |
| 5 | An engine failed |

When more than one applies, the first of 2, 1, 5, 3, 4 wins.

## With Tessera and Mortar

Tessera reads the code, Mortar writes its history, and Temperare checks it. Each one works alone.

- Rules with `engine = "tessera"` are accepted, but for now they are reported "not available" whether or not Tessera is installed: Tessera does not yet define the records Temperare would read.
- Temperare needs nothing from Mortar.

## Working on Temperare

`CLAUDE.md` is where an agent starts. This repository carries its own `temperare.toml` and is checked by itself.

`make check` type-checks the workspace and `make test` runs the unit tests (`GROUP=integration` or `GROUP=validation` for the other kinds). `make accept` runs the mission's Done list against the real engines on throwaway repositories and writes a report to `target/accept-report.md` (`ACCEPT_REPORT=<file>` for another place). It fails when a check fails or does not run, for example when an engine is not installed.

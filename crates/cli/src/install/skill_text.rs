//! `SKILL_TEXT`: the Claude Code skill `temperare install` writes into each harness it wires (S-32, S-33).

/// The skill file's whole text: YAML front matter (`name`, `description`), then how to run Temperare, the tokens-saved
/// line `temperare check`'s text opens with when an engine ran, and its ask (sum the lines, tell the user at the end of
/// the reply), that `--json` carries the numbers as `saved` instead, that `savings_line = false` turns line and ask
/// off, and how to read the exit codes. It teaches the commands as `temperare`, the name that answers wherever it is installed: `tm` exists only
/// where install made the link. A Rust constant, compiled in: the repository holds no SKILL.md (S-33). The exit codes
/// are S-15's, as crates/cli/src/output/exit_code.rs gives them.
pub const SKILL_TEXT: &str = "\
---
name: temperare
description: Check code against the rules this repository has chosen, with Temperare (temperare check), and read its report.
---

# Temperare

Temperare checks code against the rules a repository or kit has chosen to enforce, and reports what it finds. It
runs engines (clippy, SwiftLint, ruff and ast-grep; Tessera's checks are not available yet, so a tessera rule always
reports not available) and never edits the code it checks.
The rules come from `temperare.toml` at the top of the git repository.
`tm` is its short name where `temperare install` made the link.

## Commands

- `temperare check [--base <rev>] [--json] [<path>...]`: check the repository, or only the paths given. With
  `--base`, only violations new since that commit are reported. `--json` prints the report as JSON.
- `temperare rules`: list the rules in force and whether each one's engine is available.
- `temperare rules --loosened --base <rev>`: list what the rule file loosened since that commit; exit 0 nothing, 1
  something loosened, 2 a bad --base or rule file.

## Tokens saved

When an engine ran, the first line of `temperare check`'s text output estimates the tokens it saved:
`[temperare] tokens saved ≈ N (P%) — this output ≈ X tok vs the raw output of K engine run(s) ≈ B tok (estimate).`
It is followed by an ask: at the end of your reply, tell the user the total temperare tokens saved this turn, the sum
of each such line across your temperare calls. With `--json` there is no line: the object carries the numbers as
`saved`. `savings_line = false` in `~/.config/temperare/config.toml` turns the line and its ask off.

## Exit codes

- 0: nothing refuses; warnings alone exit 0.
- 1: a quality error finding refuses the change. Fix it, or add an approved exception to the rule file.
- 2: a usage error. The message says what to type instead.
- 3: a rule could not run because its engine, or what the rule needs, is not available. Do what the message names
  (install the engine; for clippy, put the rule file inside the Cargo project it checks), or run without those rules;
  Tessera's checks are not available yet, so installing Tessera changes nothing.
- 4: security error findings only. Raise them with their owner; work goes on.
- 5: an engine failed. Read its output.
- 6: a rule of the repository's own covers nothing: its paths match no file, or no file its engine checks.

\"Not available\" is never \"clean\": exit 3 means some rules did not run, so the code was not fully checked.
";

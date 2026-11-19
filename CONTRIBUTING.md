# Contributing to Temperare

Thank you for your interest in Temperare. This document explains how to contribute.

The short version: open an issue to report a problem or propose an idea, or open a pull request with a change and
its tests.

> **License:** Temperare is MIT licensed ([`LICENSE`](LICENSE)).

---

## Issues: tell us what you know

If a rule reported something it should not have, missed something it should have caught, or said "clean" when an
engine did not run, open an issue and tell us. You do not need a fix in hand.

A good issue has:

- the output of `tm --version`, and the version of the engine involved;
- the smallest `temperare.toml` and input file that show it;
- what `tm check --json` printed, and its exit code;
- what you expected.

A document that disagrees with the program is a bug too. Say which page and which command.

Do not put a security problem in a public issue. See [Reporting security issues](#reporting-security-issues).

---

## Pull requests

Keep a change focused, include its tests, and say what it does and why. For a large change, open an issue first to
check the direction.

### Build and test

You need git, a current stable Rust toolchain with cargo, `make` and a POSIX `sh`. The workspace sets no minimum
Rust version.

```
make check                             # cargo check, the whole workspace, every target
make test                              # the unit tests of every crate
make test-one DIR=crates/<crate>       # the unit tests of one crate
make test GROUP=integration            # the tests that touch the disk, git or a real engine
make test GROUP=validation             # the tests of the built binary
make accept                            # the acceptance checks, with a report
```

`GROUP` is `unit` (the default), `integration` or `validation`, and works with `test-one` as well.
`CARGO_TEST_ARGS=<name>` passes a test name filter. `make accept` needs clippy, SwiftLint, ruff and ast-grep
installed, and the integration tests of the `engines` crate run those engines for real. `make accept` writes its
report to `target/accept-report.md` and fails when a check fails or does not run.

Start with the smallest test that proves your change, then widen.

### The layout

One binary, built from a Cargo workspace of five crates. A crate depends only on the crates below it.

```
cli       the binary: the commands, the output, the exit codes
 ├── report   the report and its counts; new findings only, against a base commit
 ├── engines  one adapter per engine: run it, read its output into findings
 ├── rules    the rule file: read it, check it, scope, exceptions; the rule and finding types
 └── procio   the one place that runs another program, git included
```

`procio` and `rules` depend on no other crate here. `engines` depends on both. `report` depends on `rules` and
`procio`. `cli` depends on all four.

### What a change must keep

These hold today, and a pull request that breaks one will be asked to change.

1. **Rules, not parsers.** Temperare parses no source code. A new check is a rule on an engine. A new language is a
   new engine adapter.
2. **Engines are programs.** No engine is linked. Every program, git included, is started by `procio` and nowhere
   else. The repository's own `temperare.toml` checks this: run `tm check` in your checkout.
3. **"Not available" is never "clean".** A rule that could not be run says so. So does an engine that ran and gave
   no answer that can be read, and so does a rule whose `check` cannot be matched back to the engine's findings: it
   fails, it does not report nothing.
4. **A rule's value is never an engine option.** A value from the rule file reaches an engine joined to its flag
   (`--select=<check>`), and an ast-grep rule file is read only from inside the rule file's folder.
5. **It checks and reports.** No code path edits the code under check, and none decides what a finding blocks.
6. **The rule file is the only home** of a rule and of an exception. Nothing is read from a comment in the code.
7. **Useful alone.** Reads no environment variable of its own, no service, nothing needed beyond git and the engines the
   rules name.
8. **The output is a contract.** The fields of a finding, the JSON keys, the text lines, the exit codes and their
   order are what callers build on. A change to any of them is a breaking change: say so, and update
   [`docs/reference/OUTPUT.md`](docs/reference/OUTPUT.md) and [`CHANGELOG.md`](CHANGELOG.md) in the same pull
   request.

### How the code is written

- **Small files.** One public item per file, named for it, with its tests in a sibling `<name>_tests.rs`.
- **Comments say what the code does now** and why, not what it used to do.
- **Pure where it can be.** A command line is built by a function that runs nothing; an engine's output is read by a
  function that is given the bytes. The functions that touch the disk or start a program are few and thin.

### How the tests are grouped

- **Unit** tests are given an engine's output; they do not run the engine, git or any other program. The captured
  output lives in `crates/engines/src/<engine>/fixtures/`.
- **Integration** tests run the real thing on files the test makes: git on a throwaway repository it removes, an
  engine on a temporary file. They never touch a repository that was there before the test. They live in a crate's
  `tests/` folder and in the `integration` modules inside its `src/`.
- **Validation** tests run the built binary. The acceptance checks are among them, in `crates/cli/tests/validation/`.

No test reaches the network.

### Adding an engine

An engine adapter is a folder in `crates/engines/src/` with three parts: a function that builds the command line, a
function that reads the engine's output into hits, and a function that joins the two. To add one:

1. Add the engine to `Engine` in `crates/rules/src/types/engine.rs` and to `Engine::from_name` beside it
   (`engine_from_name.rs`), and its name to the "unknown engine" fix text in
   `crates/rules/src/rule_file/read_rule.rs` and to `engine_name` in `crates/report/src/render_text.rs`.
2. Add its program name in `crates/engines/src/program.rs`. An engine that needs more than one program on `PATH`,
   as clippy does, says so in `crates/engines/src/availability.rs`.
3. Write the adapter folder, with captured output of the real engine as fixtures and unit tests for the builder and
   the reader.
4. Dispatch to it in `crates/engines/src/engine_hits.rs`.
5. Add an integration test that runs the real engine on a file the test makes.
6. Document it in [`docs/reference/ENGINES.md`](docs/reference/ENGINES.md) and in the README's tables.

### Documents

A change that alters what a command, a flag, a rule-file key or the output does changes its document in the same
pull request. Every statement in `README.md` and `docs/reference/` is meant to be checkable against the code.

---

## Reporting security issues

**Do not report security-sensitive issues in public.**

Temperare runs other programs with the privileges of the person who starts it. A security problem disclosed in a
public issue is disclosed to everyone before it can be fixed.

> **Security contact:** report privately through GitHub's
> [Report a vulnerability](https://github.com/codedaptive/temperare/security/advisories/new) form, or email
> <security@codedaptive.com>. See [`SECURITY.md`](SECURITY.md) for the full policy, including what is and is not a
> vulnerability in a tool whose job is to run linters.

---

## A note on conduct

Engage in good faith, assume good faith in others, and keep discussion focused on the work. That covers it.

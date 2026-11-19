# Temperare documentation

This folder holds the reference for Temperare. The layout follows what a reader arrives wanting, and each page
describes what the code does now.

## How to find what you want

**You are new and want to know what this is.**
Start with the [`README`](../README.md): what Temperare is, who it is for, what it is not, how to install it, and a
first run you can copy.

**You are an AI assistant, or you are setting one up.**
[`AI_START_HERE.md`](../AI_START_HERE.md) says what to read and in what order, when Temperare helps and when it does
not, and how to install and run it for a user. [`llms.txt`](../llms.txt) is the short index of every document.

**You are writing or changing rules.**
[`reference/RULE_FILE.md`](reference/RULE_FILE.md) is the whole of `temperare.toml`: every key, how a rule's scope
is worked out, kits and owners, exceptions, and every problem `tm` reports in a rule file.

**You are reading what `tm` printed, or writing a program that does.**
[`reference/OUTPUT.md`](reference/OUTPUT.md) gives the text form line by line, the JSON form field by field, the
exit codes and the order they win in, and the `next:` line.

**A rule says "not available" or "failed", or an engine does not behave as you expect.**
[`reference/ENGINES.md`](reference/ENGINES.md) gives the exact command line run for clippy, SwiftLint, ruff and
ast-grep, what `check`, `config` and `params` mean to each, and what Tessera rules do today.

**You want to run it on code you do not trust, or report a vulnerability.**
[`SECURITY.md`](../SECURITY.md) says what Temperare runs, what the rule file can make it do, and how to report.

**You want to change Temperare itself.**
[`CONTRIBUTING.md`](../CONTRIBUTING.md) has the layout of the code, the build and test commands, and the rules the
code keeps.

**You want to know what changed.**
[`CHANGELOG.md`](../CHANGELOG.md).

## The pages

| Page | What it answers |
|---|---|
| [`reference/RULE_FILE.md`](reference/RULE_FILE.md) | What can I write in `temperare.toml`, and what does each key do? |
| [`reference/OUTPUT.md`](reference/OUTPUT.md) | What does `tm` print, and what does each exit code mean? |
| [`reference/ENGINES.md`](reference/ENGINES.md) | What exactly is run for each engine, and when is a rule "not available" or "failed"? |

## Conventions

- A reference page describes behaviour you can check: every command, flag, key and output shape on it is in the
  code. If a page and the program disagree, the program is right and the page has a bug; please report it.
- File names in `reference/` are upper case with underscores.
- Examples use `tm`, the short name. `temperare` answers the same commands.

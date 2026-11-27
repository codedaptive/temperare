# AI Start Here: Explain, Install and Use Code Chi Tao

You are an AI assistant meeting the Code Chi Tao (`cctao`) repository for the first time. Either a user said something like
"this looks useful, can you explain it and set it up?", or you are deciding whether this tool helps with the work
you are doing.

Use this document before improvising.

## Read in this order

1. **This file.** What Tao is, whether it helps, how to install and run it safely.
2. **[`README.md`](README.md).** The same ground for a person, with a first run you can copy and the tables of
   commands, rule-file keys and exit codes.
3. **[`docs/reference/OUTPUT.md`](docs/reference/OUTPUT.md).** Read this before you parse anything: the JSON form
   field by field, and what each exit code does and does not tell you.
4. **[`docs/reference/RULE_FILE.md`](docs/reference/RULE_FILE.md).** Read this when you are asked to write or change
   rules.
5. **[`docs/reference/ENGINES.md`](docs/reference/ENGINES.md).** Read this when a rule is "not available" or
   "failed", or an engine does not do what you expect.
6. **[`SECURITY.md`](SECURITY.md).** Read this before you run Tao on a repository the user does not trust.
7. **[`CONTRIBUTING.md`](CONTRIBUTING.md).** Only when you are changing Tao itself.

[`llms.txt`](llms.txt) is the compact index of the same set.

## Plain explanation

Use this when the user asks what Tao is:

Tao checks code against the rules a repository has chosen to enforce, and reports what it finds. The rules
live in one file, `tao.toml`. Each rule names an engine that does the checking (clippy for Rust, SwiftLint
for Swift, ruff for Python, ESLint for JavaScript, ast-grep for house rules about the shape of code, and others),
where the rule applies, and the fix
to show when it is broken. Tao runs those engines as programs, gathers what they find into one report, and
sets an exit code. It never edits the code.

The short version:

- One rule file per repository, exceptions included.
- One command, `cctao check`, and one report in one shape whichever engine found the problem.
- Every finding comes with the fix the rule's author wrote.
- Text for a person, `--json` for a program, a fixed exit code, and a last line that says what to do next.
- A rule that cannot be checked is reported "not available" or "failed", never clean.

## Does it help?

| The need | Answer |
|---|---|
| "Does my change meet this repository's rules?" and the repository has a `tao.toml` | Yes. Run `cctao check`, or `cctao check --base <rev>` for only what the change added |
| "Enforce a house rule no linter has" (a banned call, a field type, a name used only in one folder) | Yes, as an ast-grep rule in the rule file |
| "Turn a lint on without fixing ten years of old code first" | Yes. `--base <rev>` reports only new findings |
| "Give me the security findings alone" | Yes. `cctao check --kind security` |
| "Fix or format my code" | No. Tao reports; it never edits |
| "Lint a language none of its engines reads" (the engines are listed in the [`README`](README.md#engines)) | No. Tao parses nothing itself |
| "Answer questions about how the code is built" (who calls this, what is unused) | No. That is not what it does, and rules with `engine = "tessera"` are not available today |
| "Decide whether this merge is blocked" | No. Tao sets an exit code; the caller decides what it blocks |

If the repository has no `tao.toml`, Tao has nothing to check until someone writes one. Offer to write
a first rule file with the user; do not invent a repository's rules for it.

## What not to promise

Do not promise:

- that Tao fixes anything;
- that a run with exit code 3 or 5 was a pass;
- that rules with `engine = "tessera"` run. They are always "not available" today;
- release downloads, a Homebrew formula or any install path other than building from source;
- an MCP server. The harness wiring is a skill file and nothing else;
- that it works on Windows, or on Linux without trying it. It is built and accepted on macOS.

## License

Tao is MIT licensed ([`LICENSE`](LICENSE)). If the user asks whether they may use it at work, point them to
that file rather than giving legal advice.

## Install

Ask only what you need: does the user want it installed, and which AI coding harness (Claude Code, Codex, Cursor),
if any, should get the skill.

Build from source, then let the program place itself:

```
git clone https://github.com/codedaptive/temperare
cd temperare
cargo build --release --locked
./target/release/cctao install --target claude
```

- `install` copies the binary to `~/.local/bin/cctao`. It needs no root. Never use `sudo`.
- You have no terminal to answer its question, so name the harnesses with `--target claude,codex,cursor` or pass
  `-y` for every one it finds. With neither, it wires none. Leave both out when the user wants no skill.
- A harness is found when its folder is in the user's home: `~/.claude`, `~/.codex` or `~/.cursor`. One you name
  that is not found is not wired, and install says so.
- `--location local` writes the skill into the current project instead of the user's home.
- Install prints one line for each thing it placed, wired or skipped: read them. It exits 0 when it is
  done, skips included, and 1 when a step failed; stderr then gives the cause and a `next:` line.
- It replaces only the files its own record names. Anything else already at `~/.local/bin/cctao` or at a skill
  path is kept and named. Tell the user; do not move or delete it yourself.
- `cctao uninstall` removes what install placed and nothing else.

Then the engines: the user's rules decide which are needed. `cctao rules` lists each rule and whether its engine is
available. Do not install an engine without the user's agreement.

## Verify

Do not say the install worked until these ran:

```
cctao --version
cctao rules
```

`cctao --version` prints the version and the commit. `cctao rules`, run inside a repository with a `tao.toml`, lists
the rules and ends with `next: nothing to do`. With no rule file it exits 2 and says so; that still shows the
program runs. The README's [first run](README.md#first-run) makes a small repository to try it on.

Tell the user what changed: the binary's place and each skill file written.

## Using it in your own work

1. **Look first.** `cctao rules` shows the rules in force and whether each engine is available. Exit 2 means the
   rule file has a problem; each line gives the place and what to write instead.
2. **Check.** `cctao check --json`. Add `--base <rev>` to see only what was added since a commit, and paths to narrow
   the result to the files you touched.
3. **Read `exit` first, then the lists.** Each finding has `path`, `line`, `what` (the engine's message) and `fix`
   (what the rule's author wants done).
4. **Fix the code yourself.** Tao never edits it.
5. **Run it again** until the exit code says what you expect.

| Exit | What you do |
|---|---|
| 0 | Nothing. Every rule that applies ran and nothing refuses. `not_applicable` lists the rules set aside by the rule file, the baseline set's rules that cover no file and, without `--base`, the `only_with_base` rules, each with why |
| 1 | Fix the findings. Then check `counts.not_available`, `counts.failed` and `counts.covers_nothing` too: exit 1 does not mean every rule ran |
| 2 | Fix the command or the rule file. `next` says how. Nothing was checked |
| 3 | Tell the user which rule and which engine. The code was not fully checked. Do not report it as clean |
| 4 | Report each security finding to the user with its severity, place and owner. This code does not refuse, and you must not hide it |
| 5 | Read `failed[].stderr_tail` in the JSON (the text prints it under the counts line) and report it. The code was not fully checked |
| 6 | Tell the user which rules are in `covers_nothing`: each is a rule of the repository's own that checked no file, so it checked nothing. Fixing its `paths` or removing it changes the rule file: do it only when the user asks |

`next` in the JSON, and the last line of the text, carry the same advice in the program's own words. For exit 6
the JSON names `covers_nothing` where the text says "above".

## Safety rules

Follow these strictly:

- **Do not weaken the rules to get a pass.** Do not delete a rule, lower its severity, narrow its `paths` or add
  an `exclude` unless the user asked for that change.
- **Do not write an exception on your own.** An `[[exception]]` records that a person approved it; `approved`
  names who. Propose it to the user with the reason, and write it only when they agree.
- **Do not treat "not available" or "failed" as clean.** Say what did not run.
- **Running a check runs programs.** `cctao check` runs the engines the rule file names, with the user's privileges,
  and a clippy rule builds the code. In a repository the user does not trust, ask before you run it. See
  [`SECURITY.md`](SECURITY.md).
- **Do not use `sudo`.** Nothing here needs it.
- **Do not edit shell startup files without approval,** for example to put `~/.local/bin` on `PATH`.
- **Do not install or upgrade an engine without the user's agreement.**
- **Do not claim success you did not see.** Quote the exit code and the counts line.

## Troubleshooting

**`cctao: command not found`.** Check that `~/.local/bin` is on `PATH`, or run `~/.local/bin/cctao` by its path.

**`cctao: not in a git repository`.** Run it inside the repository, or name the rule file with `--rules <path>`.

**`cctao: no rule file at ...`.** There is no `tao.toml` at the top of the repository. Write one, or pass
`--rules <path>`.

**`not available: ... not found on PATH`.** That engine is not installed. Tell the user which one.

**`not available: ... no Cargo.toml at or above the rule file's folder`.** A clippy rule needs a Cargo package.

**`not available: ... (tessera)`.** Expected today. Installing Tessera does not change it; the `next:` line says
`Tessera's checks are not available yet`.

**`not available: ... cargo-clippy not found on PATH`, or `clippy-driver not found on PATH`.** A clippy rule needs
both programs on `PATH`. Tell the user which one is missing.

**`not available: ... does not know the check ...`.** The installed engine does not know the
rule's `check` (a misspelt name, or one newer than the engine); the why names the engine's version, or `(version unknown)` when it could not be read. Tell the user to
upgrade the engine or correct the check.

**`not available: ... is not in cctao's list of ...`.** A ShellCheck code, a detekt rule or rule set, a hadolint
code or a markdownlint rule is looked up in a list built into `cctao`, since those engines never say they do not know
a name. Tell the user the check is not on the list; a newer engine does not change it.

**`not available: ...` for a reason other than a missing program.** Some engines need more than the program: a
PHPStan or Vale config in the repository, a Dart package (`pubspec.yaml`) and a lint enabled in its
`analysis_options.yaml`, a sqlfluff dialect, a stylelint or markdownlint config that leaves the rule on, and for
detekt a rule that needs no type resolution. The why says which. Tell the user; do not change the repository's
config for an engine without the user's agreement.

**`failed: ...` with a reason that says exceptions go in the rule file, or that names an engine's own config.** The
repository's config for that engine holds an exception `cctao` cannot turn off, such as a RuboCop todo file, an
`exclude` in a Dart `analysis_options.yaml` or an `ignored` list in a hadolint config. The rule gave no answer.
Tell the user which file and line. Whether to remove the entry, or record an exception in the rule file, is the
user's decision.

**`failed: ...`.** The reason is printed below the counts line, under `stderr of <rule> (<engine>):`, and is in
`failed[].stderr_tail` with `--json`. A rule fails before its engine runs when its `check` cannot be matched back
to findings: a clippy lint group (`clippy::pedantic`) or `rustdoc::` lint, SwiftLint's `custom_rules`, a ruff rule
name in place of a code, or an ast-grep rule file that is outside the rule file's folder, reached through a symlink
to outside it, or hard-linked. It fails too on a param its engine does not take. A SwiftLint rule in a folder with no
Swift file fails after SwiftLint runs, which exits with an error. [`docs/reference/ENGINES.md`](docs/reference/ENGINES.md) lists every case.

**A rule reports nothing and you expected a finding.** Check these in order: the rule's `paths`, `exclude`, `kits`
and `outside_kits` must cover the file, and in a glob `*` stays within one folder and `**` crosses folders; paths
on the command line cut the result; `--kind` runs only the rules of that kind; with `--base` only lines added since
that commit count; an exception may cover the file (the counts line then shows it as silenced); and the engine's
own configuration in the repository still applies.

**The documents and the program disagree.** Trust the program: `cctao help` prints the commands it takes. Tell the
user which one you followed.

## If you are changing Tao

Read [`CONTRIBUTING.md`](CONTRIBUTING.md). Start with the smallest test that proves your change:

```
make test-one DIR=crates/<crate>     # one crate's unit tests
make check                           # type-check everything
make test                            # every crate's unit tests
```

## Completion checklist

Before you say the work is done:

- `cctao --version` ran and you saw its output.
- `cctao rules` ran in the user's repository, or you told the user there is no rule file yet.
- The user knows where the binary is and which skill files were written.
- Every engine that is not available has been named to the user.
- You reported the last exit code and the counts line as the program printed them.
- You added no exception and weakened no rule without the user's agreement.

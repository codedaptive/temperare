# Security Policy

Temperare runs other programs over your code, with your privileges, and tells you what they found. If you find a
problem that touches that trust, please report it privately.

## Security by construction

These are properties of the code, not of how it is deployed.

- **Programs are run directly, never through a shell.** Each argument reaches the program as one argument, as
  written. A `--base` value is always read by git as a revision, never as an option, and the paths you give on the
  command line are never passed to git or to an engine: they only cut the result.
- **A rule's value is passed as a value, not as an option.** A `check`, `config` or `params.allow` value that
  begins with `-` is a problem in the rule file, and nothing runs. Each value reaches its engine joined to its own
  flag, as in `--select=<check>`.
- **An ast-grep rule file is read only from inside the rule file's folder.** A `check` that is an absolute path, or
  that resolves outside the folder through `..` or a symlink, fails the rule and ast-grep is not run. On Unix the
  file is opened one path component at a time from that folder, following no symlink, and a file with a second
  hard link is refused. On other systems only the path checks apply.
- **The scripts listed for a ShellCheck rule never include a symlink.** `tm` picks them among the files git lists
  in the rule file's folder and leaves out every path that is a symlink or lies under a symlinked folder, whatever
  it points at, and everything that is not a regular file. To tell a script that has no extension it reads at most
  the file's first 257 bytes, on Unix only, opened without following a symlink and without waiting on a FIFO; on
  other systems it reads none and such a file is left out.
- **The sources listed for a clang-tidy rule never include a symlink.** `tm` picks them among the files git lists
  in the rule file's folder by their extension, keeps only regular files that are not symlinks, and reads none of
  them itself.
- **One place starts programs.** Everything Temperare runs, git included, goes through one small crate, `procio`.
  The repository's own rule file holds that line: a rule reports any other code that starts a process.
- **It does not edit what it checks.** `tm check` and `tm rules` write to stdout and stderr. `tm check` creates
  temporary files in the system's temporary folder for four kinds of rule, and only for them:
  - an ast-grep rule that gets as far as its scan (each one, with or without `params`): a copy of its rule file
    with its params filled in, `tm-ast-grep-<pid>-<n>.yml`;
  - a SwiftLint rule with `max_complexity` or `max_lines`: the threshold as a SwiftLint config,
    `tm-swiftlint-<pid>-<n>.yml`;
  - a clippy rule with `max_nesting`: a new folder `tm-clippy-<pid>-<n>` holding one `clippy.toml` with the
    threshold;
  - a Periphery rule that gets as far as its run: a new folder `tm-periphery-<pid>-<n>` holding the package's
    build and its index, a scratch path for each local package's manifest reading, SwiftPM's description of the
    package and an empty Periphery config. Nothing is built in
    the repository or in DerivedData.

  Each is created new, never opened over an existing file or folder, and removed when the run of that rule ends. If
  `tm` is killed in between, it is left there. No config file of the repository is written. A ruff threshold is
  passed as an argument, and a ShellCheck, git, pylint, clang-tidy, golangci-lint, gitleaks, cargo-audit or vulture
  rule writes no file. A
  golangci-lint run passes `--fix=false`, so a repository config that asks golangci-lint to fix cannot edit the
  code, and runs go with `GOFLAGS` removed, so go does not rewrite `go.mod` or `go.sum`. A rule that is refused, or
  that covers no file, writes none either. Besides these, `tm check` appends one line to its savings log,
  `~/.local/state/temperare/savings.jsonl`, when an engine ran, making that folder when it is missing.
  The line holds the time, the rule file's folder as an absolute path, the verb and two token counts; no code and no
  finding.
- **No network code.** Temperare opens no connection. Its dependencies are parsers (TOML, JSON, globs,
  regular-expression syntax) and their support crates, and on Unix `libc`, for the calls that open a file without
  following a symlink: an ast-grep rule file, the first bytes of a file that may be a shell script, and the savings
  log.
- **A check reads nothing back between runs.** There is no cache and no background process. The files Temperare
  keeps are the record `temperare install` writes (see [What install touches](#what-install-touches)) and the savings
  log above, which it only appends to and never reads. It reads one file of the user's,
  `~/.config/temperare/config.toml`, for the switch that turns the savings line off, and never writes it.
- **git is pointed at one repository.** git runs with the variables that could redirect it to another repository,
  index or object store removed, with credential prompts turned off, and without taking the index lock.
- **A program it runs cannot read your terminal.** The caller's stdin is never passed on.
- **"Not available" is never "clean".** A rule that has a file to check and whose engine is missing, or that cannot
  be run, is reported as such, is counted, and has its own exit code. A rule that failed has another, and a rule
  `tm` refuses from the rule file alone fails whether or not its engine is installed. A rule that covers no file is
  not run, and the report says so ([below](#what-this-does-not-defend-against)).

## What this does not defend against

Limits, as plainly as possible.

**The rule file is trusted input.** `temperare.toml` decides which engines run, what each is asked, and which config
file each is given. Temperare keeps a rule's value from being read as an engine option and keeps an ast-grep rule
file inside the rule file's folder, as above. It does not judge what a value asks for. A `config` is handed to
SwiftLint, ruff, ShellCheck, pylint, clang-tidy, golangci-lint or gitleaks as written and may name any file, inside the
repository or not. An ast-grep rule file is the
repository's own, and ast-grep runs whatever it holds over the code. Running `tm check` in a repository is running
that repository's instructions.

**The engines are trusted programs.** Temperare finds git, cargo, SwiftLint, ruff, ast-grep, ShellCheck, pylint,
clang-tidy, golangci-lint, go, gitleaks, cargo-audit, vulture, Periphery and swift on your `PATH` and runs whatever is there, with your environment and your privileges. Whoever controls `PATH` controls what runs.
There is no sandbox.

**A clippy rule builds the code.** `cargo clippy` compiles the crate. A build runs that crate's build scripts and
procedural macros, and cargo may download its dependencies. That is code from the repository running as you.

**A Periphery rule builds the package.** Before Periphery scans, `swift` reads the package's `Package.swift`, and
the `Package.swift` of each local package it depends on; each is Swift code that runs as you. `swift build` then
compiles the package and its tests, running any build plugins it declares. The package is looked for no higher
than the top of the git repository, and a local package outside the repository, or one that does not resolve,
makes the rule not available before anything is built, so Temperare asks swift to read no manifest outside the
repository. The build goes to Temperare's temporary folder, and with `--disable-automatic-resolution` it fetches
no dependency; a source-control or registry dependency, or a binary target with a `url`, in any of those manifests
makes the rule not available before anything is built.

**A golangci-lint rule loads the code with go.** go runs with `GOTOOLCHAIN=local`, `GOPROXY=off` and `GOSUMDB=off`,
so it downloads no toolchain and no module and looks up no checksum; code that needs a module not already on the
machine fails its rule.

**A cargo-audit rule is as current as its advisory database.** cargo-audit runs with `--no-fetch` and
`--no-yanked`, so it reaches no network: it reads the RustSec advisory database already at
`$CARGO_HOME/advisory-db` (else `$HOME/.cargo/advisory-db`), and Temperare never fetches or updates it. An advisory
published after that copy was fetched is not reported, and the report does not say how old the copy is. Fetch it
yourself (`cargo audit update-db`) as often as you need. With no copy there the rule is not available, never clean.

**A gitleaks rule never prints the secret.** gitleaks runs with `--redact`, every gitleaks finding's message is a
fixed text of Temperare's, never gitleaks' own, and of gitleaks' stderr only its error lines are kept, so a matched
secret does not reach the text or the JSON report. The finding's path, line and column are still given, and the
secret is still in the file.

**The engines read their own configuration.** Each engine looks for its config in the code it checks, as it always
does. Except for the exceptions below, Temperare passes no flag that turns that off, so a repository's own linter config shapes what its linter
reports. The exceptions:

- for a clippy rule with `max_nesting`, `CLIPPY_CONF_DIR` points that run at Temperare's temporary `clippy.toml`,
  which clippy reads in place of the repository's own;
- a SwiftLint run given `--config` (the rule's own `config`, or Temperare's threshold file) reads those files in
  place of its own config search;
- a ShellCheck run passes `--norc` unless the rule gives a `config`, so the repository's `.shellcheckrc` is not
  read (with a `config`, that file is read in its place);
- a clang-tidy run passes `--config={}` unless the rule gives a `config`, so no `.clang-tidy` in the code is read
  (with a `config`, that file is read in its place);
- a pylint run turns every message off and the rule's one on, on its command line, which pylint lays over its
  config, so a config cannot turn the rule's message off;
- a golangci-lint run's fixed flags (`--enable-only`, `--fix=false`, no cap on issues) hold over the repository's
  config;
- a gitleaks run has `GITLEAKS_CONFIG` and `GITLEAKS_CONFIG_TOML` removed from its environment, so a config set
  there cannot replace the rule's `config` or the folder's `.gitleaks.toml`; it passes `--ignore-gitleaks-allow`, so
  a `gitleaks:allow` comment silences nothing; and the rule fails before gitleaks runs when the config gitleaks
  reads (or a file it extends) declares an allowlist or does not parse, or when the folder's `.gitleaksignore`
  lists anything, since gitleaks would apply those whatever it is told;
- a cargo-audit rule fails when cargo-audit's report shows it applied an `audit.toml` (settings other than its
  defaults), since that file's `ignore` list would silence advisories outside the rule file;
- a vulture run passes `--config=/dev/null`, so the `[tool.vulture]` table of the repository's `pyproject.toml` is
  not read;
- a Periphery run passes `--config` naming an empty file Temperare writes, so the package's `.periphery.yml` is not
  read.

**Some suppressions written in the code cannot be turned off.** vulture honours a `# noqa` comment on an item's
line, and reads a whitelist module the repository keeps as ordinary code that uses names. Periphery honours
`// periphery:ignore` (with `:all` or `:parameters`) and `// periphery:override`. Neither engine has a switch for
these, so a finding they silence is not reported and not counted. A repository that takes `standard@1` gets each
`# noqa` reported by `tm-python-suppression`; no released set reports the Periphery comments.

**There is no time limit.** Temperare waits for an engine for as long as it runs. An engine that hangs, hangs the
check.

**An exception is as trustworthy as the review of the rule file.** Anyone who can change `temperare.toml` can add
an `[[exception]]`, remove a rule or lower its severity. `approved` is text that Temperare records and does not
verify. Every report counts the exceptions in force so that a new one is visible, but only a review of the file
stops it.

**A security finding does not stop anything.** A finding of kind `security` is reported with its severity, its
place and its owner, and a run whose only errors are security findings ends with exit code 4, which is made to be
different from the code a caller refuses on. If your process must stop on a security finding, the caller has to do
that.

**Exit code 1 can hide a rule that did not run.** When a quality error is found, the exit code is 1 even if another
rule was not available or its engine failed. The counts in the output say so; the exit code alone does not.

**A rule can be written so that it matches nothing.** A `check` Temperare cannot match back to findings fails its
rule and is never clean. A rule whose ast-grep pattern matches no code reports no findings and is not flagged. A
rule that covers none of the files git lists in the rule file's folder (the tracked files in the work tree and the
untracked files git does not ignore) is not run at all: its engine runs no check for it (it is still asked its
version), and whether the engine is installed does not change the result. One of the repository's own rules is
then reported as covering nothing, and makes the exit code 6 when nothing ranks above it; a baseline set's rule is
only counted as not applicable and does not change the exit code. A ShellCheck rule that covers no shell script is
the same, and so is a cargo-audit rule whose folder has no `Cargo.lock` it covers (only the rule file's folder's own
`Cargo.lock` is read), and so is a clang-tidy rule that covers files but no C-family source, whether or not
clang-tidy is installed. Try a new rule on a file you know breaks it.

**Engine messages are printed as given.** In the text output a finding's message is the engine's own text
(gitleaks' apart, as above). The JSON output escapes it.

## Running it on code you do not trust

- Read `temperare.toml` first, and the files it names: each ast-grep rule file and each `config`.
- Expect a clippy or Periphery rule to build the code. If you would not build the repository, do not check it with
  either.
- Use a rule file of your own with `--rules <path>` when you want your rules and not the repository's.
- Run it where a build of that code would be safe: a container, a virtual machine or a throwaway account.

## What install touches

`temperare install` needs no root. It copies the binary to `~/.local/bin/temperare`, links `tm` beside it, writes a
skill file for each AI coding harness you choose, and records what it placed in
`~/.local/state/temperare/installed`. It makes the folders those paths need and writes nowhere else.

- It does not take the name `tm` from another program on your `PATH` unless you pass `--short-name`.
- It replaces only the files its own record names. Anything else already at the binary's path or at a skill path
  is kept, and so is anything at the `tm` path that is not already the link install would make. Install says so.
- Each file is written whole to a new temporary file beside it and then renamed into place, so a symlink at the
  path is not written through.
- A symlink, or anything else that is not a plain file, at the record's path refuses the install or the uninstall
  before anything is placed or removed.
- `temperare uninstall` removes what the record names and nothing else: the files, the `tm` link only while it
  still points where install pointed it, and the folders install made only once they are empty.

## Reporting a vulnerability

**Do not open a public issue for a security problem.**

Use GitHub's [Report a vulnerability](https://github.com/codedaptive/temperare/security/advisories/new) form under
this repository's Security tab, or email <security@codedaptive.com>.

A useful report gives the output of `tm --version`, the versions of the engines involved, the smallest rule file
and input that show the problem, and what you saw.

## What counts

Anything that breaks a property under "Security by construction" or "What install touches", for example:

- a way to make Temperare run a program, or pass an argument, that the rule file and `PATH` did not name;
- a way to make it write or remove a file other than the ones named above;
- a rule that has a file to check reported as clean when its engine did not run, or a silenced finding that is not
  counted;
- a `--base` value read as an option by git, or a path on the command line reaching git or an engine;
- a rule's `check`, `config` or `allow` value read by an engine as an option;
- an ast-grep rule file read from outside the rule file's folder;
- an install that replaces a file its record does not name, or an uninstall that removes something its record
  does not name.

The limits under "What this does not defend against" are how the tool is meant to work. A report that a hostile
rule file can run a hostile build is correct, and is not a vulnerability in Temperare.

## What to expect

Temperare is maintained on a best-effort basis. We read security reports, work confirmed problems privately, and
disclose once a fix is out. We are glad to credit reporters. There is no bug bounty and no response-time guarantee.

## Supported versions

| Version | Supported |
|---|---|
| The `main` branch (0.1.x) | Yes. Fixes land here |

There are no release branches yet.

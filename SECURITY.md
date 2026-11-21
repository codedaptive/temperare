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
- **One place starts programs.** Everything Temperare runs, git included, goes through one small crate, `procio`.
  The repository's own rule file holds that line: a rule reports any other code that starts a process.
- **It does not edit what it checks.** `tm check` and `tm rules` write to stdout and stderr. `tm check` creates
  temporary files in the system's temporary folder for three kinds of rule, and only for them:
  - an ast-grep rule that gets as far as its scan (each one, with or without `params`): a copy of its rule file
    with its params filled in, `tm-ast-grep-<pid>-<n>.yml`;
  - a SwiftLint rule with `max_complexity` or `max_lines`: the threshold as a SwiftLint config,
    `tm-swiftlint-<pid>-<n>.yml`;
  - a clippy rule with `max_nesting`: a new folder `tm-clippy-<pid>-<n>` holding one `clippy.toml` with the
    threshold.

  Each is created new, never opened over an existing file or folder, and removed when the run of that rule ends. If
  `tm` is killed in between, it is left there. No config file of the repository is written. A ruff threshold is
  passed as an argument, and a ShellCheck or git rule writes no file. A rule that is refused, or that covers no
  file, writes none either. Besides these, `tm check` appends one line to its savings log,
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
  not run, and the report does not say so ([below](#what-this-does-not-defend-against)).

## What this does not defend against

Limits, as plainly as possible.

**The rule file is trusted input.** `temperare.toml` decides which engines run, what each is asked, and which config
file each is given. Temperare keeps a rule's value from being read as an engine option and keeps an ast-grep rule
file inside the rule file's folder, as above. It does not judge what a value asks for. A `config` is handed to
SwiftLint or ruff as written and may name any file, inside the repository or not. An ast-grep rule file is the
repository's own, and ast-grep runs whatever it holds over the code. Running `tm check` in a repository is running
that repository's instructions.

**The engines are trusted programs.** Temperare finds git, cargo, SwiftLint, ruff, ast-grep and ShellCheck on your
`PATH` and runs whatever is there, with your environment and your privileges. Whoever controls `PATH` controls what runs.
There is no sandbox.

**A clippy rule builds the code.** `cargo clippy` compiles the crate. A build runs that crate's build scripts and
procedural macros, and cargo may download its dependencies. That is code from the repository running as you.

**The engines read their own configuration.** Each engine looks for its config in the code it checks, as it always
does. Except for the exceptions below, Temperare passes no flag that turns that off, so a repository's own linter config shapes what its linter
reports. Three exceptions: for a clippy rule with `max_nesting`, `CLIPPY_CONF_DIR` points that run at Temperare's
temporary `clippy.toml`, which clippy reads in place of the repository's own; and a SwiftLint run given `--config`
(the rule's own `config`, or Temperare's threshold file) reads those files in place of its own config search; and a ShellCheck run, which passes `--norc` unless the rule gives a `config`, so the repository's `.shellcheckrc` is not read (with a `config`, that file is read in its place).

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
version), whether the engine is installed does not change the result, and no line or
count of the report says so. The exit code is what it would be without that rule, so a
rule with a mistyped `paths` and a missing engine can end in exit 0. A ShellCheck rule that covers no shell script
is the same. Try a new rule on a file you know breaks it.

**Engine messages are printed as given.** In the text output a finding's message is the engine's own text. The JSON
output escapes it.

## Running it on code you do not trust

- Read `temperare.toml` first, and the files it names: each ast-grep rule file and each `config`.
- Expect a clippy rule to build the code. If you would not build the repository, do not check it with a clippy rule.
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

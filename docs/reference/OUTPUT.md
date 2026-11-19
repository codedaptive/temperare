# Output and exit codes

What `tm` prints, as text and as JSON, and what each exit code means. Both forms carry the same result.

## Where output goes

- **stdout:** the result of a check, the `tm rules` listing, the problems of a rule file, the usage text and the
  version.
- **stderr:** a usage error, as two lines: `tm: <what is wrong>` and `next: <what to type instead>`.

`tm install` and `tm uninstall` print one line per step on stdout. A failure is two lines on stderr,
`tm: cannot install: <cause>` (or `cannot uninstall`) and a `next:` line. When install asks which harnesses to
wire, it asks on stderr and reads the answer from stdin.

With `--json`, stdout holds exactly one JSON object on one line and nothing else. One case prints no JSON: a command
line `tm` cannot read at all (an unknown command or flag, a flag given twice, a flag with no value) is reported on
stderr only, exit 2, whether or not `--json` was there.

## `tm check`, as text

```
src/new.py:2:17  error quality no-none-compare  owner payments
  Comparison to `None` should be `cond is None`
  fix: Compare with `is None`.

not available: file-erosion (tessera): Tessera's slop records are not defined yet
failed: clippy-warnings (clippy), no exit code
1 findings (1 error, 0 warning; 0 security), 1 silenced, 1 exceptions in force, 1 not available, 1 failed
stderr of clippy-warnings (clippy):
  clippy param alow: a clippy rule reads only the `allow` param
next: fix the findings above, or add an approved [[exception]] to /path/to/temperare.toml
```

In order:

1. **One block per finding,** three lines and a blank line:
   - `<path>:<line>:<col>`, two spaces, `<severity> <kind> <rule id>`, then `  owner <owner>` when the file has an
     owner. `:<col>` is left out when the engine gives no column.
   - the engine's own message;
   - `fix: ` and the rule's fix text.
2. **One line per rule that was not available:** `not available: <rule id> (<engine>): <why>`.
3. **One line per rule that failed:** `failed: <rule id> (<engine>), exit <code>`, or `no exit code` when the
   engine did not run to an exit.
4. **The counts line.**
5. **Why each failed rule failed:** `stderr of <rule id> (<engine>):`, then the lines of its `stderr_tail`, each
   indented two spaces. When there is nothing to show, the one line `stderr of <rule id> (<engine>): (empty)`.
   A run with no failed rule prints none of this.
6. **The `next:` line.**

Findings are sorted by path, line, column and rule id; the other lists by rule id. The same results from the
engines give the same output. A silenced finding is counted on the counts line and not listed.

## `tm check --json`

The run above, as `--json` prints it (on one line; wrapped here to read):

```json
{
  "counts": {
    "errors": 1, "failed": 1, "findings": 1, "not_available": 1,
    "quality": 1, "security": 0, "silenced": 1, "warnings": 0
  },
  "exceptions_in_force": 1,
  "exit": 1,
  "failed": [
    {
      "code": null,
      "engine": "clippy",
      "rule": "clippy-warnings",
      "stderr_tail": "clippy param alow: a clippy rule reads only the `allow` param"
    }
  ],
  "findings": [
    {
      "col": 17,
      "engine": "ruff",
      "fix": "Compare with `is None`.",
      "kind": "quality",
      "line": 2,
      "owner": "payments",
      "path": "src/new.py",
      "rule": "no-none-compare",
      "severity": "error",
      "what": "Comparison to `None` should be `cond is None`"
    }
  ],
  "next": "fix the findings above, or add an approved [[exception]] to /path/to/temperare.toml",
  "not_available": [
    { "engine": "tessera", "rule": "file-erosion", "why": "Tessera's slop records are not defined yet" }
  ],
  "silenced": [
    {
      "exception": {
        "approved": "who approved it",
        "path": "legacy.py",
        "reason": "the legacy module is rewritten in 2.0",
        "rule": "no-none-compare"
      },
      "finding": {
        "col": 17,
        "engine": "ruff",
        "fix": "Compare with `is None`.",
        "kind": "quality",
        "line": 2,
        "owner": "payments",
        "path": "legacy.py",
        "rule": "no-none-compare",
        "severity": "error",
        "what": "Comparison to `None` should be `cond is None`"
      }
    }
  ]
}
```

| Field | Meaning |
|---|---|
| `findings` | The kept findings: covered by no exception and, with `--base`, on a new line |
| `silenced` | Each finding an exception covered, with that exception |
| `exceptions_in_force` | How many exceptions the rule file holds, used in this run or not. Always present |
| `not_available` | Each rule that could not be checked: `rule`, `engine`, `why` |
| `failed` | Each rule that failed: `rule`, `engine`, `code` (the engine's exit code, or `null` when it had none or was not run), `stderr_tail` (the last 20 lines of what explains the failure: the engine's stderr, or Temperare's own reason when the engine printed none or was not run) |
| `base` | The full id of the commit `--base` named. Present only with `--base` |
| `counts` | The numbers below |
| `exit` | The exit code of this run |
| `next` | The text of the `next:` line |

Read fields by name, not by position.

### A finding

| Field | Meaning |
|---|---|
| `rule` | The rule's `id` |
| `engine` | `clippy`, `swiftlint`, `ruff`, `ast-grep` or `tessera` |
| `kind` | `quality` or `security`, from the rule |
| `severity` | `error` or `warning`, from the rule |
| `owner` | Who owns the file, from the rule file. Left out when it names none |
| `path` | The file, relative to the rule file's folder, with `/` between its parts |
| `line` | The line, counted from 1 |
| `col` | The column, counted from 1. Left out when the engine gives none |
| `what` | The engine's own message |
| `fix` | The rule's fix text |

Every engine's findings have this one shape.

### An exception, inside `silenced`

`rule`, `path`, `reason`, `approved`, and `what` when the exception has one.

### `counts`

| Field | Counts |
|---|---|
| `findings` | The kept findings |
| `errors`, `warnings` | The kept findings of each severity |
| `quality`, `security` | The kept findings of each kind |
| `silenced` | The findings an exception covered |
| `not_available` | The rules that were not available |
| `failed` | The rules that failed |

## Exit codes

The exit codes of `tm check`:

| Code | Meaning |
|---|---|
| 0 | Nothing to refuse: no finding of severity `error`, every rule ran. Warnings alone end here |
| 1 | At least one kept finding of kind `quality` and severity `error` |
| 2 | Nothing was checked: the command was used wrongly, there is no rule file, the rule file has a problem, a path is outside the rule file's folder, or `--base` does not name a commit |
| 3 | At least one rule was not available |
| 4 | The only kept findings of severity `error` are of kind `security`, and every rule ran |
| 5 | At least one rule failed: its engine gave no answer that can be read, or the rule was refused before its engine ran (see [`ENGINES.md`](ENGINES.md#not-available-and-failed)) |

The first that applies wins, in this order: 2, 1, 5, 3, 4, 0.

What that order means for a caller:

- **0 and 4** mean every rule ran. Nothing was unavailable and no engine failed.
- **1** says a quality error was found. It does not say every rule ran: a run with a quality error and a missing
  engine ends with 1. Read `counts.not_available` and `counts.failed`, or the counts line.
- **3 and 5** mean the code was not fully checked. They are never "clean".
- **4** never refuses. A security finding is reported with its severity, its place and its owner, and what happens
  next is the caller's decision.

`tm rules` exits 0 when the rule file is good, whatever the engines' availability, and 2 when it is not. `tm help`
and `tm --version` exit 0.

`tm install` and `tm uninstall` have their own three codes, and 1 does not mean a finding there:

| Code | Meaning |
|---|---|
| 0 | Done. A step that was skipped, such as a `tm` link another program owns or a harness that was not found, is still 0; a line says what was skipped |
| 1 | A step failed, or the install record could not be read or written. stderr gives the cause and a `next:` line |
| 2 | Used wrongly: an unknown flag or value, `HOME` not set or not an absolute path, or the running binary could not be found |

`tm uninstall` with no install record removes nothing, says so, and exits 0.

## The `next:` line

The last line of text, and `next` in JSON, says what to do, by exit code:

| Exit | Text |
|---|---|
| 0 | `nothing to do` |
| 1 | `fix the findings above, or add an approved [[exception]] to <rule file>` |
| 2 | The fix for the error |
| 3 | `install <engine>, <engine> (or run without those rules)`, naming each engine that was not available |
| 4 | `raise the security findings with their owner; work goes on` |
| 5 | `see the engine output above` |

For exit 3 the engines are named as a rule file names them (`clippy`, `swiftlint`, `ruff`, `ast-grep`), each once.
Tessera is never among the engines to install, because installing it changes nothing today:

- only Tessera rules were not available: `Tessera's checks are not available yet (run without those rules)`;
- Tessera rules and others: `install ruff; Tessera's checks are not available yet (or run without those rules)`.

Only an engine missing from PATH is named to install. Any other reason is given as `<engine>: <reason>`, each once.
A clippy rule in a folder with no `Cargo.toml` at or above it gives `clippy: no Cargo.toml at or above the rule
file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)`; the
`not available:` line of that rule keeps the reason.

For exit 5 the output meant is the `stderr of <rule> (<engine>):` lines of the text form, which are
`failed[].stderr_tail` in the JSON.

## New findings only: `--base`

With `--base <rev>`, a finding is kept only when its line was added since that commit:

- a line that differs between the commit and your working tree, committed or not;
- any line of an untracked file that git does not ignore;
- any line of a file under an untracked nested repository.

A finding on an older line is dropped before exceptions are applied, so it is neither kept nor silenced nor
counted. A `--base` that names no commit is exit 2, never an empty result.

## Errors that stop a run, as JSON

A usage error inside `tm check --json` or `tm rules --json` (no rule file, not in a git repository, a path outside
the rule file's folder, a `--base` that names no commit) prints the two lines on stderr and this on stdout:

```json
{"error":"not in a git repository: /path/to/folder","exit":2,"next":"Run tm inside a git repository, or pass --rules <path>."}
```

A rule file with problems prints, with `--json`:

```json
{"exit":2,"next":"fix each problem listed for /path/to/temperare.toml","problems":[{"col":10,"fix":"Use one of: clippy, swiftlint, ruff, ast-grep, tessera.","line":5,"what":"unknown engine \"pylint\""}],"rule_file":"/path/to/temperare.toml"}
```

Without `--json` it prints one line per problem, `file:line:col  what  fix`, then the `next:` line. See
[`RULE_FILE.md`](RULE_FILE.md#problems).

## `tm rules`

```
rule file: /path/to/temperare.toml
rules (3):
  no-none-compare  ruff  quality  error  paths: all files  available
  git-only-in-deploy  ast-grep  quality  error  paths: **/*.rs  exclude: **/generated/**  outside kits: deploy  available
  file-erosion  tessera  quality  error  paths: all files  not available: Tessera's slop records are not defined yet
exceptions in force (1):
  no-none-compare  legacy.py  what: Comparison to  approved: who approved it  reason: the legacy module is rewritten in 2.0
next: nothing to do
```

Each rule line is the id, the engine, the kind, the severity, the scope and the availability. The scope is the
rule's `paths` (`all files` when it has none), then `exclude:`, `kits:` and `outside kits:`, each only when the
rule has it. Availability is a look on `PATH`; no engine is run. Each exception line is the rule, the path, `what:`
when the exception has one, who approved it and the reason.

`tm rules --json` prints one object:

| Field | Meaning |
|---|---|
| `rule_file` | The rule file that was read |
| `rules` | Each rule: `id`, `engine`, `kind`, `severity`, `check`, `paths`, `exclude`, `kits`, `outside_kits` (each of the four an array, empty when the rule has none), `available` (true or false) and `why` (the reason when not available, else `null`) |
| `exceptions` | Each exception: `rule`, `path`, `reason`, `approved`, and `what` when it has one (left out otherwise) |
| `exceptions_in_force` | How many exceptions there are |
| `exit`, `next` | As for `tm check` |

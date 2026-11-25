# Engines

An engine is a program Temperare runs to do the reading. Temperare parses no source code, links no engine, and
takes its findings only from what the engine prints (detekt writes its report to a file `tm` names, which `tm`
reads). One read of the code is its own: for a ShellCheck rule it reads the first line of a
file with no extension, to tell a shell script by its `#!` line ([shellcheck](#shellcheck)). For some engines it also
reads the repository's config for that engine, to find a setting that would hide findings outside the rule file; each
engine's section says which files. This page says exactly
what is run for each one, what `check` means to it, and what is "not available" and what is "failed".

The engines, in the order of this page: [clippy](#clippy), [swiftlint](#swiftlint), [ruff](#ruff),
[ast-grep](#ast-grep), [shellcheck](#shellcheck), [git](#git), [pylint](#pylint), [clang-tidy](#clang-tidy),
[golangci-lint](#golangci-lint), [gitleaks](#gitleaks), [cargo-audit](#cargo-audit), [vulture](#vulture),
[periphery](#periphery), [eslint](#eslint), [mypy](#mypy), [pmd](#pmd), [detekt](#detekt), [rubocop](#rubocop),
[phpstan](#phpstan), [dart](#dart), [actionlint](#actionlint), [sqlfluff](#sqlfluff), [hadolint](#hadolint),
[yamllint](#yamllint), [vale](#vale), [stylelint](#stylelint), [markdownlint](#markdownlint) and
[tessera](#tessera).

## What holds for every engine

- **Found on `PATH`.** An engine is available when the programs it needs are on the `PATH` that `tm` was started
  with. `tm rules` shows the answer per rule without running anything.
- **Run as a program, never through a shell.** Each argument is passed as one argument, as written.
- **A rule's value is passed as a value, not as an option.** A `check`, `config` or `params.allow` value that
  begins with `-` is a problem in the rule file, and nothing runs. Each value reaches the engine joined to its own
  flag, as in `--select=<check>`, so the engine reads it as that flag's value. Where an engine has no such flag, the
  value goes inside text `tm` writes (ESLint's `--rule` JSON, yamllint's `-d` config, Vale's `--filter`, detekt's
  config layer), and its `check` is held to a narrow set of characters first, so it cannot end that text.
- **One run per rule, or one shared run.** Rules share one run of their engine only when that run gives each of
  them what it would get alone: the same engine, the same `config` and the same `params`, no param Temperare writes
  to a temporary file or folder, and the same check or checks the engine reads together. When the two checks
  differ: for clippy, neither check is `warnings`. For ruff, the codes do not overlap, neither selects `RUF100`, and no pair ruff
  calls incompatible is split. For SwiftLint, neither check is `superfluous_disable_command`. For ast-grep, the
  same rule file with no params. Only those four engines share. A rule of any other engine runs alone: pylint,
  clang-tidy, golangci-lint, gitleaks, cargo-audit, vulture, Periphery, ShellCheck, git, ESLint, mypy, PMD, detekt,
  RuboCop, PHPStan, dart, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint, markdownlint and Tessera. Each
  rule still keeps only its own check's
  findings, in its own paths and with its own exceptions. A rule that is refused, or
  that covers no file ([below](#the-order-a-rule-settles-in)), runs nothing and joins no shared run.
- **The order of runs.** A rule joins the first group, in rule-file order, whose rules can all share with it. The
  groups run one after another, in the order of each group's first rule, and the rule runs' results are kept in
  rule-file order; the report sorts findings as OUTPUT.md describes.
- **Run over the rule file's folder.** The engine checks the whole folder (clippy: the Cargo package that holds
  it; ShellCheck: every script `tm` lists there; clang-tidy: the C-family sources `tm` lists there that the rule
  covers). mypy, PMD, detekt, PHPStan, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint and markdownlint
  are likewise handed the files of their kind that git lists there and the rule covers, never a file git ignores.
  ESLint, RuboCop and dart are handed the folder, and their own config decides which files they read.
  Temperare then keeps the findings that are inside that folder, in the files the rule covers (its `paths`,
  `exclude`, `kits`, `outside_kits`) and, when you gave paths on the command line, in those paths. Paths on the
  command line narrow the result; they do not make the engines do less work.
- **Its version is read once per check.** Before the rules run, each engine named by the rules this check runs is
  asked its version once, in the rule file's folder, when it is available: `cargo clippy --version`,
  `swiftlint version`, `ruff --version`, `ast-grep --version`, `shellcheck --version`, `git --version`,
  `pylint --version`, `clang-tidy --version`, `golangci-lint --version`, `gitleaks version`,
  `cargo-audit --version`, `vulture --version`, `periphery version`, `eslint --version`, `mypy --version`,
  `pmd --version`, `detekt --version`, `rubocop --force-default-config --version`, `phpstan --version`,
  `dart --suppress-analytics --version`, `actionlint -version`, `sqlfluff --version`, `hadolint --version`,
  `yamllint --version`, `vale --version`, `stylelint --version`, `markdownlint-cli2 --help`. RuboCop is asked with
  `--force-default-config` because a plain `--version` loads the folder's config first. markdownlint-cli2 is asked
  with `--help` because it has no version flag: `--version` would lint. Tessera is not asked. An engine is asked even
  when every rule that names it is refused or covers no file. The version is named in the reason of a check the
  engine does not know, and of a ShellCheck code, a detekt name, a hadolint code or a markdownlint rule that is not
  on `tm`'s list (below).
- **No time limit.** `tm` waits for an engine as long as it runs.
- **The engine's environment is yours.** Temperare adds nothing to it, except: for a clippy rule with `max_nesting` it
  sets `CLIPPY_CONF_DIR` for that run to a temporary folder holding `clippy.toml`, which clippy reads instead of the
  repository's own `clippy.toml`; for ShellCheck it removes `SHELLCHECK_OPTS`; for gitleaks it removes
  `GITLEAKS_CONFIG` and `GITLEAKS_CONFIG_TOML` ([gitleaks](#gitleaks)); for golangci-lint it keeps Go local
  ([golangci-lint](#golangci-lint)); for PMD it removes `CLASSPATH`; for detekt it removes `JAVA_TOOL_OPTIONS`,
  `JDK_JAVA_OPTIONS` and `_JAVA_OPTIONS`; for RuboCop it empties `RUBOCOP_OPTS` and points the proxy variables at
  a dead address ([rubocop](#rubocop)); for actionlint it removes `SHELLCHECK_OPTS`; for sqlfluff it sets `HOME` to
  the top of the repository and removes `XDG_CONFIG_HOME`, `PYTHONPATH` and `PYTHONUSERBASE`
  ([sqlfluff](#sqlfluff)); for hadolint it removes every `HADOLINT_*` variable ([hadolint](#hadolint)); and Vale's
  config probe runs without `PWD` ([vale](#vale)). It sets no `NODE_PATH` for ESLint, stylelint or
  markdownlint-cli2: each resolves its plugins as it does when you run it yourself.
- **The engine finds its own configuration,** as it does when you run it yourself, except where the rule says
  otherwise. A rule's `config` is passed to SwiftLint, ruff, pylint, golangci-lint, gitleaks, ShellCheck or
  clang-tidy as its config file. clang-tidy without a `config`, and ShellCheck without one, read no config file of the repository's
  (see [clang-tidy](#clang-tidy) and [shellcheck](#shellcheck)). vulture and Periphery never read one: a rule's
  `config` does not reach them, and `tm` points each at an empty config of its own ([vulture](#vulture),
  [periphery](#periphery)). A gitleaks config that declares an allowlist fails the rule ([gitleaks](#gitleaks)). A SwiftLint threshold passes a
  config file of its own, after the folder's `.swiftlint.yml` when there is one and the rule has no `config` (see
  [swiftlint](#swiftlint)). A clippy `max_nesting` replaces the repository's `clippy.toml` for that run, as above.
  A ruff threshold is layered over the rule's `config`, or over the config ruff finds when the rule has none, and
  turns nothing off. A rule's `config` is also passed to ESLint, RuboCop, sqlfluff and stylelint as their config
  file. mypy, hadolint and actionlint are always told which config to read, the rule's or the repository's own that
  `tm` finds, so none on your machine outside the repository is read. detekt, PHPStan, yamllint and Vale get a config
  `tm` writes for the run, which rests on the rule's `config` or the repository's own and turns off what would hide
  findings. A rule's `config` does not reach PMD or dart, and a markdownlint rule that names one fails
  ([markdownlint](#markdownlint)).
- **Exceptions outside the rule file.** The rule file is the only home for an exception, and many engines have
  ways of their own to silence a finding: a comment in the code, an ignore file, a list in their config. Where `tm`
  turns one off, fails the rule over one it cannot turn off, or has to leave one honoured, the engine's section
  says so.
- **The engine's exit code is not a failure by itself.** Linters exit non-zero when they find something. Temperare
  reads the output; each engine's section says which exits are an answer and which fail the rule.
- **The rule decides severity.** A finding's `severity`, `kind` and `fix` come from the rule in `temperare.toml`.
  The engine's own severity for it is not read.

## Not available, and failed

Neither is ever "clean".

**Not available** (exit 3) means the rule could not be run at all:

| `why` | When |
|---|---|
| `<program> not found on PATH` | The engine's program is not installed, or not on `PATH`. A clippy rule names the first of `cargo-clippy` and `clippy-driver` that is missing. A ShellCheck rule names `git` when ShellCheck is there and git, which lists its scripts, is not. A golangci-lint rule names `go` when golangci-lint is there and go is not. A clang-tidy rule names `git` when clang-tidy is there and git, which lists its sources, is not. A Periphery rule names `swift` when Periphery is there and swift, which builds the package, is not. A PHPStan rule names `php` when phpstan is there and php, which runs it, is not. An actionlint rule whose check is `shellcheck` or `pyflakes` names that program when actionlint is there and it is not. A markdownlint rule names `markdownlint-cli2`, the program it runs. A rule of an engine `tm` lists the files for with git (mypy, PMD, detekt, dart, PHPStan, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint, markdownlint) names `git` when the engine is there and git is not |
| `no Package.swift at or above the rule file's folder: Periphery builds the project, and tm runs it only on a Swift package` | A Periphery rule outside a Swift package in its repository, such as one in an Xcode project ([periphery](#periphery)) |
| `<subject> depends on <identities>, which tm would have to fetch: tm never fetches packages, so Periphery cannot build it here` | A Periphery rule on a package that, or one of whose local packages, names a source-control or registry dependency or a binary target with a `url` ([periphery](#periphery)) |
| `<subject> depends on the local package <path>, which does not resolve: ...` or `..., outside the repository: ...` | A Periphery rule on a package that depends on a local package `tm` cannot read, or that is not the repository's ([periphery](#periphery)) |
| `no compile database (compile_commands.json) or compile flags (compile_flags.txt) for <file> or any folder above it, so clang-tidy would check it without flags` | A clang-tidy rule on a source with neither file at or above it ([clang-tidy](#clang-tidy)) |
| `no Cargo.toml at or above the rule file's folder` | A clippy rule in a folder that is not inside a Cargo package |
| ``<engine> <version> does not know the check `<check>`: <what it said>`` | The installed clippy, SwiftLint, ruff, pylint, clang-tidy, golangci-lint, gitleaks, ESLint, mypy, PMD, detekt, RuboCop, PHPStan, dart, actionlint, sqlfluff, yamllint, Vale or stylelint does not know the rule's `check`: a name it does not have, often because it is older than the rule, or for ESLint and stylelint because the repository's config does not load the plugin that has it. For cargo-audit, an advisory id its local advisory database does not hold (`<check> is not in the advisory database at <db>`). `<version>` is `(version unknown)` when the version could not be read |
| ``no advisory database at <db>: fetch it with `cargo audit update-db --db <db>` (tm never fetches it)`` | A cargo-audit rule when the advisory database has not been fetched ([cargo-audit](#cargo-audit)). With neither `CARGO_HOME` nor `HOME` set the reason is `no advisory database: neither CARGO_HOME nor HOME is set` |
| `shellcheck <version>: <check> is not in temperare's list of shellcheck codes (taken from shellcheck <list version>)` | A ShellCheck rule whose code is not on the list built into `tm`: the 510 codes of ShellCheck 0.11.0, so `<list version>` is `0.11.0` ([below](#shellcheck)) |
| `detekt <version>: <name> is not in temperare's list of detekt codes (taken from detekt 1.23.8)` | A detekt rule whose check names a rule or rule set that is not among detekt 1.23.8's 214 built-in rules and 9 rule sets ([detekt](#detekt)) |
| `detekt needs type resolution for <rules>, which needs a classpath that tm does not give detekt, so they would report nothing` | A detekt rule whose check picks, by name or through its rule set, a rule that needs type resolution ([detekt](#detekt)) |
| `hadolint <version>: <check> is not in temperare's list of hadolint codes (taken from hadolint 2.15.1)` | A hadolint rule whose code is not one of hadolint 2.15.1's 71 `DL` codes on `tm`'s list. An `SC` code never is, and the why says so ([hadolint](#hadolint)) |
| `markdownlint <version>: <check> is not in temperare's list of markdownlint rules (taken from markdownlint 0.42.2)` | A markdownlint rule whose check is not the id or an alias of one of markdownlint 0.42.2's 53 rules. `<version>` is markdownlint-cli2's ([markdownlint](#markdownlint)) |
| ``<config> turns <rule> off with its `<key>` setting: tm runs markdownlint with the repository's own config, so the rule would report nothing`` | A markdownlint rule whose check a `.markdownlint.json`, `.yaml` or `.yml` that applies to its files turns off ([markdownlint](#markdownlint)) |
| `stylelint's config does not turn on <check> for any file the rule covers` | A stylelint rule whose check the repository's stylelint config turns on for none of the rule's files ([stylelint](#stylelint)) |
| `the rule names no config and its folder holds no phpstan.neon, phpstan.neon.dist or phpstan.dist.neon, nor their dot-prefixed forms: tm runs phpstan with the repository's own config` | A PHPStan rule with no `config`, in a folder with no PHPStan config ([phpstan](#phpstan)) |
| `no pubspec.yaml at or above the rule file's folder: dart analyze checks a Dart package, and tm runs it only in one` | A dart rule outside a Dart package in its repository ([dart](#dart)) |
| ``<file> does not enable the lint `<check>`: dart runs a lint only where analysis_options.yaml enables it`` | A dart rule whose check is a lint that an `analysis_options.yaml` governing the folder does not enable. With no such file the why starts `no analysis_options.yaml governs the rule's folder, so nothing enables the lint` ([dart](#dart)) |
| ``dart cannot resolve `<uri>`: the package's dependencies are not fetched (no .dart_tool/package_config.json); run `dart pub get` once in the package, since tm never fetches packages`` | A dart rule on a package whose dependencies were never fetched, when dart cannot find a `package:` URI. An `include:` of an options file that cannot be resolved has a like why ([dart](#dart)) |
| ``no dialect is set in the repository's sqlfluff config: set `dialect` under `[sqlfluff]` in its .sqlfluff`` | A sqlfluff rule in a repository whose sqlfluff config sets no dialect ([sqlfluff](#sqlfluff)) |
| `Vale finds no config (.vale.ini, _vale.ini, vale.ini, .vale or _vale) at or above the rule file's folder: tm runs Vale only with the repository's own config` | A Vale rule with no `config`, where Vale finds no config ([vale](#vale)) |
| `Vale would read the config <file>, outside the repository: ...` | A Vale rule where the config Vale finds, or one it loads, is outside the repository ([vale](#vale)) |
| ``no BasedOnStyles in <config> names the style `<style>`: tm never turns on a Vale style the repository did not choose`` | A Vale rule whose check's style the repository's Vale config does not use ([vale](#vale)) |
| `reports only what a change adds; run with --base` | A git rule, and an ast-grep rule in count mode, run without `--base` ([git](#git), [ast-grep](#ast-grep)) |
| `Tessera's slop records are not defined yet` | A rule with `engine = "tessera"` ([below](#tessera)) |

A rule that is refused, or that covers no file, is settled before any of these is looked at
([the order a rule settles in](#the-order-a-rule-settles-in)).

An engine too old for a rule is not available rather than failed: the fix is a newer engine, or a `check` this one
knows. ast-grep, git and Tessera have no such case: an ast-grep `check` is your own YAML, which ast-grep either reads
or rejects. ShellCheck, detekt, hadolint and markdownlint are not asked whether they know a name: none of them says
so when it does not, so the name is looked up in a list built into `tm`. (detekt does say so for a rule on `tm`'s
list that the installed detekt lacks.) dart is asked in a run of its own, in a temporary package, before the
check.

**Failed** (exit 5) means the rule gave no answer that can be trusted. Either the engine gave none: it was found
and could not be started, it printed something that is not its findings format, or a clippy build did not succeed.
Or Temperare refused the rule before running the engine, because the result could not be matched back to the rule
or the rule's own files could not be used:

- a clippy `check` that is a lint group or a `rustdoc::` lint;
- a SwiftLint `check` of `custom_rules`;
- a ruff `check` with a part that is not a rule code, a code prefix or `ALL`;
- an ast-grep `check` that is not a readable file inside the rule file's folder (a baseline set's ast-grep rule
  apart: its YAML is inside `tm` and no file is read for it, see [ast-grep](#ast-grep));
- a ShellCheck `check` that is not one code;
- a git `check` other than `test_ratio`;
- a pylint `check` that is not one message id, or that is an old id pylint renamed;
- a clang-tidy `check` that is not one check name;
- a golangci-lint `check` with a part that is not a linter name, or that is `typecheck`;
- a gitleaks `check` with an empty part, or a part that is `ALL`;
- a cargo-audit `check` that is not an advisory id or a kind, or that is `yanked`;
- a vulture `check` that is not one vulture code;
- a Periphery `check` that is not one of Periphery's result hints or `all`;
- an ESLint `check` that is not one rule id;
- a mypy `check` that is not one error code;
- a PMD `check` with a part that is not a Java category or rule reference;
- a detekt `check` with a part that is not a well-formed name;
- a RuboCop `check` with a part that is not a full cop name;
- a PHPStan `check` that is not a level;
- a dart `check` that is not one lower-case code;
- an actionlint `check` that is not shaped like a kind;
- a sqlfluff `check` that is not one rule reference;
- a hadolint `check` that is not one code;
- a yamllint `check` that is not one rule id;
- a Vale `check` that is not `Style.Rule`;
- a stylelint `check` that is not one rule name;
- a markdownlint `check` that is not shaped like a rule name;
- `params` that cannot be used.

Or the repository's own config for the engine holds an exception `tm` cannot turn off, or is in a form `tm` cannot
read, so the rule could read clean over findings it hides: a mypy config that silences the check, a RuboCop todo
file or `Exclude`, a dart `analysis_options.yaml` with an `exclude` or an `ignore`, an actionlint config with an
`ignore`, a hadolint config with an `ignored` list, a stylelint config that turns the check off for some files, a
markdownlint-cli2 options file. Each engine's section has its cases.

A rule refused before its engine ran has no exit code. The reason is printed under the counts line of the text
output (`stderr of <rule> (<engine>):`) and carried in `failed[].stderr_tail` of the JSON output: the last 20 lines
of the engine's stderr, or Temperare's own reason when the engine printed none or was not run.

### The order a rule settles in

Each rule is settled by the first of these that applies:

1. **Refused.** `tm` refuses the rule from the rule file alone, reading no other file and running nothing: a clippy
   `rustdoc::` lint, SwiftLint's `custom_rules`, a ruff `check` with a part that is not a code, a ShellCheck `check`
   that is not one code, a git `check` other than `test_ratio`, a pylint `check` that is not one message id or is an
   old one, a clang-tidy `check` that is not one check name, a golangci-lint `check` that is not linter names, a
   gitleaks `check` with an empty or `ALL` part, a cargo-audit `check` that is not an advisory id or a kind, a
   vulture `check` that is not one vulture code, a Periphery `check` that is not a result hint, an ESLint, mypy,
   PMD, detekt, RuboCop, PHPStan, dart, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint or markdownlint
   `check` that is not shaped as its section says, an
   ast-grep `check` that is an absolute path or
   whose `..` climbs above the rule file's folder at any step (read from the text alone), a
   param the engine does not take, a param with a bad value, or a threshold out of range or with another check
   (ast-grep's placeholder params apart: see below). The rule is failed (exit 5).
2. **No file.** The rule covers none of the files git lists in the rule file's folder, or it is a ShellCheck rule
   and covers none of the scripts there, or it is a cargo-audit rule and the folder has no `Cargo.lock` the rule
   covers, or it is a clang-tidy rule and covers none of the C-family sources there, or it is a rule of another
   engine `tm` picks the files for (mypy, PMD, detekt, dart, PHPStan, actionlint, sqlfluff, hadolint, yamllint, Vale,
   stylelint, markdownlint) and covers none of the files of that engine's kind there. The rule runs nothing
   ([below](#a-rule-that-covers-no-file)). ESLint and RuboCop are the exception: their own config decides which
   files they read, so a rule of theirs that covers some file is settled at step 5, from what the engine says it
   read.
3. **Needs a base.** A rule with `only_with_base = true`, run without `--base`, settles here, whatever its engine.
   Its engine is not looked up on `PATH` and runs no check for it, and when it shares a run with other rules its
   check is left out of that run. It is not applicable: counted in the text, listed under `not_applicable` in JSON
   ([`OUTPUT.md`](OUTPUT.md#new-findings-only---base)). An engine found on `PATH` is still asked its version for
   the engines line.
4. **Not available.** A program the engine needs is not on `PATH`, or it is Tessera. (What else an engine needs
   is looked for in the run, at step 5: cargo-audit's advisory database, a Swift or Dart package, a PHPStan or
   Vale config, a compile database, a name on `tm`'s list.)
5. **The run.** What the engine's run comes to: findings, none, not available or failed.

So a refused rule is failed (exit 5) whether or not its engine is installed, and installing the engine would not help it. Its engine is still asked its version, as above. The one exception is an `only_with_base` rule run without `--base`, which is listed under `not_applicable` instead, as any such rule is.

Some refusals need more than the rule file, so they are made in the run, at step 5, after steps 2 to 4; the ast-grep YAML's own disk checks are listed in its section:

- a clippy lint group, which is told from a lint by `clippy-driver -W help`;
- an ast-grep rule file that is missing or cannot be read, or that leads outside the rule file's folder through a
  symlink;
- an ast-grep placeholder param that cannot be used (a `{{name}}` with no param, a param with no `{{name}}`, a
  param that is neither a string nor a list of strings), which needs the YAML read;
- a gitleaks rule whose folder carries an exception outside the rule file: an allowlist in the config gitleaks
  reads, a config that does not parse, or a `.gitleaksignore` that lists anything ([gitleaks](#gitleaks));
- a mypy, RuboCop, dart, actionlint, hadolint, stylelint or markdownlint rule whose repository config holds an
  exception `tm` cannot turn off, or is in a form `tm` cannot read (each in its section);
- a markdownlint rule that names a `config` ([markdownlint](#markdownlint)).

Such a rule fails when its engine is installed and it covers a file. With the engine missing it is not available,
and when it covers no file it runs nothing.

Outside a git repository a ShellCheck rule that finds ShellCheck fails on git's listing of its scripts, before its
code is looked up in `tm`'s list ([shellcheck](#shellcheck)). So does a rule of any other engine `tm` lists the files
for with git, once its engine is found: clang-tidy, mypy, PMD, detekt, PHPStan, actionlint, sqlfluff, hadolint,
yamllint, Vale, stylelint and markdownlint, and dart, which asks git for the top of the repository. ESLint and
RuboCop rules run outside a git repository.

### A rule that covers no file

Before any rule runs, `tm` asks git for the files of the rule file's folder: the tracked files that are in the work
tree, and the untracked files git does not ignore. A rule that covers none of them, by its `paths`, `exclude`,
`kits` and `outside_kits`, runs nothing. Its engine runs no check for it, and whether the engine is on `PATH` does
not change its result (the engine is still asked its version for the engines line, below). A ShellCheck rule
that covers a file is settled the same way when it covers none of the scripts in the folder, and so is a
cargo-audit rule when the folder holds no `Cargo.lock` (a regular file, not a symlink) or the rule does not cover
it ([cargo-audit](#cargo-audit)), and so is a clang-tidy rule that covers a file but none of the C-family sources
git lists there ([clang-tidy](#clang-tidy)), whether or not clang-tidy is installed.

The same holds for every other engine `tm` picks the files for. A rule that covers files, but none of the kind its
engine checks among those git lists, runs nothing, whether or not the engine is installed:

| Engine | The files it checks |
|---|---|
| `mypy` | `.py` and `.pyi` |
| `pmd` | `.java` |
| `detekt` | `.kt` and `.kts` |
| `dart` | `.dart` |
| `phpstan` | `.php` |
| `actionlint` | `.yml` and `.yaml` files directly in the rule file's folder's `.github/workflows` |
| `sqlfluff` | names ending `.sql`, `.sql.j2`, `.dml`, `.ddl` or `.pkb`, in any case |
| `hadolint` | `Dockerfile`, `Containerfile` and `<name>.Dockerfile` |
| `yamllint` | names ending `.yaml` or `.yml`, and `.yamllint` |
| `vale` | the markup and code formats listed under [vale](#vale) |
| `stylelint` | `.css`, in any case |
| `markdownlint` | `.md` and `.markdown` |

Each is a regular file, never a symlink.

**ESLint and RuboCop are settled after their run.** Their own config decides which files they read, so `tm` cannot
list those files beforehand. A rule of theirs that covers none of the files git lists runs nothing, as above. One
that covers some file needs its engine on `PATH` and is run; when none of the files the engine says it read is one
the rule covers, the rule covers no file, with the why
`ESLint linted no file in the rule's paths that git lists (its config decides which files it lints)` or
`no file rubocop inspects is in the rule's paths`. So with ESLint or RuboCop missing, such a rule is not available.
A PHPStan rule is settled so too when its config's `stubFiles` name every file handed over
([phpstan](#phpstan)).

These are not among those files, so a rule that covers only such files covers no file: a file git ignores, a
tracked file deleted from the work tree, a skip-worktree file whether or not it is there, and the files of an
untracked nested repository.

The report shows such a rule by where it comes from:

- **One of the repository's own rules covers nothing.** It is on a `covers nothing: <rule> (<engine>)` line in the
  text, under `covers_nothing` in JSON (`rule`, `engine`), and in the `covers nothing` count, and it makes the exit
  6 when nothing ranks above it ([`OUTPUT.md`](OUTPUT.md#exit-codes)).
- **A baseline set's rule is not applicable.** The text counts it in the `not applicable` count and gives it no
  line; JSON lists it under `not_applicable` with the why `its paths match no file in the rule file's folder`, or
  `no file its engine checks` for a ShellCheck rule that covers files but no script, a cargo-audit rule with no
  `Cargo.lock` it covers, a clang-tidy rule that covers files but no C-family source, or a rule of an engine in the
  table above that covers files but none of that kind; an ESLint or RuboCop rule settled after its run has the why
  given above. It does not change the exit code.

A refused rule is not such a rule: it is settled first and is failed, whatever it covers. A rule with
`only_with_base = true`, run without `--base`, is not applicable for want of `--base` whatever it covers,
as any such rule is. A check whose only rule is one of its own and covers no file prints
`covers nothing: <rule> (<engine>)`, then
`0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed, 0 not applicable, 1 covers nothing, 0 expired`
and `next: fix the paths of each rule that covers nothing above, or remove it from <rule file>`, and exits 6. (The
example line shows no exceptions in force; a rule file with exceptions shows their count.) That is so whether its
engine is installed or not. The one difference is the engines line: an engine found on `PATH` is still asked its
version and listed, and a missing one is not. In `--json`, `saved.counted` holds the rule with `chars` 0.

`tm rules` shows each rule's scope. It runs no engine, so it never lists an ESLint or RuboCop rule as covering no
file unless the rule covers none of the files git lists.

When git cannot list the files (the rule file's folder is in no git repository, or git is missing or fails), no
rule is settled by its paths, its scripts, its C-family sources or the other files `tm` picks for an engine, and
each goes on to steps 3 to 5. A cargo-audit rule is the one
exception: when the rule file's folder is still found, `tm` looks for its `Cargo.lock` on disk, so a rule whose
folder has no `Cargo.lock` it covers still settles here.

## clippy

- **Available when** `cargo-clippy` and `clippy-driver` are both on `PATH`. clippy is run through `cargo`, so
  `cargo` must be there too.
- **Where:** the nearest folder at or above the rule file's folder that holds a `Cargo.toml`. For a rule file inside
  one member of a workspace, that is the member, so only that crate is linted.
- **What runs,** all three in that folder:

  ```
  clippy-driver -W help
  cargo locate-project --workspace --message-format plain
  cargo clippy --all-targets --message-format=json --quiet -- --warn=<check> [--allow=<lint>]...
  ```

  The first lists the lints and lint groups of the toolchain in use; `check` is looked up in that list before
  anything is built. The second names the workspace root, which is where cargo prints file paths from.
  `--all-targets` lints tests, examples and benches too, so test code is checked like any other. `--warn=<check>`
  turns the lint on even if it is off by default. Each lint in `params.allow` becomes `--allow=<lint>` after it, so
  an allowed lint stays off even under `warnings`.
- **`check`:**
  - one lint that `clippy-driver -W help` lists, such as `clippy::bool_comparison` or the compiler's own
    `unused_variables`. `-` and `_` are read alike, so `clippy::bool-comparison` is the same lint. Messages with
    that code are the findings, at any level;
  - or `warnings`: every warning-level message that has a lint code is a finding, the compiler's own lints
    included. The finding's message starts with the code, such as
    `clippy::bool_comparison: equality checks against true are unnecessary`, so it names the lint.
- **Failed before clippy runs** when `check` is any of these, so a rule that could match nothing never reads as
  clean:
  - a lint group, such as `clippy::all`, `clippy::pedantic` or `unused`. clippy reports each message under its own
    lint's code, so a group's findings cannot be told from other warnings. Name each lint, or use `warnings`;
  - a `rustdoc::` lint, which clippy never runs.

  A name the toolchain does not list (a misspelt lint, one renamed or removed, or one newer than this toolchain)
  makes the rule not available before clippy runs, with the reason
  ``clippy <version> does not know the check `<check>`: it is not a lint this toolchain lists (clippy-driver -W help)``.
  A rule's own `allow` cannot hide it.

  The rule also fails when `clippy-driver -W help` cannot be read or lists no lint or no group, and when clippy
  itself reports that it did not take `check` as named.
- **`params`:** `allow`, a list of strings; and `max_nesting`, a whole number from 1 to 1000, read only with check
  `clippy::excessive_nesting` (tm writes it as `excessive-nesting-threshold` in `clippy.toml` for that run). Any other
  param, or a bad value, fails the rule before clippy runs.
- **`config`:** not used.
- **A finding** is placed at the message's primary span. A message cargo prints once per target is reported once.
- **Failed when** the build does not succeed. A build that failed has not linted everything, so it is not an
  answer.
- **It builds your code.** `cargo clippy` compiles the crate and what it depends on. Temperare sets no target
  folder, so cargo builds where it normally does, the repository's own `target/`. The first run takes as long as a
  build, and a build runs the crate's build scripts and procedural macros.

## swiftlint

- **Available when** `swiftlint` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  swiftlint lint --reporter json --quiet --only-rule=<check> [--config=<config>] [--config=<threshold file>]
  ```
- **`check`:** one SwiftLint rule identifier, such as `force_cast`. Violations with exactly that identifier are the
  findings. For a custom rule, write the custom rule's own identifier.
- **Failed** when `check` is `custom_rules`, before SwiftLint runs: under `--only-rule` that name runs no custom
  rule, so the rule could match nothing. Failed too when SwiftLint finds no Swift file to lint in the folder (it
  exits with an error instead of an empty result).
- **Not available** when SwiftLint says `check` is not a valid rule identifier: it does not know that rule, with the
  rule's `config` applied, so a custom rule the config defines is known. The reason quotes SwiftLint's own warning
  line, which names the identifier. SwiftLint exits 0 with no findings in that case, so its warning is the only sign
  that nothing was checked.
- **Paths under `/private` on macOS.** SwiftLint prints file paths with a leading `/private` removed; a finding is
  placed under the rule file's folder either way, so code under `/tmp` and `/var` is checked as anywhere else. A
  path that cannot be placed under the folder at all makes the rule Failed, never clean.
- **`config`:** passed as written, so a relative path is taken from the rule file's folder. Temperare does not read
  or copy the file.
- **`params`:** `max_complexity`, a whole number from 1 to 1000, read only with check `cyclomatic_complexity`; and
  `max_lines`, a whole number from 1 to 100000, read only with check `file_length`. At most one threshold per rule.
  The threshold is written to a temporary `.yml` file that sets the check's warning and error levels both to it, and
  that file is passed as the last `--config=`, so it wins over the rule's `config`. A rule with a threshold and no
  `config` passes the folder's own `.swiftlint.yml` first, when there is one, since an explicit `--config` would
  otherwise make SwiftLint skip it. No config of yours is changed, and the temporary file is removed after the run.
  A threshold out of range or with another check, and any other param, fail the rule before SwiftLint runs.
- **A finding** has no column when SwiftLint gives none, as for a violation of a whole line or file.

## ruff

- **Available when** `ruff` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  ruff check --no-cache --output-format json --select=<check> [--config=<config>] [--config=lint.mccabe.max-complexity=<n>] .
  ```

  `--no-cache` keeps ruff from leaving a `.ruff_cache` folder in the code it checks.
- **`check`:** a ruff selector made of rule codes:
  - a rule code, such as `E711`;
  - a code prefix, such as `E7` or `F`, for every rule whose code starts with it;
  - `ALL`, for every rule;
  - or several of these with commas and no spaces between them, such as `E711,F401`.

  A diagnostic is a finding when its code starts with one of the parts, or when a part is `ALL`.
- **Failed** when a part of `check` is anything else, such as a rule name (`none-comparison`) or a lower-case code,
  before ruff runs: Temperare could not tell which findings it selects. Failed too when ruff says a code `check`
  selects has been remapped to another, since its findings would carry the new code. Write the new code.
- **Not available** when ruff says a part of `check` is an unknown rule selector: a code this ruff does not have,
  often one newer than it. The reason quotes ruff's own line, which names the code.
- **`config`:** passed as written, so a relative path is taken from the rule file's folder. Temperare does not read
  or copy the file.
- **`params`:** `max_complexity`, a whole number from 1 to 1000, read only with check `C901`, passed as
  `--config=lint.mccabe.max-complexity=<n>` after the rule's `config`. ruff lays that over the config file, so the
  threshold wins over the same key set there, and nothing is written. A threshold out of range or with another check,
  and any other param, fail the rule before ruff runs.

## ast-grep

ast-grep is for rules about the shape of code that no stock linter states: a house rule.

- **Available when** `ast-grep` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  ast-grep scan --rule <a filled copy of your rule file> --json=stream
  ```
- **`check`:** the path of an ast-grep rule file (YAML), relative to the rule file's folder. Every match of that
  file is a finding. One file can hold several rules, separated by `---`, for example one per language.
- **The YAML must be a file inside the rule file's folder.** The rule fails, before the scan runs, when `check`:
  - is an absolute path;
  - climbs above the rule file's folder through `..` at any step, read as text: each folder name goes down one, `..`
    up one, and the path is outside once it goes above the folder, even if a later part names the folder again
    (`../<this folder>/x.yml` fails);
  - resolves to a place outside the rule file's folder through a symlink;
  - names a file that is missing or cannot be read.

  The first two are refusals from the rule file alone, made before anything else
  ([the order a rule settles in](#the-order-a-rule-settles-in)), so they fail the rule whether or not ast-grep is
  installed. The last two need the disk, so they are made in the run.

  A symlink whose target is inside the rule file's folder is resolved by these checks, so it is read.

  On Unix more checks hold. The file is opened one path component at a time from `/` down through the rule file's
  folder, following no symlink, so the file read is the one inside the folder: if a component has become a symlink
  or something that is not a folder since the path was resolved, or the file is removed while it is read, the rule
  fails with `changed while it was opened`. A folder on that walk that cannot be opened (one you may pass through
  but not read) fails the rule, naming that folder. A YAML path that is not a regular file, such as a FIFO or a
  folder, fails the rule without waiting on it. And a file with more than one hard link fails the rule, wherever
  the other link is, because Temperare cannot tell where a file's other names are. On other systems only the path
  checks above apply.
- **A baseline set's rule carries its YAML inside `tm`.** For an ast-grep rule of a set the rule file takes
  ([`RULE_FILE.md`](RULE_FILE.md#baseline-sets)), `check` names the YAML inside the set, such as
  `ast-grep/tm-rust-suppression.yml`, and is not a path in your repository. Nothing is read from the rule file's
  folder for it: none of the file checks above is made, and a file of that name in your repository is not read.
  The rest is as for your own rule: the YAML's `{{name}}`s are filled from the rule's `params`, the filled copy is
  written to a temporary file, and `ast-grep scan` runs with it in the rule file's folder.
- **`params`:** the values for the placeholders in the YAML. Temperare reads your rule file, replaces each
  `{{name}}` with the param of that name, writes the result to a temporary file, runs ast-grep with it and removes
  it:
  - a string param goes in as written;
  - a list of strings goes in as its items joined by `|`, each escaped to match literally, ready for a `regex:`
    line.

  The rule fails, before the scan runs, when the YAML has a `{{name}}` with no param of that name, when a param
  is neither a string nor a list of strings, or when a param has no `{{name}}` in the YAML, so a misspelt param
  name cannot pass unnoticed. The reason for the last names the YAML's placeholders: with params `entity_type` and
  `entity_types` and a YAML that names only `{{entity_types}}`, it is
  ``param entity_type in rules/no-stored-bool.yml: the rule YAML has no {{entity_type}}; its placeholders are {{entity_types}}``.
- **Count mode: `count` and `shared_setup`.** These two params are not placeholders; they are taken out before the
  YAML is filled. `count = "setup-vs-tests"`, its only value, turns count mode on. `shared_setup`, a list of globs
  relative to the rule file's folder, names files that are never reported, and needs `count`. Any other `count`, a
  `shared_setup` without `count`, one that is not a list of strings, or a glob in it that does not compile, fails
  the rule before anything runs, whether or not ast-grep is installed.

  In count mode the YAML matches each file's root node and each test function, and a match is not a finding. A file
  the change adds lines to is reported when its non-blank lines outside the test functions outnumber those inside
  them: one finding per file, at its first added line, with no column and the message
  `<n> lines of setup, <m> lines of tests`. A file with no test function is not reported.

  Count mode needs the base commit, which `tm check --base <rev>` hands it. Without `--base`, a count-mode rule
  that gets as far as its run is not available, with the why `reports only what a change adds; run with --base`,
  and ast-grep is not run for it. With `only_with_base = true` and no `--base` the rule is not applicable instead,
  as any such rule is.
- **A YAML that ast-grep rejects** fails the rule. ast-grep has no "not available" case for an unknown `check`: the
  check is your own YAML, which ast-grep either reads or rejects.
- **`config`:** not used.
- **A finding's message** is the `message` of the ast-grep rule that matched, or that rule's `id` when it has no
  message.

A house rule with a list of names:

```toml
[[rule]]
id = "no-stored-bool"
engine = "ast-grep"
check = "rules/no-stored-bool.yml"
paths = ["**/*.rs"]
fix = "Store a bitmap field; expose the Bool as a computed accessor."
params = { entity_types = ["Order", "Invoice"] }
```

`rules/no-stored-bool.yml`:

```yaml
id: no-stored-bool-rust
language: rust
message: stored Bool on a persisted entity
severity: error
rule:
  kind: field_declaration
  has:
    field: type
    regex: ^bool$
  inside:
    kind: struct_item
    stopBy: end
    has:
      field: name
      regex: ^({{entity_types}})$
```

Before ast-grep runs, `{{entity_types}}` becomes `Order|Invoice`. A `bool` field on `Order` or `Invoice` is a
finding; one on any other struct is not.

"Named only inside one kit" is an ast-grep rule that matches the name, with `outside_kits` naming the kit that may
use it:

```toml
[kits.deploy]
paths = ["deploy/**"]

[[rule]]
id = "git-only-in-deploy"
engine = "ast-grep"
check = "rules/git-only-in-deploy.yml"
paths = ["**/*.rs"]
outside_kits = ["deploy"]
fix = "Run git through the deploy kit; only deploy starts git."
```

`rules/git-only-in-deploy.yml`:

```yaml
id: git-only-in-deploy
language: rust
message: runs git outside the deploy kit
severity: error
rule:
  pattern: Command::new("git")
```

## shellcheck

- **Available when** `shellcheck` is on `PATH`. `tm` lists the scripts with git, so git must be there too.
- **A list of codes built into `tm`.** ShellCheck never says it does not know a code: asked for one it does not
  have, it runs, matches nothing and reads clean. So a rule's code is looked up in a list of ShellCheck's codes that
  is built into `tm`: the 510 codes the ShellCheck 0.11.0 source emits. The list is copied from that source when
  `tm` is built and never fetched. A ShellCheck rule comes to one of these, in the order
  [a rule settles in](#the-order-a-rule-settles-in):
  - failed, when its `check` is not one code or it has a param;
  - nothing run, when git lists the scripts and it covers none of them;
  - not available, with `shellcheck not found on PATH`. This comes before any git error: when git cannot list the
    scripts, the rule goes on to ShellCheck's own check first, so with neither git nor ShellCheck installed the why
    is `shellcheck not found on PATH`;
  - with ShellCheck installed, not available, with `git not found on PATH`, when git is not installed and so cannot
    list the scripts; or failed, with git's own reason, when git cannot list them;
  - not available, when its code is not on the list, with
    ``shellcheck <version>: <check> is not in temperare's list of shellcheck codes (taken from shellcheck 0.11.0)``.
    `<version>` is the installed ShellCheck's, or `(version unknown)` when it could not be read; the last word is
    the list's version, so a code newer than ShellCheck 0.11.0 reads as a gap in `tm`'s list, not in your
    ShellCheck;
  - otherwise ShellCheck's run, below: findings, none, or failed.

  `tm rules` runs nothing, so it shows a ShellCheck rule as available whenever `shellcheck` is on `PATH`.
- **Where:** the rule file's folder, which must be inside a git repository.
- **`check`:** one ShellCheck code, `SC` then digits, such as `SC2086`. Anything else (`sc2086`, `2086`, two codes)
  fails the rule before anything runs, with the reason
  ``shellcheck check `<check>` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects``.
  A well-formed code that is not on the list makes the rule not available, as above; nothing runs for it, so a code
  `tm` does not list is never clean.
- **`params`:** none. Any param fails the rule before anything runs, with the reason
  `shellcheck param <name>: a shellcheck rule takes no params`.
- **Which files it is handed.** `tm` picks the scripts itself, among the files git lists under the rule file's
  folder: the tracked files, then the untracked files git does not ignore. A file is a script when
  - its extension is `sh`, `bash`, `ksh` or `dash`; or
  - it has no extension and its first line is a `#!` line that runs `sh`, `bash`, `dash` or `ksh`, named directly
    (`#!/bin/sh`) or through `env` (`#!/usr/bin/env bash`; env's own flags are skipped). `tm` reads at most the
    file's first 257 bytes for this, and a first line longer than 256 bytes is not read as a shebang. Another shell,
    such as zsh or fish, does not count.

  A file with any other extension is never a script, whatever its first line. Never handed over:
  - a symlink, and any file under a symlinked folder, whatever it points at;
  - anything that is not a regular file, and a file with no extension that cannot be read;
  - a tracked file deleted from the work tree;
  - a skip-worktree file, whether or not it is there.

  A script with an unresolved merge conflict is listed once, as it is in the work tree. The shebang is read on Unix
  only: on another system a file with no extension is never a script.
- **No script, nothing run.** A rule that covers none of those scripts runs nothing, whether or not ShellCheck is
  installed and whether or not its code is on the list ([above](#a-rule-that-covers-no-file)).
- **Outside a git repository** the scripts cannot be listed. A rule that finds ShellCheck fails, with no exit code
  and git's reason (`<folder>: not a git repository`), before its code is looked up in the list. A rule that does
  not find ShellCheck is not available.
- **What is run for a code on the list,** in the rule file's folder:

  ```
  shellcheck -f json1 --enable=all --rcfile=<config> -- <script>...
  ```

  or, for a rule with no `config`, the same with `--norc` in place of `--rcfile=<config>`. Every script under the
  folder is handed over, and the rule's code is not passed: every check runs, `--enable=all` turning the optional
  ones on, and `tm` keeps the comments whose code is the rule's, in the files the rule covers. `--` ends the
  options, so a script whose name begins with `-` is read as a file; one named exactly `-` is passed as `./-`.
  `SHELLCHECK_OPTS` is removed from ShellCheck's environment for this run, since ShellCheck reads more options
  from it.
- **`config`:** passed as `--rcfile=<config>`, as written, so a relative path is taken from the rule file's folder.
  Temperare does not read or copy the file. Without a `config` the run gets `--norc`, so no `.shellcheckrc` in the
  code is read: a `disable=` there cannot silence a rule.
- **Failed,** for a code on the list, when ShellCheck exits with anything but 0 or 1; when it says it was
  `unable to read --rcfile`; when what it prints is not its `json1` output; and when it reports that it could not
  parse a script (an error-level comment with a code from `SC1000` to `SC1999`), unless the rule's own code is one
  of those.
- **A finding** is placed at the comment's line and column, and its message is ShellCheck's.
- **Runs alone.** A ShellCheck rule never shares a run, even with another ShellCheck rule.

## git

`git` as an engine counts what a change adds. It is separate from the git `tm` runs for `--base` and to list files,
which needs no rule.

- **Available when** `git` is on `PATH`.
- **It needs `--base`.** It counts a change against the base commit `tm check --base <rev>` names. Without
  `--base`, a git rule that is not refused, covers a file and finds git is not available, with the why
  `reports only what a change adds; run with --base`, and nothing is counted. With `only_with_base = true` and no
  `--base` the rule is not applicable instead, as any such rule is.
- **`check`:** `test_ratio`, the only one. Any other fails the rule before anything runs, with the reason
  ``git check `<check>` is not one git counts: the git entry checks only `test_ratio` ``.
- **`params`,** all three required:
  - `max_ratio`, a number from 0.1 to 100: the most lines of test a change may add for each line of code;
  - `floor`, a whole number from 0 to 100000: a change that adds fewer lines of test than this is not reported;
  - `test_paths`, a list of globs with at least one item, relative to the rule file's folder: a file one of them
    matches counts as test, and any other file the rule covers counts as code.

  A missing param, a value out of range, a glob that does not compile, and any other param, fail the rule before
  anything runs.
- **What it counts:** the lines the change adds in the files the rule covers. For a tracked file
  those are the lines git shows as added against the base; an untracked file git does not ignore counts whole,
  unless it is binary, a symlink or not a regular file. A change that adds at least `floor` lines of test, and more
  than `max_ratio` lines of test for each line of code, gets one finding: at the first added line of the test file
  with the most added lines, with no column, and a message such as
  `this change adds 120 lines of test for 40 lines of code (3.0 to 1; the rule's limit is 2 past 50 lines)`.
- **`config`:** not used.
- **Runs alone.** A git rule never shares a run.

## pylint

- **Available when** `pylint` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  pylint --persistent=n --output-format=json2 --disable=all --enable=unknown-option-value,useless-option-value --enable=<check> [--rcfile=<config>] .
  ```

  `--disable=all` then `--enable=<check>` runs the one message. pylint lays its command line over the config file,
  so a config that disables the check does not hide it. `unknown-option-value` (W0012) and `useless-option-value`
  (R0022) stay on so that pylint says when it does not know the check. `--persistent=n` keeps pylint from writing its
  statistics file under your home folder.
- **`check`:** one pylint message id: a letter `C`, `R`, `W`, `E`, `F` or `I` and four digits, such as `C0121`.
  Messages with exactly that id are the findings.
- **Failed before pylint runs** when `check` is:
  - anything but a message id, such as a symbol (`singleton-comparison`), a category, a checker name or a list.
    Those turn on messages reported under other ids, so a finding could not be matched back. The reason is
    ``pylint check `<check>` is not a message id (a letter C, R, W, E, F or I and four digits, as C0121), so tm cannot tell which findings it selects``;
  - an old id that pylint 4.1.4 still takes and reports under its new id (such as `W0703`, now `W0718`). The rule
    would read clean, so the reason names the id to write:
    ``pylint check `W0703` is an old id of W0718 in pylint 4.1.4, whose findings carry W0718: name W0718``.
- **Not available** when pylint says it does not know `check`: its W0012 or R0022 message names the check, often
  because the id is newer than the installed pylint or was removed from it. The reason quotes pylint's message.
- **Failed** when what pylint prints is not its json2 output, as when it finds no file to lint.
- **`config`:** passed as `--rcfile=<config>`, as written, so a relative path is taken from the rule file's folder.
  Without one, pylint finds its own configuration.
- **`params`:** none. Any param fails the rule before pylint runs, with the reason
  `pylint param <name>: a pylint rule takes no params`.
- **A finding** is placed at the message's line and column, and its message is pylint's. pylint's own messages
  about its options are not findings.
- **Runs alone.** A pylint rule never shares a run.

## clang-tidy

- **Available when** `clang-tidy` is on `PATH`. `tm` lists the sources with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed.** `tm` picks the sources itself, among the files git lists under the rule file's
  folder (the tracked files, then the untracked files git does not ignore): those ending `.c`, `.cc`, `.cpp`,
  `.cxx`, `.c++`, `.m` or `.mm`, that are regular files and not symlinks, and that the rule covers. That is C, C++,
  Objective-C and Objective-C++. A header (`.h`, `.hpp`) is not handed over itself. A rule that covers files but
  none of these sources runs nothing ([above](#a-rule-that-covers-no-file)); this is settled before clang-tidy is
  looked for, so it is the same whether or not clang-tidy is installed.
- **What runs,** in the rule file's folder:

  ```
  clang-tidy --quiet --warnings-as-errors=-* --checks=-*,<check> --config-file=<config> <source>...
  ```

  or, for a rule with no `config`, the same with `--config={}` in place of `--config-file=<config>`. Each source is
  passed as an absolute path. `--checks=-*,<check>` turns every check off, then the rule's on.
- **A compile database is needed.** Each source needs a `compile_commands.json` or a `compile_flags.txt` in its
  folder or a folder above it. Without one clang-tidy checks the file with no flags and still exits 0, so its
  findings would not be your project's. The rule is then not available, with the reason
  `no compile database (compile_commands.json) or compile flags (compile_flags.txt) for <file> or any folder above it, so clang-tidy would check it without flags`.
  The `file` entries of a `compile_commands.json` must be absolute paths: with relative ones clang-tidy prints its
  findings relative to the database, and the rule fails.
- **`check`:** one clang-tidy check name: a letter, then letters, digits, `-`, `.` and `_`, such as
  `bugprone-branch-clone` or `clang-analyzer-core.NullDereference`. Diagnostics tagged with exactly that name are the
  findings. A glob (`*`), a list (`,`) or a leading `-` fails the rule before clang-tidy runs, with the reason
  ``clang-tidy check `<check>` is not one check name (letters, digits, `-`, `.` and `_`, starting with a letter), so tm cannot tell which findings it selects``.
- **Not available** when clang-tidy says `Error: no checks enabled.`: the run turns on only the rule's check, so
  clang-tidy does not know it. The reason is
  ``clang-tidy <version> does not know the check `<check>`: Error: no checks enabled.``.
- **Failed** when clang-tidy exits with anything but 0, as when a source does not compile; the reason quotes its
  first `error:` line.
- **`config`:** passed as `--config-file=<config>`, as written, so a relative path is taken from the rule file's
  folder. Without one the run gets `--config={}`, so no `.clang-tidy` file in the code is read: the rule file is the
  only home for a rule.
- **`params`:** none. Any param fails the rule before clang-tidy runs, with the reason
  `clang-tidy param <name>: a clang-tidy rule takes no params`.
- **A finding** is placed at the diagnostic's line and column, and its message is clang-tidy's.
- **Runs alone.** A clang-tidy rule never shares a run.

## golangci-lint

- **Available when** `golangci-lint` and `go` are both on `PATH`: golangci-lint loads the packages with go. With go
  missing the reason is `go not found on PATH`, and the `next:` line asks you to install golangci-lint.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  golangci-lint run --output.json.path=stdout --output.text.path=stderr --show-stats=false --issues-exit-code=1 --max-issues-per-linter=0 --max-same-issues=0 --uniq-by-line=false --allow-parallel-runners --path-mode=abs --path-prefix= --fix=false --enable-only=<check> [--config=<config>] ./...
  ```

  Each fixed flag holds against the repository's own config: every issue is reported, none is capped or folded,
  nothing is fixed in the code (a config's `issues.fix` would edit it), and a second golangci-lint may run beside
  this one.
- **Go stays local.** The run gets `GOTOOLCHAIN=local`, `GOPROXY=off` and `GOSUMDB=off`, so go downloads no
  toolchain and no module and looks up no checksum, and `GOFLAGS` is removed, so nothing is written to `go.mod` or
  `go.sum`. `PWD` is set to the rule file's folder. Code that cannot be loaded with what is already on this machine
  fails the rule (below).
- **`check`:** golangci-lint linter names: one, such as `staticcheck`, or several joined by commas with no spaces,
  such as `staticcheck,govet`. Issues from those linters are the findings. A linter's own check, such as
  staticcheck's `SA1019`, cannot be a `check`: golangci-lint takes those only from its config file.
- **Failed before golangci-lint runs** when a part of `check` is not a linter name (lower-case letters, digits and
  `_`) or is `typecheck`, golangci-lint's loading step. The reason is
  ``golangci-lint check `<check>`: `<part>` is not a linter name; tm picks golangci-lint's linters by name (`staticcheck`), not a linter's own checks (`SA1019`)``.
- **Not available** when golangci-lint says a linter of `check` is unknown (`unknown linters: '<names>'`). The
  reason quotes its line.
- **Failed** when golangci-lint exits with anything but 0 or 1 (as when the folder holds no Go module), when what
  it prints is not its JSON, and when it reports an issue from `typecheck`: the code did not load or compile, so no
  linter checked it. The reason then is `golangci-lint could not load the code, so its linters did not check it: <text>`.
- **`config`:** passed as `--config=<config>`, as written, so a relative path is taken from the rule file's folder.
  Without one, golangci-lint finds its own configuration. `--enable-only` decides the linters either way.
- **`params`:** none. Any param fails the rule before golangci-lint runs, with the reason
  `golangci-lint param <name>: a golangci-lint rule takes no params`.
- **A finding** is placed at the issue's line and column (none when golangci-lint gives 0), and its message is the
  issue's text.
- **Runs alone.** A golangci-lint rule never shares a run.

## gitleaks

- **Available when** `gitleaks` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  gitleaks dir --no-banner --no-color --redact --ignore-gitleaks-allow --report-format=sarif --report-path=- --enable-rule=<check> [--config=<config>] .
  ```

  `dir` scans the files in the folder as they are on disk; gitleaks' `git` mode, which reads the history, is not
  used. The SARIF log on stdout is the answer, whatever gitleaks' exit code.
- **The secret is never printed.** `--redact` has gitleaks write `REDACTED` in place of what it matched, and every
  finding's message is one fixed text of `tm`'s, `gitleaks found a secret here; tm does not print it`, never
  gitleaks' own. Of gitleaks' stderr `tm` keeps only its error lines (`ERR` and `FTL`), so no finding reaches the
  report through a failed rule's stderr either.
- **The environment:** `GITLEAKS_CONFIG` and `GITLEAKS_CONFIG_TOML` are removed for the run, so a config set in your
  environment cannot stand in for the rule's. Nothing gitleaks does here reaches the network.
- **`check`:** a gitleaks rule id, such as `aws-access-token`, or several joined by commas with no spaces. Results
  with exactly one of those ids are the findings. Ids are not checked for their shape, since a repository's own
  gitleaks config can name rules of its own.
- **Failed before gitleaks runs** when a part of `check` is empty or is `ALL`, with the reason
  ``gitleaks check `<check>`: `ALL` is not a gitleaks rule id`` or
  ``gitleaks check `<check>`: an empty part is not a gitleaks rule id``.
- **Not available** when gitleaks says `Requested rule <id> not found in rules` for a part of `check`. The reason
  quotes that line.
- **Failed** when what gitleaks prints is not a SARIF log. The reason is `gitleaks printed no SARIF log and no
  error`, with its error lines in its place when it printed any.
- **`config`:** passed as `--config=<config>`, as written, so a relative path is taken from the rule file's folder.
  Without one, gitleaks reads the folder's `.gitleaks.toml` when there is one, else its own default rules. The
  config's own rules apply.
- **The rule file is the only home for an exception.** A `gitleaks:allow` comment in the code silences nothing
  (`--ignore-gitleaks-allow`). gitleaks' other ways to silence a finding cannot be turned off, so each fails the
  rule before gitleaks runs, once gitleaks is found on `PATH`:
  - an allowlist in the config gitleaks reads (the rule's `config`, else the folder's `.gitleaks.toml`), or in a
    file it extends, with its keys matched in any case as gitleaks reads them. The reason is
    ``gitleaks config `<name>` declares an allowlist (`<key path>`); the rule file is the only home for an exception, so tm reads no findings under it``.
    A part of the key path that is not one of gitleaks' table names prints as `<key>`, since config text can hold a
    secret;
  - such a config that does not parse as TOML:
    ``gitleaks config `<name>` does not parse as TOML, so tm cannot tell whether it declares an allowlist and reads no findings under it``;
  - a `.gitleaksignore` in the rule file's folder that lists anything (a line that is not empty and not a `#`
    comment): `` `.gitleaksignore` in the rule's folder lists findings to ignore; the rule file is the only home for an exception, so tm reads no findings under it ``.
    gitleaks reads that file whatever it is told, so `tm` cannot switch it off.

  gitleaks' own default configuration leaves some paths out of every scan, such as `.svg` files and
  `package-lock.json`. That is gitleaks' definition of its checks, as a lint's scope is, not an exception.
- **`params`:** none. Any param fails the rule before gitleaks runs, with the reason
  `gitleaks param <name>: a gitleaks rule takes no params`.
- **A finding** is placed at the result's line and column.
- **Runs alone.** A gitleaks rule never shares a run.

## cargo-audit

- **Available when** `cargo-audit` is on `PATH`, and its advisory database has been fetched (below).
- **Which file it checks:** the `Cargo.lock` in the rule file's folder, and only that one: a lock file in a folder
  above or below is not read. With no `Cargo.lock` there (a symlink does not count, since it could point outside the
  repository), or one the rule does not cover, the rule runs nothing and covers nothing
  ([above](#a-rule-that-covers-no-file)). This is settled before cargo-audit is looked for on `PATH`.
- **What runs,** in the rule file's folder:

  ```
  cargo-audit audit --no-fetch --no-yanked --json --db <db> --file <folder>/Cargo.lock
  ```

- **Nothing reaches the network.** `<db>` is cargo-audit's own default database, `$CARGO_HOME/advisory-db`, else
  `$HOME/.cargo/advisory-db`; `tm` only reads those two variables to find it and sets none. `--no-fetch` reads the
  database as it is, and `--no-yanked` keeps cargo-audit off the crates.io index. A database with no `crates` folder
  makes the rule not available, with the reason
  ``no advisory database at <db>: fetch it with `cargo audit update-db --db <db>` (tm never fetches it)``.
  An old database is read as it is: fetch it again to see newer advisories.
- **`check`:** an advisory id, `RUSTSEC-` then a four-digit year, `-` and four digits (`RUSTSEC-2023-0071`), or a
  kind: `vulnerability`, `unmaintained`, `unsound`, `notice`, or `all` for every advisory. The check is not passed
  to cargo-audit: it reports every advisory and `tm` keeps the rule's. An advisory is kept when the check is `all`,
  its kind or its id.
- **Failed before cargo-audit runs** when `check` is neither an id nor a kind, with the reason
  ``cargo-audit check `<check>` is not an advisory id (RUSTSEC-YYYY-NNNN) or a kind: all, vulnerability, unmaintained, unsound or notice``.
  `yanked` is refused too, since cargo-audit learns of a yanked crate only from the crates.io index:
  ``cargo-audit check `yanked` needs the crates.io index, which tm never fetches; name an advisory id or a kind: all, vulnerability, unmaintained, unsound or notice``.
- **Not available** when the check is an advisory id the database does not hold: cargo-audit would report nothing
  for it, which is not clean. The reason ends `<check> is not in the advisory database at <db>`.
- **An `audit.toml` silences nothing.** cargo-audit applies an `audit.toml` it finds, and its `ignore` list can
  silence an advisory outside the rule file. When cargo-audit's report shows settings other than its defaults, the
  rule fails, with the reason
  `cargo-audit applied an audit.toml tm was not given (<the settings>); the rule file is the only home for an exception, so tm reads no findings under it`.
  Put an exception in the rule file instead ([`RULE_FILE.md`](RULE_FILE.md#an-exception-exception)).
- **Failed** also when what cargo-audit prints is not its JSON report, with the reason
  `cargo-audit printed no JSON report: <the start of what it printed>`.
- **`config`:** not used.
- **`params`:** none. Any param fails the rule before cargo-audit runs, with the reason
  `cargo-audit param <name>: a cargo-audit rule takes no params`.
- **A finding** is on `Cargo.lock`, with no line, and reads
  `<id>: <crate> <version>: <title>; upgrade to <version>` (several patched versions joined with ` or `), or ends
  `; no patched version`.
- **Runs alone.** A cargo-audit rule never shares a run.

## vulture

- **Available when** `vulture` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  vulture --config=/dev/null [--exclude=<what git ignores>] .
  ```

  with `PYTHONWARNINGS=ignore` set for the run, so a Python warning about a file vulture reads well does not land
  on stderr. Nothing reaches the network.
- **What it scans.** Two things decide it, and they are separate:
  - **Left out of the scan:** what git ignores under the folder. Before vulture runs, `tm` asks git for its ignored
    paths (`git ls-files --others --ignored --exclude-standard --directory`) and passes each ignored `.py` file, and
    each ignored folder that holds one (a `.gitignore`d virtualenv, build output), to `--exclude`. vulture neither
    reads nor reports them, so a name used only there counts as unused. Tracked files, and untracked files git does
    not ignore, are scanned. Outside a git repository nothing is left out and the whole folder is scanned.
  - **Dropped after the scan:** findings outside the rule's `paths`, `exclude`, `kits` and `outside_kits`. These do
    not narrow the scan, so a file the rule does not cover is still read, and one vulture cannot read or parse
    still fails the rule (below).

  With no Python file to scan, vulture prints nothing and the rule reads clean.
- **Failed before vulture runs** when an ignored path holds a `,` (vulture splits `--exclude` on commas):
  `cannot leave the git-ignored path <path> out of vulture's scan: vulture splits --exclude on commas`; and when
  the ignored Python paths do not fit in one argument:
  ``too many git-ignored Python paths under the folder for vulture's --exclude (more than 98304 bytes in one argument); ignoring their folders instead (a `dir/` line in .gitignore) keeps the list short``.
- **`check`:** one vulture code: `V101` (unused attribute), `V102` (class), `V103` (function), `V104` (import),
  `V105` (method), `V106` (property), `V107` (variable, and unused argument) or `V201` (unreachable code). vulture
  reports every kind at once, so `tm` keeps the findings of the rule's code. Any other check fails the rule before
  vulture runs, with the reason
  ``vulture check `<check>` is not a vulture code (one of V101, V102, V103, V104, V105, V106, V107, V201), so tm cannot tell which findings it selects``.
  `F401` and `F841`, flake8's codes, are refused with the code to use instead: `V104` and `V107`.
- **No confidence floor.** vulture's confidence is fixed by kind (60 for definitions, 90 for imports, 100 for
  unreachable code and unused arguments), so the check already chooses it.
- **Failed** when vulture exits with anything but 0 (nothing found) or 3 (findings), and when it writes anything to
  stderr, which it does only for a file it could not read or parse: a rule never reads clean over a file vulture
  skipped. Also when it prints a line that is not a finding.
- **`config`:** not used. `--config=/dev/null` keeps vulture from reading the folder's `pyproject.toml`, whose
  `[tool.vulture]` settings (`ignore_names`, `ignore_decorators`, `exclude`, `min_confidence`) would hide findings
  outside the rule file.
- **What vulture still honours,** which `tm` cannot turn off:
  - a `# noqa` comment on the item's line, bare or naming its code (or `F401`, `F841`). A repository that takes
    `standard@1` gets each such comment reported by `tm-python-suppression`, as any `# noqa` is;
  - a whitelist module the repository keeps, such as one made with `--make-whitelist`, when git does not ignore it:
    it is Python code under the folder, so the names it uses count as used, and `tm` cannot tell it from real use;
  - vulture's own bundled whitelists for standard-library modules (such as `sys`, `logging` and `unittest`), read
    when the code imports the module. That is vulture's definition of its check, not an exception.
- **`params`:** none. Any param fails the rule before vulture runs, with the reason
  `vulture param <name>: a vulture rule takes no params`.
- **A finding** is placed at vulture's line, with no column, and its message is vulture's without the confidence.
- **Runs alone.** A vulture rule never shares a run.

## periphery

- **Available when** `periphery` is on `PATH`, and the rule file's folder is inside a Swift package. Periphery finds
  unused code from a build's index, so `tm` builds the package first, with `swift`, which must be on `PATH` too.
- **Where:** the nearest folder at or above the rule file's folder that holds a `Package.swift`, looking no higher
  than the top of the git repository, so a package outside the repository is never built. The whole package is
  built and scanned; only results in the rule file's folder are kept. A rule file's folder in no git repository
  fails the rule, with git's error.
- **Swift packages only.** With no `Package.swift` at or above the folder within the repository, as in an Xcode
  project, the rule is not available:
  `no Package.swift at or above the rule file's folder: Periphery builds the project, and tm runs it only on a Swift package`.
- **What runs,** in the package's folder unless said otherwise:

  ```
  swift package --scratch-path <scratch>/build dump-package
  swift package --scratch-path <scratch>/dep-<n> dump-package      (in each local package's folder)
  swift package --scratch-path <scratch>/build describe --type json
  swift build --build-tests --build-system swiftbuild --scratch-path <scratch>/build --disable-automatic-resolution
  periphery scan --skip-build --format json --disable-update-check --quiet --index-store-path <scratch>/build/out --json-package-manifest-path <scratch>/describe.json --config <scratch>/periphery.yml
  ```

  `<scratch>` is a folder `tm` makes under the system's temporary folder and removes after the run, however it
  ends: nothing is built in the repository or in DerivedData. The tests are built too, so code only they use counts
  as used.
- **Offline, and only the repository's own code.** Before anything is built, `tm` reads the manifest of the package
  and of every local package it depends on by path, and of theirs in turn, each once. The rule is not available
  when any of them:
  - names a source-control or registry dependency, or a binary target with a `url`, all of which SwiftPM would
    fetch: `<subject> depends on <identities>, which tm would have to fetch: tm never fetches packages, so Periphery cannot build it here`,
    where a binary target reads `binary target <name> (<url>)`;
  - depends on a local package whose path does not resolve:
    `<subject> depends on the local package <path>, which does not resolve: tm cannot vouch for it, so Periphery cannot build it here`;
  - depends on a local package outside the repository:
    `<subject> depends on the local package <path>, outside the repository: tm cannot vouch for it, so Periphery cannot build it here`.

  `<subject>` is `the package` for the rule's own package and `the local package <path in the repository>` for one
  it reaches. A binary target with a local `path` is allowed. `--disable-automatic-resolution` makes a build that
  would resolve a dependency fail instead, and `--disable-update-check` keeps Periphery from asking for a newer
  release.
- **A limit of Periphery 3.8.0's own.** Periphery credits a top-level reference in `main.swift` to the nearest
  declaration above it in that file. When that declaration is unused, the function the top-level code calls is
  reported unused too. It is the same through `tm`, a native build and Periphery's own build. To avoid it, put the
  top-level code before the declarations, or the declarations in other files.
- **Failed** when a `swift` step exits with anything but 0 (a package that does not compile gives no answer), with
  its stderr, or its stdout when stderr is empty; when the build leaves no index store; and when Periphery prints
  something that is not its JSON result list.
- **`check`:** one of Periphery's result hints, written as Periphery writes it: `unused`, `assignOnlyProperty`,
  `redundantProtocol`, `redundantConformance`, `redundantPublicAccessibility`, `superfluousIgnoreCommand`, or
  `all` for every result. Periphery reports every result at once, so `tm` keeps those with the rule's hint. Any other
  check fails the rule before anything runs, with the reason
  ``periphery check `<check>` is not a Periphery result hint: all, unused, assignOnlyProperty, redundantProtocol, redundantConformance, redundantPublicAccessibility or superfluousIgnoreCommand``.
- **`config`:** not used. `--config` names an empty file `tm` writes, so the package's `.periphery.yml`, whose
  retain and exclude options are exceptions outside the rule file, is never read.
- **What Periphery still honours,** which `tm` cannot turn off: `// periphery:ignore` comments in the code (also
  `periphery:ignore:all` and `periphery:ignore:parameters`) and `// periphery:override`. No released baseline set
  reports them: `standard@1`'s `tm-swift-suppression` reports `swiftlint:disable` only.
- **`params`:** none. Any param fails the rule before anything runs, with the reason
  `periphery param <name>: a periphery rule takes no params`.
- **A finding** is placed at the result's line and column and reads `<kind> <name>: <hints>`.
- **Runs alone.** A Periphery rule never shares a run.

## eslint

- **Available when** `eslint` is on `PATH`. `tm` runs that `eslint` and does not look in the folder's
  `node_modules/.bin`. It sets nothing in ESLint's environment, `NODE_PATH` included, so the plugins a config names
  must resolve as they do when you run that `eslint` yourself.
- **Where:** the rule file's folder.
- **What runs:**

  ```
  eslint --format json --no-inline-config --no-ignore --pass-on-unpruned-suppressions --no-error-on-unmatched-pattern [--config=<config>] --rule {"<check>":"error"} .
  ```

  The `--rule` pair turns the check on and carries a severity only, so the options the config gives that rule are
  kept.
- **`check`:** one ESLint rule id, such as `no-var` or `@typescript-eslint/no-explicit-any`. Messages whose `ruleId`
  is exactly that id are the findings.
- **Failed before ESLint runs** when `check` is empty, is `@` alone, or holds a character other than an ASCII letter
  or digit, `-`, `_`, `.` or `/` (a leading `@` apart), with the reason
  ``eslint check `<check>` is not an ESLint rule id; tm passes it to `--rule` and matches findings by it``.
- **Not available** when ESLint says the config it loaded does not define the rule: a line on its stderr that names
  the check in double quotes and says it could not find the rule in its plugin, or could not find the plugin. The
  reason quotes that line. A line naming another rule is not read as the check's.
- **Which files:** ESLint's config decides which files it lints. `tm` hands it the folder and keeps the findings in
  the files git lists, so a finding in a file git ignores is dropped. When git cannot list the files, every finding
  is kept.
- **No file is settled after the run.** When none of the files ESLint linted is both listed by git and covered by
  the rule, the rule covers no file ([above](#a-rule-that-covers-no-file)). ESLint has run by then, so it must be
  on `PATH`.
- **What is turned off:** `eslint-disable` comments and every other inline config (`--no-inline-config`), and the
  config's `ignores` (`--no-ignore`); ESLint's own defaults still leave out `node_modules`. A finding that an
  `eslint-suppressions.json` hides is read from the suppressed messages of ESLint's output and reported like any other;
  `--pass-on-unpruned-suppressions` keeps a stale entry in that file from failing the run.
- **Failed** when stdout is not ESLint's JSON array, as when its config does not load; the reason is ESLint's own
  stderr when it wrote any. Failed too when ESLint reports a fatal message for any file, such as one it could not parse, whatever
  the rule and even in a file git does not list, with the reason `eslint could not parse <file>: <message>`. Any
  exit code with the JSON array on stdout is otherwise an answer.
- **`config`:** passed as `--config=<config>`, as written, so a relative path is taken from the rule file's folder.
  Without one, ESLint finds its own configuration.
- **`params`:** none. Any param fails the rule before ESLint runs, with the reason
  `eslint param <name>: an eslint rule takes no params`.
- **A finding** is placed at the message's line and column (none when ESLint gives none), and its message is
  ESLint's.
- **Runs alone.** An ESLint rule never shares a run.

## mypy

- **Available when** `mypy` is on `PATH`. `tm` lists the files with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files ending `.py` or `.pyi` that git lists under the rule file's folder (the
  tracked files, then the untracked files git does not ignore), that are regular files and not symlinks, and that
  the rule covers. A virtualenv git ignores never reaches mypy. A rule that covers files but none of these runs
  nothing ([above](#a-rule-that-covers-no-file)); this is settled before mypy is looked for.
- **What runs,** in the rule file's folder:

  ```
  mypy --output=json --cache-dir=/dev/null --enable-error-code=<check> --config-file=<config> -- <file>...
  ```

  `--cache-dir=/dev/null` keeps mypy from writing a cache, and `--install-types` is never passed, so nothing is
  fetched.
- **The config is always named.** It is the rule's `config`; else the config mypy itself would find, looked for in
  each folder from the rule file's up to the top of the repository, in mypy's order: `mypy.ini`, `.mypy.ini`,
  `pyproject.toml` when it has a `[tool.mypy]` table, `setup.cfg` when it has a `[mypy]` section. With none of those
  the run gets an empty `--config-file=`, which makes mypy read no config at all. So a config in your home folder is
  never read.
- **`check`:** one mypy error code: lower-case words of letters and digits joined by single `-`, the first starting
  with a letter, such as `arg-type` or `no-untyped-def`. `--enable-error-code` turns on a code that is off by
  default and wins over a `disable_error_code` in the config's global section. Messages of severity `error` with
  exactly that code are the findings. A code that mypy reports only under a strictness flag (`no-untyped-def` needs
  `disallow_untyped_defs`) is reported only when the config sets that flag.
- **Failed before mypy runs** when `check` is anything else, such as a list or a capitalised code, with the reason
  ``mypy check `<check>` is not one mypy error code (lowercase words joined by `-`, as `arg-type`), so tm cannot tell which findings it selects``.
- **Failed before mypy runs, once mypy is found,** when the config named above cannot be read, or silences the
  check outside the rule file:
  - `ignore_errors` set true, in any section;
  - `disable_error_code` listing the check in a per-module section (`[mypy-<pattern>]` or `[DEFAULT]` in an INI
    file, an entry of `[[tool.mypy.overrides]]` in a `pyproject.toml`), which the command line does not override;
  - `ignore_missing_imports` set true, when the check is `import-not-found` or `import-untyped`.

  The reason is
  `mypy config <file>: <section> <key> silences mypy findings outside the rule file; exceptions go in the rule file`.
  A `config` ending `.toml` that does not parse as TOML fails the rule too. The config's `exclude` does not apply,
  since the files are named on the command line.
- **Not available** when mypy says `Invalid error code(s):` and names the check. The reason quotes that line.
- **Failed** when mypy exits 2: a blocking error (a syntax error, a duplicate module) after which it checks nothing
  else. Temperare's own reason is
  ``mypy stopped at a blocking error, so it checked nothing for `<check>`: <file>:<line>: <message>``. The one
  exception is the check `syntax`, when every error mypy printed is a syntax error: those are its findings. Failed
  too when a line of stdout is not one of mypy's JSON messages. Exit 0 and exit 1 are answers.
- **What mypy still honours,** which `tm` cannot turn off: `# type: ignore` comments, the inline
  `# mypy: ignore-errors` and `# mypy: disable-error-code=` comments, and `@typing.no_type_check`.
- **`config`:** passed as `--config-file=`, taken from the rule file's folder.
- **`params`:** none. Any param fails the rule before mypy runs, with the reason
  `mypy param <name>: a mypy rule takes no params`.
- **A finding** is placed at the message's line (none when mypy ties it to no line) and column, counted from 1, and
  its message is mypy's.
- **Runs alone.** A mypy rule never shares a run.

## pmd

- **Available when** `pmd` is on `PATH`. `tm` lists the files with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Java only.** PMD is handed the files ending `.java` that git lists under the rule file's folder (the tracked
  files, then the untracked files git does not ignore), that are regular files and not symlinks, and that the rule
  covers. PMD's other languages are not run. A rule that covers files but no `.java` file runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before PMD is looked for.
- **What runs,** in the rule file's folder:

  ```
  pmd check --no-cache --no-progress --format=sarif --suppress-marker=TM-NOPMD-OFF --rulesets=<check> --file-list=<list>
  ```

  `<list>` is a temporary file naming the files, one per line; it is removed after the run. `CLASSPATH` is removed
  from PMD's environment, since PMD's launcher would put it ahead of PMD's own rulesets. `--no-cache` keeps PMD
  from writing an analysis cache, and no `--aux-classpath` is given.
- **`check`:** PMD rule references: one rule, `category/java/<category>.xml/<Rule>`; a whole category,
  `category/java/<category>.xml`; or several joined by commas with no spaces. PMD runs only those rules, and every
  result in its report is a finding.
- **Failed before PMD runs** when a part of `check` is anything else. The rule file is the only home for a rule and
  its exceptions, and a ruleset file can carry both, so a ruleset file or URL is refused:
  ``pmd check `<check>`: `<part>` is not a PMD Java category or rule reference (category/java/<category>.xml[/<Rule>]); a ruleset file or URL is refused, since the rule file is the only home for rules and their exceptions``.
  A category of another language gives `` `<part>` is not Java, the only language tm runs PMD on``, and an empty
  part `an empty part is not a PMD rule reference`.
- **Failed before PMD runs, once PMD is found,** when a file to hand over has a `,` in its path, which PMD reads in
  a file list as a separator:
  `<file>: pmd reads ',' in --file-list as a separator, so tm cannot hand it this file`.
- **Not available** when PMD says `[WARN] No rules found in ruleset <part>` (a rule its category does not have) or
  `[ERROR] Cannot load ruleset <part>:` (a category it does not have) for a part of `check`. The reason quotes that
  line.
- **Failed** when PMD exits 5: it could not process every file, so some went unchecked. Failed too on any exit but 0
  (no violation) and 4 (violations), and when what it prints is not a SARIF log.
- **`// NOPMD` silences nothing.** PMD has no switch for its suppression marker, only an option that renames it, so
  `tm` renames it to a text no code carries.
- **What PMD still honours,** which `tm` cannot turn off: `@SuppressWarnings` naming `"PMD"`, `"PMD.<Rule>"`,
  `"all"` or `"unused"`.
- **`config`:** not used. A rule's `config` does not reach PMD.
- **`params`:** none. Any param fails the rule before PMD runs, with the reason
  `pmd param <name>: a pmd rule takes no params`.
- **A finding** is placed at the result's line and column, and its message is PMD's.
- **Runs alone.** A PMD rule never shares a run.

## detekt

- **Available when** `detekt` is on `PATH`. `tm` lists the files with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files ending `.kt` or `.kts` that git lists under the rule file's folder (the
  tracked files, then the untracked files git does not ignore), that are regular files and not symlinks, and that
  the rule covers. Build output and generated code that git ignores are never handed over. A rule that covers files
  but none of these runs nothing ([above](#a-rule-that-covers-no-file)); this is settled before detekt is looked
  for.
- **A list of names built into `tm`.** A rule's `check` is looked up in a list of detekt 1.23.8's built-in rules:
  214 rules in 9 rule sets (`comments`, `complexity`, `coroutines`, `empty-blocks`, `exceptions`, `naming`,
  `performance`, `potential-bugs`, `style`). The list is built into `tm` and never fetched.
- **`check`:** detekt rule names and rule-set names, as that list writes them: one, such as `MagicNumber`, or
  several joined by commas with no spaces. A rule name picks that rule. A rule-set name picks every rule of the set,
  those detekt ships turned off included. Results whose rule is one the check picks are the findings.
- **No classpath, so no rule that needs type resolution.** `tm` passes detekt no classpath. 67 of the 214 rules need
  one to resolve types, and without it they would report nothing. A check that picks any of them, by name or
  through its rule set, is not available, with the reason
  `detekt needs type resolution for <rules>, which needs a classpath that tm does not give detekt, so they would report nothing`,
  and nothing runs. Every rule set but `comments` and `empty-blocks` holds such a rule, so only those two can be
  named as a set; from the other seven, name the rules you want. `Deprecation` is one of the 67, so detekt cannot
  report calls to deprecated Kotlin code through `tm`.
- **Failed before detekt runs** when a part of `check` is not a well-formed name (an ASCII letter, then ASCII
  letters, digits or `-`), such as `detekt.style.MagicNumber`. The reason starts
  ``detekt check has `<part>`, which is not a well-formed rule or rule-set name``.
- **Not available** when a part of `check` is well formed and not on the list, such as `ALL` or `Style`, with the
  reason `detekt <version>: <part> is not in temperare's list of detekt codes (taken from detekt 1.23.8)`. Nothing
  runs for it: detekt would run, match nothing and read clean. Not available too when the installed detekt says a
  rule from the list is `misspelled or does not exist`; the reason quotes its line. Both are settled in the run,
  after the files and `PATH` are looked at.
- **What runs,** in the rule file's folder:

  ```
  detekt --input <file>,<file>... --config [<config>,]<layer> --report sarif:<report>
  ```

  `<layer>` is a config `tm` writes for the run and `<report>` the file detekt writes its SARIF report to; both are
  temporary files, removed after the run. `JAVA_TOOL_OPTIONS`, `JDK_JAVA_OPTIONS` and `_JAVA_OPTIONS` are removed
  from detekt's environment; `JAVA_HOME` is kept. `tm` passes no `--plugins`, `--classpath`, `--baseline` or
  `--build-upon-default-config`.
- **`config`:** the rule's `config`, taken from the rule file's folder; else the folder's `detekt.yml` when there
  is one; else none. `tm`'s layer always comes after it, and detekt lets the last config win key by key. The
  repository's config and the layer are the whole config: an option neither sets takes detekt's built-in default.
- **What the layer sets.** For each rule the check picks, and for its rule set: `active: true`, and empty `excludes`
  and `includes`; for the rule, an empty `ignoreAnnotated` too. So a repository config that turns the rule or its set
  off, or leaves out paths or annotated code, silences nothing. The rule's own options in the repository's config,
  such as MagicNumber's `ignoreNumbers` or a threshold, still apply. The layer also turns config validation and the
  report on.
- **A baseline hides nothing.** `tm` never passes `--baseline`, and detekt reads none by itself, so a
  `detekt-baseline.xml` in the repository is not read.
- **What detekt still honours,** which `tm` cannot turn off: `@Suppress` in the code.
- **Failed** when detekt exits with anything but 0 (no issues) or 2 (issues): 1 is a usage error or a crash, and 3
  a config that fails validation. The `comments` rule set holds `AbsentOrWrongFileLicense`, which needs a license
  template file; without one detekt exits 1, so a rule that names `comments` fails rather than reading clean. Failed
  too, before detekt runs, when a file or config path holds `,` or `;` (detekt splits its paths on both), or the
  list of files is longer than 98304 bytes; narrower `paths` keep it short.
- **`params`:** none. Any param fails the rule before detekt runs, with the reason
  `detekt param <name>: a detekt rule takes no params`.
- **A finding** is placed at the result's line and column, and its message is detekt's.
- **Runs alone.** A detekt rule never shares a run.

## rubocop

- **Available when** `rubocop` is on `PATH`.
- **Where:** the rule file's folder.
- **What runs,** all in that folder. The check run:

  ```
  rubocop --cache false --no-server --format json --ignore-disable-comments --only=<check> [--config=<config>] .
  ```

  then, to see what the repository's config hides:

  ```
  rubocop --show-cops=<check> --debug [--config=<config>]
  rubocop --show-cops=<check> --force-default-config
  rubocop --list-target-files --force-default-config .
  ```

  The second of those three runs only when the first shows an `Exclude` for a cop of the check. `--cache false`
  keeps RuboCop from writing its result cache, and `--no-server` from starting or using its server.
- **The environment:** every one of those runs gets `RUBOCOP_OPTS` set empty, so no options come from your
  environment; and `http_proxy`, `https_proxy`, `HTTP_PROXY` and `HTTPS_PROXY` set to `http://127.0.0.1:9`, where
  nothing listens, with `no_proxy` and `NO_PROXY` empty. A config with a remote `inherit_from` then fails instead of
  being fetched.
- **`check`:** full cop names: one, such as `Lint/UselessAssignment`, or several joined by commas with no spaces.
  `--only` runs those cops even where the config disables them. Offenses whose cop is one of them are the findings.
- **Failed before RuboCop runs** when a part of `check` is not a full cop name, such as a department (`Style`), a
  wildcard or an option. A department runs every cop in it, a set that changes from one RuboCop version to the next,
  so a rule names its cops. The reason is
  ``rubocop check `<check>`: `<part>` is not a full cop name such as `Lint/UselessAssignment` ``.
- **Failed before RuboCop runs, once RuboCop is found,** when the rule file's folder holds a `.rubocop` file, which
  RuboCop reads as more command-line options:
  `<path> gives rubocop options of its own; tm runs rubocop with its own options only`.
- **Not available** when RuboCop says `Unrecognized cop or department: <cop>.` for a cop of `check`. The reason
  quotes that line.
- **Which files:** RuboCop's config decides which files it inspects. No file is settled after the run: when none of
  the files RuboCop says it inspected is one the rule covers, the rule covers no file
  ([above](#a-rule-that-covers-no-file)).
- **`# rubocop:disable` and `# rubocop:todo` comments silence nothing** (`--ignore-disable-comments`).
- **Failed when the repository's config holds an exception.** RuboCop has no option that turns these off, so each
  fails the rule after the check run:
  - a `.rubocop_todo.yml` the config reads that names a cop of the check, even in a comment:
    `rubocop's todo file <path> names <cop> on line <n>: a todo file is a list of exceptions, and exceptions go in the rule file`;
  - an `Exclude` for a cop of the check with an entry RuboCop's stock config does not have:
    `the repository's rubocop config excludes <entry> from <cop>, beyond rubocop's stock Exclude; exceptions go in the rule file`;
  - an `AllCops` `Exclude` or `Include` that leaves out a file the rule covers and RuboCop's stock config inspects:
    `the repository's rubocop config (AllCops Exclude or Include) leaves out <path>, a file in the rule's paths; exceptions go in the rule file`.
- **Two cases those checks do not see.** A `.rubocop.yml` in a subfolder applies its own todo file and `Exclude`s
  to the files under it, and only the config of the rule file's folder is read. A cop's `Include` can narrow it to
  fewer files than the stock one, and only `Exclude` lists are compared.
- **Failed** when RuboCop exits with anything but 0 (no offenses) or 1 (offenses), as for a config it cannot load;
  when what it prints is not its JSON report; and when it reports a file it could not parse, unless the check names
  `Lint/Syntax`, the cop RuboCop reports that under. Temperare's own reason for the last is
  `rubocop could not read <file>: <message>`. Failed too when one of the three later runs exits with anything but
  0, with its stderr.
- **`config`:** passed as `--config=<config>`, as written, so a relative path is taken from the rule file's folder.
  Without one, RuboCop finds the `.rubocop.yml` at or above the folder.
- **`params`:** none. Any param fails the rule before RuboCop runs, with the reason
  `rubocop param <name>: a rubocop rule takes no params`.
- **A finding** is placed at the offense's start line and column, and its message is RuboCop's.
- **Runs alone.** A RuboCop rule never shares a run.

## phpstan

- **Available when** `phpstan` and `php` are both on `PATH`: the `phpstan` program is run by `php`. `tm` lists the
  files with git, so git must be there too. The rule also needs a PHPStan config (below).
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files ending `.php` that git lists under the rule file's folder (the tracked
  files, then the untracked files git does not ignore), that are regular files and not symlinks, and that the rule
  covers. PHPStan sees the rest of the code only as the config's own bootstrap and scan settings let it. A rule that
  covers files but no `.php` file runs nothing ([above](#a-rule-that-covers-no-file)); this is settled before
  PHPStan is looked for.
- **What runs,** in the rule file's folder:

  ```
  phpstan analyse --no-progress --no-interaction --no-ansi --error-format=json --level=<check> --configuration=<wrapper> -- <file>...
  ```

- **`check`:** a PHPStan level, `0` to `10` or `max`, passed as `--level`. The rule reports every message PHPStan
  gives at that level, in the files the rule covers. A check is a level, not one of PHPStan's rules.
- **Failed before PHPStan runs** when `check` is neither digits nor `max`, with the reason
  ``phpstan check `<check>` is not a PHPStan level (0 to 10, or max)``.
- **Not available** when PHPStan says the level's config file `was not found`: a number the installed PHPStan does
  not have, such as `11`. The reason quotes its line.
- **The repository's own config is needed.** It is the rule's `config`; else the first of `.phpstan.neon`,
  `phpstan.neon`, `.phpstan.neon.dist`, `phpstan.neon.dist`, `.phpstan.dist.neon` and `phpstan.dist.neon` in the
  rule file's folder, the order PHPStan itself looks in. With none the rule is not available, with the reason
  `the rule names no config and its folder holds no phpstan.neon, phpstan.neon.dist or phpstan.dist.neon, nor their dot-prefixed forms: tm runs phpstan with the repository's own config`.
- **The wrapper config.** PHPStan is pointed at a config `tm` writes in a new temporary folder. It includes the
  repository's config, puts PHPStan's `tmpDir` beside itself, and empties two settings:
  - `ignoreErrors`, which drops the config's own entries and those of any baseline file it includes;
  - `excludePaths`, so no exclusion in the config drops a file handed over.

  The wrapper, its folder and the cache PHPStan writes there are removed after the run, so the run leaves nothing
  behind.
- **Stated limits:** what still keeps a finding out of the report, and does not fail the rule.
  - Inline `@phpstan-ignore`, `@phpstan-ignore-line` and `@phpstan-ignore-next-line` comments. PHPStan has no option
    that turns them off.
  - A config's `stubFiles` that names a file handed over: PHPStan reads a stub for its types and does not analyse
    it, so that file's findings are not reported. When `stubFiles` names every file handed over, PHPStan says
    `No files found to analyse.` and the rule covers no file, with the why `no file its engine checks`.
  - A service the config tags `phpstan.ignoreErrorExtension`, which can drop errors.

  The wrapper cannot empty `stubFiles` or the services without dropping PHPStan's own, so `tm` leaves both.
- **Failed** when what PHPStan prints is not its JSON report, as for a config it cannot load or a `config` the rule
  names that does not exist; and when the report holds an error outside any file, such as a crashed worker, since
  the files it lists may then be partial: `phpstan reported an error outside any file: <error>`. PHPStan exits 1
  both for findings and for failures, so its exit code is not read. Failed too, before PHPStan runs, when the
  config's path holds `'`, `%` or a line break, which `tm` cannot write into the wrapper, or is not UTF-8.
- **`config`:** the repository config the wrapper includes, taken from the rule file's folder.
- **`params`:** none. Any param fails the rule before PHPStan runs, with the reason
  `phpstan param <name>: a phpstan rule takes no params`.
- **A finding** is placed at the message's line (none when PHPStan gives none), with no column, and its message is
  PHPStan's. The ` (in context of ...)` that PHPStan adds to a file's name for a finding in a trait is cut.
- **Runs alone.** A PHPStan rule never shares a run.

## dart

- **Available when** `dart` is on `PATH`, and the rule file's folder is inside a Dart package. `tm` asks git for the
  top of the repository, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Dart packages only.** The package is the nearest folder at or above the rule file's folder that holds a
  `pubspec.yaml`, looking no higher than the top of the repository. With none the rule is not available:
  `no pubspec.yaml at or above the rule file's folder: dart analyze checks a Dart package, and tm runs it only in one`.
- **No Dart file, nothing run.** A rule that covers files but no `.dart` file git lists runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before dart is looked for.
- **What runs:**

  ```
  dart --suppress-analytics analyze --format machine .
  ```

  twice: first in a temporary package `tm` makes and removes, to learn what the check is, then in the rule file's
  folder. `--suppress-analytics` turns dart's analytics off for the run without changing your setting. `tm` never
  runs `dart pub get`.
- **`check`:** one diagnostic code or lint name, in lower case, such as `unused_local_variable` or
  `prefer_single_quotes`. dart has no option that selects one: it reports everything it finds, and `tm` keeps the
  diagnostics whose code is the check. A rule's `config` does not reach dart.
- **Failed before dart runs** when `check` is anything else, such as an upper-case code, a list or a glob, with the
  reason
  ``dart check `<check>` is not one code: a dart check is one diagnostic code or lint name, lower case (unused_local_variable, prefer_single_quotes)``.
- **What the check is.** The temporary package's options name the check both as a diagnostic and as a lint, and
  dart warns of each it does not know. So `tm` learns one of three things:
  - dart knows no such code: the rule is not available, with dart's own words;
  - the code is a diagnostic: dart reports it wherever it finds it;
  - the code is a lint: dart runs it only where an `analysis_options.yaml` enables it.
- **A lint must be enabled.** A lint is not available unless every `analysis_options.yaml` that governs the folder
  enables it, itself or through a file it includes: as an item under `linter: rules:`, or as `<lint>: true` there.
  Any other value for it (`false`, a quoted `true`, nothing) counts as not enabled. The reason is
  ``<file> does not enable the lint `<check>`: dart runs a lint only where analysis_options.yaml enables it``, or
  starts `no analysis_options.yaml governs the rule's folder, so nothing enables` when there is no such file.
- **Which options files are read.** The nearest `analysis_options.yaml` at or above the rule file's folder, and each
  one on disk below it, in a folder git ignores or not, through symlinked folders, but not in a folder whose name
  starts with `.`; and every file each of them names with `include:`. `tm` reads them from disk as dart does.
- **Failed when the options hide findings.** dart cannot be told to ignore these, so each fails the rule and the
  reason names the file and the line:
  - a key `exclude`, in any case, anywhere in an options file, which covers `analyzer: exclude:`;
  - a value `ignore`, in any case, anywhere, which covers `analyzer: errors: <code>: ignore`;
  - an options file that is read from outside the repository: the nearest one above its top, a symlink's target, an
    include. A `package:` include is the package's declared dependency and is read wherever it lies.
- **Failed when `tm` cannot read an options file as dart does.** The files are read as YAML: block and flow style,
  every quoting, anchors and aliases. These forms fail the rule, and the reason names the file and the line:
  - text a YAML parser rejects;
  - a second document in the file;
  - a key that repeats in one mapping;
  - a `!!` tag on text that does not read as that type, and any `!!` tag other than `!!null`, `!!bool`, `!!int`,
    `!!float`, `!!str`, `!!map` on a mapping and `!!seq` on a sequence;
  - a key that is not a scalar;
  - a merge key `<<`;
  - an alias inside the node its anchor names;
  - more than 100000 entries, or more than 64 levels of nesting.

  dart drops a whole options file for some of these and reads others differently by section, so a rule over such a
  file could read clean for a lint dart never ran.
- **Not available** also when the package's dependencies were never fetched (it has no
  `.dart_tool/package_config.json`) and dart cannot find a `package:` URI:
  ``dart cannot resolve `<uri>`: the package's dependencies are not fetched (no .dart_tool/package_config.json); run `dart pub get` once in the package, since tm never fetches packages``.
  An `include:` that cannot be resolved, and an options file or folder that cannot be read, make the rule not
  available too, each with its own reason.
- **What dart still honours,** which `tm` cannot turn off: `// ignore: <code>` and `// ignore_for_file: <code>`
  comments, and their `type=lint` form.
- **Failed** when dart exits with anything but 0, 2 or 3, which are its answers (nothing or infos only, warnings,
  errors); when a line it prints is not a diagnostic; and when the first run, in the temporary package, exits with
  anything but 0 or 2.
- **`config`:** not used.
- **`params`:** none. Any param fails the rule before dart runs, with the reason
  `dart param <name>: a dart rule takes no params`.
- **A finding** is placed at the diagnostic's line and column, and its message is dart's.
- **Runs alone.** A dart rule never shares a run.

## actionlint

- **Available when** `actionlint` is on `PATH`. `tm` lists the files with git, so git must be there too. A rule
  whose check is `shellcheck` needs `shellcheck` on `PATH` as well, and one whose check is `pyflakes` needs
  `pyflakes`.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the `.yml` and `.yaml` files directly in `.github/workflows` under the rule file's
  folder that git lists (the tracked files, then the untracked files git does not ignore), that are regular files
  and not symlinks, and that the rule covers. A rule that covers files but none of these runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before actionlint is looked for.
- **What runs,** in the rule file's folder:

  ```
  actionlint -no-color -format <template> -config-file=<config> -shellcheck=<program> -pyflakes=<program> -- <workflow>...
  ```

  `<template>` makes actionlint print one JSON object: the kinds this run defined, and the errors it found.
  `SHELLCHECK_OPTS` is removed from the environment, since it can silence ShellCheck findings.
- **`check`:** one actionlint kind, such as `expression`, `action`, `runner-label` or `shellcheck`. actionlint has
  no option that selects a kind: every kind runs, and `tm` keeps the errors of the rule's kind.
- **ShellCheck and pyflakes run only for their own check.** For the check `shellcheck` the run gets
  `-shellcheck=shellcheck` and an empty `-pyflakes=`; for `pyflakes` the reverse; for any other check both are
  empty, so neither program runs. actionlint skips a kind without a word when its program is missing, so `tm` looks
  for the program first and the rule is not available without it, with `<program> not found on PATH`.
- **Failed before actionlint runs** when `check` is not lower-case words joined by single `-`, with the reason
  ``actionlint check `<check>` is not an actionlint check kind (lowercase words joined by -, such as expression)``.
- **Not available** when the check is not among the kinds the run defined. The reason lists them:
  ``actionlint <version> does not know the check `<check>`: the kinds this run defined are <kinds>``.
- **`config`:** always named, as `-config-file=`: the rule's `config`, taken from the rule file's folder; else the
  first of `.github/actionlint.yaml` and `.github/actionlint.yml` at the top of the repository, where actionlint
  itself looks; else `/dev/null`. So actionlint reads exactly the file `tm` read.
- **Failed when the config holds an `ignore`.** actionlint drops every finding an `ignore` pattern matches and has
  no option that turns that off. The reason names the file and the line and says
  `actionlint drops findings matching it and tm cannot turn it off; put exceptions in the rule file`. `tm` reads the
  config as YAML, so a quoted or escaped key and one reached through an alias are found. A config it cannot read (a
  repeated key, a second document, a merge key `<<`) fails the rule too, and so does one that cannot be opened.
  The config's `self-hosted-runner` and `config-variables` are the repository's own definitions and are left as
  they are.
- **What actionlint still honours,** which `tm` cannot turn off: a `# shellcheck disable=` comment inside a `run:`
  script.
- **Failed** when what actionlint prints is not that JSON object. actionlint exits 1 when it finds anything and 3
  when it fails, so its exit code is not read.
- **`params`:** none. Any param fails the rule before actionlint runs, with the reason
  `actionlint param <name>: an actionlint rule takes no params`.
- **A finding** is placed at the error's line and column (none when actionlint gives 0), and its message is
  actionlint's.
- **Runs alone.** An actionlint rule never shares a run.

## sqlfluff

- **Available when** `sqlfluff` is on `PATH`. `tm` lists the files with git, so git must be there too. The
  repository's sqlfluff config must set a dialect (below).
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files whose name ends, in any case, `.sql`, `.sql.j2`, `.dml`, `.ddl` or
  `.pkb`, sqlfluff's stock extensions, that git lists under the rule file's folder (the tracked files, then the
  untracked files git does not ignore), that are regular files and not symlinks, and that the rule covers. A rule
  that covers files but none of these runs nothing ([above](#a-rule-that-covers-no-file)); this is settled before
  sqlfluff is looked for.
- **What runs,** in the rule file's folder:

  ```
  sqlfluff lint --format json --nocolor --disable-progress-bar --disable-noqa --disregard-sqlfluffignores --exclude-rules= --ignore= --rules=<check> [--config=<config>] -- <file>...
  ```

- **The environment:** `HOME` is set to the top of the repository, so the configs sqlfluff reads from `~` and from
  the folders between `~` and each file are the repository's own. `XDG_CONFIG_HOME`, `PYTHONPATH` and
  `PYTHONUSERBASE` are removed, so no config, plugin or templater comes from outside what the repository names.
- **`check`:** one sqlfluff rule reference: a code (`LT01`), a name (`layout.spacing`), a group (`layout`) or an
  alias (`L039`), in sqlfluff's own case. sqlfluff runs that reference alone, and every violation in its report is a
  finding, at whatever code it carries, since a name or a group reports under other codes.
- **Failed before sqlfluff runs** when `check` is empty, starts with `.`, or holds a character other than an ASCII
  letter or digit, `_` or `.`, such as a comma, with the reason
  ``sqlfluff check `<check>` is not one sqlfluff rule code, name or group (as `LT01`, `layout.spacing` or `layout`), so tm cannot pass it as one rule``.
- **Not available** when sqlfluff says `Tried to allowlist unknown rule references` and names the check; it would
  otherwise lint with no rule and exit 0. The reason quotes that line.
- **A dialect is needed.** `tm` passes no `--dialect`: which SQL a repository writes is its own setting. With none
  set the rule is not available, with the reason
  ``no dialect is set in the repository's sqlfluff config: set `dialect` under `[sqlfluff]` in its .sqlfluff``.
- **What is turned off:** `-- noqa` comments (`--disable-noqa`), `.sqlfluffignore` files
  (`--disregard-sqlfluffignores`), and a config's `exclude_rules` and `ignore`, each overridden with an empty value.
  A violation the config lists under `warnings` is still a finding.
- **What sqlfluff still honours,** which `tm` cannot turn off: `-- sqlfluff:` directives written in a file, such as
  `-- sqlfluff:exclude_rules:` and `-- sqlfluff:rules:`. `standard@1` has no rule that reports them.
- **A stated limit:** a config that adds an extension to `sql_file_exts` gets no files of that extension checked,
  since `tm` picks the files by the stock extensions.
- **Failed** when sqlfluff exits with anything but 0 or 1; when what it prints is not its JSON report; when it
  reports that it could not parse, template or lex a file, with a reason that starts
  `sqlfluff could not read <file>, so it did not check it`; and when a file handed over is missing from its report,
  as one over its size limit or one whose extension the config removed:
  `sqlfluff did not check <files>; it said: <its stderr>`.
- **`config`:** passed as `--config=<config>`, as written, so a relative path is taken from the rule file's folder.
  sqlfluff lays it over the configs it finds.
- **`params`:** none. Any param fails the rule before sqlfluff runs, with the reason
  `sqlfluff param <name>: a sqlfluff rule takes no params`.
- **A finding** is placed at the violation's line and column, and its message is `<code>: <description>`.
- **Runs alone.** A sqlfluff rule never shares a run.

## hadolint

- **Available when** `hadolint` is on `PATH`. `tm` lists the files with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files named `Dockerfile`, `Containerfile` or `<name>.Dockerfile`, in that case,
  that git lists under the rule file's folder (the tracked files, then the untracked files git does not ignore),
  that are regular files and not symlinks, and that the rule covers. `Dockerfile.dev` and `dockerfile` are not
  among them. A rule that covers files but none of these runs nothing ([above](#a-rule-that-covers-no-file)); this
  is settled before hadolint is looked for.
- **A list of codes built into `tm`.** hadolint takes any text as a code and never says it does not know one. So a
  rule's code is looked up in a list of hadolint 2.15.1's 71 `DL` codes, built into `tm` and never fetched.
- **`check`:** one `DL` code on that list, such as `DL3006`. It reaches hadolint as `--error=<check>`, which also
  turns on a rule that is off by default. Results with exactly that code are the findings.
- **Failed before hadolint runs** when `check` is not `DL` or `SC` followed by digits, with the reason
  ``hadolint check `<check>` is not one hadolint code such as `DL3006`, so tm cannot tell which findings it selects``.
- **Not available** when the code is well formed and not on the list, with the reason
  `hadolint <version>: <check> is not in temperare's list of hadolint codes (taken from hadolint 2.15.1)`; nothing
  runs for it. Every `SC` code is in this case: hadolint runs ShellCheck without its optional checks and leaves out
  codes of its own, so `tm` cannot list the `SC` codes it reports, and the reason says so. `DL1000` is not on the
  list either: it is hadolint's report that it could not parse a file, not a rule. This is settled in the run, after
  the files and `PATH` are looked at.
- **What runs,** in the rule file's folder:

  ```
  hadolint --no-color -f sarif --disable-ignore-pragma --error=<check> --config=<config> -- <file>...
  ```

  with every `HADOLINT_*` variable removed from its environment (`HADOLINT_IGNORE` among them), so your environment
  cannot change a rule's answer.
- **`config`:** always named, which makes hadolint read that file and no other: the rule's `config`, taken from the
  rule file's folder; else the folder's `.hadolint.yaml`; else its `.hadolint.yml`; else an empty config `tm` writes
  for the run. So a hadolint config elsewhere on your machine is never read.
- **`# hadolint ignore=` comments silence nothing** (`--disable-ignore-pragma`), the `global ignore` form included.
- **Failed when the config lists anything under `ignored`.** hadolint has no option that takes that list back. The
  reason starts
  ``<file>: line <n>: `ignored` hides hadolint findings outside the rule file; put exceptions in the rule file``. A
  config that cannot be opened, or that `tm` cannot read as YAML (a repeated key, a second document, a merge key
  `<<`), fails the rule too.
- **What hadolint still honours,** which `tm` cannot turn off: a `# shellcheck disable=` comment inside a `RUN`
  script.
- **Failed** when hadolint says it could not read its config (it would then run on its defaults); when it reports
  `DL1000` for a file, which it could not parse and whose other findings are then missing; and when what it prints
  is not one SARIF log. Any exit code with a SARIF log on stdout is otherwise an answer.
- **`params`:** none. Any param fails the rule before hadolint runs, with the reason
  `hadolint param <name>: a hadolint rule takes no params`.
- **A finding** is placed at the result's line and column, and its message is hadolint's.
- **Runs alone.** A hadolint rule never shares a run.

## yamllint

- **Available when** `yamllint` is on `PATH`. `tm` lists the files with git, so git must be there too.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files whose name ends `.yaml` or `.yml`, and those named `.yamllint`, that git
  lists under the rule file's folder (the tracked files, then the untracked files git does not ignore), that are
  regular files and not symlinks, and that the rule covers. A rule that covers files but none of these runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before yamllint is looked for.
- **What runs,** in the rule file's folder:

  ```
  yamllint -f parsable -d <config text> -- <file>...
  ```

  `<config text>` is `{extends: <base>, ignore: [], rules: {"<check>": {ignore-from-file: []}}}`.
- **`check`:** one yamllint rule id, such as `line-length` or `truthy`. Naming it in the config text turns it on
  even where the base config turns it off; where the base gives it options, they are kept. Problems of that rule
  are the findings, at `error` or `warning` level alike.
- **Failed before yamllint runs** when `check` is not a lower-case letter followed by lower-case letters, digits and
  `-`, with the reason
  ``yamllint check `<check>` is not a yamllint rule id; a yamllint check is one rule id, such as `line-length` ``.
- **Not available** when yamllint says `invalid config: no such rule: "<check>"`. The reason quotes that line.
- **`config`:** the base the config text extends: the rule's `config`, taken from the rule file's folder; else the
  nearest `.yamllint`, `.yamllint.yaml` or `.yamllint.yml` from the rule file's folder up to the top of the
  repository; else yamllint's own `default`. `-d` takes the place of yamllint's own search, so a config in your home
  folder is never read. A `config` the rule names that is not a file fails the rule:
  `the rule's yamllint config <path> is not a file`.
- **What is turned off:** the config's top-level `ignore` and `ignore-from-file`, and the check's own, each
  replaced by the empty lists in the config text, so every file handed over is linted.
- **What yamllint still honours,** which `tm` cannot turn off: `# yamllint disable`, `disable-line` and
  `disable-file` comments.
- **Stated limits.** A config's own `yaml-files` is not read: it steers only yamllint's walk of a folder, and `tm`
  hands over files, so a file it adds under another name is not checked. yamllint runs in the rule file's folder, so
  a relative `extends` or `ignore-from-file` in a config above that folder resolves from the rule file's folder,
  not from beside the config.
- **Failed** when yamllint exits with anything but 0 or 1, as for a config it refuses (255); when it writes
  anything to stderr, which is the only sign of a base config it could not read; when it reports a `syntax` problem,
  since a file it could not read as YAML was not fully checked
  (`yamllint could not read <file> as YAML at line <n>, so its findings there are incomplete: <message>`); and when
  a line it prints is not a problem line.
- **`params`:** none. Any param fails the rule before yamllint runs, with the reason
  `yamllint param <name>: a yamllint rule takes no params`.
- **A finding** is placed at the problem's line and column, and its message is yamllint's, without the level and
  the rule id.
- **Runs alone.** A yamllint rule never shares a run.

## vale

- **Available when** `vale` is on `PATH`. `tm` lists the files with git, so git must be there too. The rule also
  needs a Vale config inside the repository that uses the check's style (below).
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** only files git lists under the rule file's folder (the tracked files, then the
  untracked files git does not ignore), that are regular files and not symlinks, that the rule covers, and that are
  in a format Vale reads on its own, by extension as written:
  - markup, whose prose Vale reads: `md`, `markdown`, `mdown`, `mkd`, `mkdn`, `mdx`, `rst`, `adoc`, `asciidoc`,
    `asc`, `html`, `htm`, `xhtml`, `org`, `txt`, `text`;
  - code, of which Vale reads only the comments: `c`, `h`, `cc`, `cpp`, `hpp`, `cs`, `go`, `java`, `js`, `jsx`,
    `ts`, `tsx`, `kt`, `kts`, `php`, `pl`, `proto`, `py`, `r`, `rb`, `rs`, `scala`, `scss`, `css`, `less`, `swift`,
    `lua`.

  Left out: what Vale would read as plain text, code and data included (such as `sh`, `json`, `yaml`, `toml`, `sql`
  and a file with no extension), and what needs another program (`dita`, `xml`). A rule that covers files but none
  of these runs nothing ([above](#a-rule-that-covers-no-file)); this is settled before Vale is looked for.
- **What runs,** both in the rule file's folder:

  ```
  vale --no-global [--config=<config>] ls-config
  vale --no-global --output=JSON --filter=<expression> --config=<wrapper> -- <file>...
  ```

  The first reads the repository's Vale config as Vale resolves it, and runs with `PWD` removed from its
  environment. The second is the check. `--no-global` keeps your global config out of both. Nothing is fetched:
  `tm` never runs `vale sync`, so a style must already be on the repository's `StylesPath`, or be Vale's built-in
  `Vale`.
- **`check`:** one Vale rule, written `Style.Rule`, such as `Vale.Spelling`. `<expression>` keeps that rule alone.
  Alerts of exactly that rule are the findings, at every level.
- **Failed before Vale runs** when `check` is not two parts joined by one `.`, each of ASCII letters, digits, `_`
  or `-`, with the reason
  ``vale check `<check>` is not a Vale rule name (Style.Rule, each part ASCII letters, digits, _ or -)``.
- **`config`:** a `config` the rule names stands in for the repository's `.vale.ini`: it is the config the first
  run reads, and with it no `.vale.ini` is needed. It meets the same checks as a config Vale finds. It is not passed
  to the check run.
- **Not available** when:
  - Vale finds no config at or above the rule file's folder and the rule names none:
    `Vale finds no config (.vale.ini, _vale.ini, vale.ini, .vale or _vale) at or above the rule file's folder: tm runs Vale only with the repository's own config`;
  - a config Vale read is outside the repository. Vale looks upward past the repository's top, so that one would be
    another project's or your own;
  - no `BasedOnStyles` in the config names the check's style:
    ``no BasedOnStyles in <config> names the style `<style>`: tm never turns on a Vale style the repository did not choose``;
  - no rule Vale loaded has the check's name. `<expression>` makes Vale stop with an error then, where a plain
    filter would keep no rule and read clean. The reason quotes Vale's error.
- **The wrapper config.** The check run is pointed at a `.vale.ini` that `tm` writes in a new temporary folder,
  never in the repository, and removes after the run. It holds the repository's `StylesPath` and `Vocab`,
  `MinAlertLevel = suggestion`, and one section, `[*]`, with `BasedOnStyles = <the check's style>`. So an alert of
  every level is reported, and the rule's `severity` decides what it counts for.
- **What is turned off.** Nothing else of the repository's config reaches Vale: its skip keys (`BlockIgnores`,
  `TokenIgnores`, `IgnoredScopes`, `SkippedScopes`, `IgnoredClasses`), its sections and their `Style.Rule = NO`
  lines are not copied into the wrapper.
- **What Vale still honours,** which `tm` cannot turn off: `<!-- vale off -->` and `<!-- vale Style.Rule = NO -->`
  comments in a file.
- **A stated limit: `Vocab` stays on.** The repository's `Vocab` is copied into the wrapper and still applies. It
  is the one setting of the repository's config, beside `StylesPath`, that reaches the check run.
- **Failed** when Vale cannot load the config, or cannot find a `config` the rule names, with Vale's own message;
  when the check run writes an error to stderr, even beside findings, since Vale then left a file unread
  (`vale stopped with <code>: <text>`); and when what it prints is not its JSON. Vale exits 1 for an alert at error
  level and 0 otherwise, so its exit code is not read.
- **`params`:** none. Any param fails the rule before Vale runs, with the reason
  `vale param <name>: a vale rule takes no params`.
- **A finding** is placed at the alert's line and the first column of its span, and its message is Vale's.
- **Runs alone.** A Vale rule never shares a run.

## stylelint

- **Available when** `stylelint` is on `PATH`. `tm` lists the files with git, so git must be there too. `tm` sets
  nothing in stylelint's environment, `NODE_PATH` included.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files ending `.css`, in any case, that git lists under the rule file's folder
  (the tracked files, then the untracked files git does not ignore), that are regular files and not symlinks, and
  that the rule covers. A rule that covers files but none of these runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before stylelint is looked for.
- **A stated limit: CSS only.** SCSS, Less and CSS inside HTML or JavaScript need a `customSyntax` the repository
  installs, so they are not checked.
- **What runs,** in the rule file's folder. First, once for each file:

  ```
  stylelint --print-config [--config=<config>] ./<file>
  ```

  then the check:

  ```
  stylelint --formatter json --ignore-disables --ignore-path=/dev/null --disable-default-ignores --allow-empty-input --no-cache [--config=<config>] --suppress-location=<empty file> ./<file>...
  ```

  `--no-cache` keeps stylelint from writing `.stylelintcache`. The first run costs one Node process for each file.
- **`check`:** one stylelint rule name, such as `color-no-invalid-hex` or `scss/at-rule-no-unknown`. stylelint has
  no option that turns one rule on, so its config decides, and `tm` asks what the config is for each file before
  the check runs. Warnings of exactly that rule are the findings.
- **Failed before stylelint runs** when `check` is empty, is `@` alone, or holds a character other than a
  lower-case ASCII letter, a digit, `-` or `/` (a leading `@` apart), with the reason
  ``stylelint check `<check>` is not a stylelint rule name; tm matches findings by it``.
- **Not available** when the config turns the check on for none of the rule's files:
  `stylelint's config does not turn on <check> for any file the rule covers`. Not available too when stylelint
  reports `Unknown rule <check>`: its config names the rule and stylelint does not have it, often because a plugin
  is not loaded. The reason quotes its text.
- **Failed, before the check runs,** when the config:
  - turns the check on for some of the rule's files and off for others, an exception hidden in the config:
    `stylelint's config turns <check> off for <file>; exceptions go in the rule file`;
  - turns on `fix`, which would make stylelint rewrite the files it checks, and which no option stops:
    `stylelint's config for <file> turns on fix, which rewrites the checked files; tm never edits the code it checks`;
  - turns on `quiet`, which drops findings of severity warning;
  - does not load, with stylelint's own message.
- **What is turned off:** `stylelint-disable` comments (`--ignore-disables`); `.stylelintignore`
  (`--ignore-path=/dev/null`); stylelint's default ignores, such as `node_modules` (`--disable-default-ignores`);
  and a `stylelint-suppressions.json`, which is never read, since stylelint is pointed at an empty one `tm` writes
  for the run and removes.
- **Failed** when the report shows a file was not linted as configured: one the config's `ignoreFiles` covers
  (`stylelint ignored <file> through ignoreFiles in its config; exceptions go in the rule file`), one stylelint
  rewrote, one it could not parse (`stylelint could not parse <file>: <text>`), or one whose config gives a rule an
  invalid option. Failed too when stylelint prints no report, as for a fatal or usage error. Any exit code with a
  report is otherwise an answer. stylelint writes its JSON report to stderr, which is where `tm` reads it.
- **`config`:** passed as `--config=<config>` to every run, as written, so a relative path is taken from the rule
  file's folder. Without one, stylelint finds its own configuration.
- **`params`:** none. Any param fails the rule before stylelint runs, with the reason
  `stylelint param <name>: a stylelint rule takes no params`.
- **A finding** is placed at the warning's line and column, and its message is stylelint's.
- **Runs alone.** A stylelint rule never shares a run.

## markdownlint

The engine `markdownlint` runs the program `markdownlint-cli2`.

- **Available when** `markdownlint-cli2` is on `PATH`. `tm` lists the files with git, so git must be there too.
  `tm` sets nothing in its environment, `NODE_PATH` included.
- **Where:** the rule file's folder, which must be inside a git repository.
- **Which files it is handed:** the files ending `.md` or `.markdown` that git lists under the rule file's folder
  (the tracked files, then the untracked files git does not ignore), that are regular files and not symlinks, and
  that the rule covers. A rule that covers files but none of these runs nothing
  ([above](#a-rule-that-covers-no-file)); this is settled before markdownlint-cli2 is looked for.
- **A list of rules built into `tm`.** markdownlint never reports a rule it does not have. So a rule's `check` is
  looked up in a list of markdownlint 0.42.2's 53 rules, each with its id and aliases, built into `tm` and never
  fetched.
- **`check`:** one markdownlint rule, by its id or an alias, such as `MD013` or `line-length`, in any case. A tag,
  such as `headings`, names a group of rules and is not a check. Results that carry the check among their rule names
  are the findings.
- **Failed before markdownlint-cli2 runs** when `check` is empty or holds a character other than an ASCII letter,
  a digit or `-`, with the reason
  ``markdownlint check `<check>` is not a markdownlint rule name (letters, digits and `-`, as MD013 or line-length)``.
- **What runs,** in the rule file's folder:

  ```
  markdownlint-cli2 --no-globs --config <options module> :<file>...
  ```

  Each file is passed as a literal path, never a glob. `<options module>` is a file `tm` writes in a new temporary
  folder and removes after the run. It turns inline config off, turns fixing off, and prints the results as one JSON
  array, since markdownlint-cli2 ships no JSON output of its own.
- **`config`:** not taken. A rule that names one fails, once markdownlint-cli2 is found, with the reason
  `a markdownlint rule takes no config: markdownlint reads the repository's own .markdownlint.json or .yaml in the rule file's folder and below`.
- **The repository's own config applies.** markdownlint-cli2 reads the first of `.markdownlint.jsonc`,
  `.markdownlint.json`, `.markdownlint.yaml`, `.markdownlint.yml`, `.markdownlint.cjs` and `.markdownlint.mjs` in a
  folder; the nearest folder's applies to a file; and it looks in the rule file's folder and below, never above it
  or in your home folder. `tm` reads the configs that apply to the rule's files, to learn whether they turn the
  check off.
- **Not available** when:
  - the check is not on the list, with the reason
    `markdownlint <version>: <check> is not in temperare's list of markdownlint rules (taken from markdownlint 0.42.2)`,
    `<version>` being markdownlint-cli2's;
  - a config that applies turns the rule off, by `default: false`, or by its id, an alias or one of its tags set to
    a false value, or by `enabled: false` under one of those:
    ``<config> turns <id> off with its `<key>` setting: tm runs markdownlint with the repository's own config, so the rule would report nothing``.

  Both are settled in the run, after the files and `PATH` are looked at.
- **Stated limits, each of which fails the rule.** `tm` reads only `.markdownlint.json`, `.yaml` and `.yml`:
  - a `.markdownlint.jsonc`, `.cjs` or `.mjs` that applies to a file of the rule. Give the config as JSON or YAML;
  - a config with an `extends` key, which `tm` does not follow;
  - a config with a repeated key, or one `tm` cannot read;
  - a YAML config whose text markdownlint-cli2 may read as JSON or TOML first (it starts with `{` or `[` and is
    not plain JSON, its top level is not a mapping, or a top-level key holds `=`);
  - a YAML config with a quoted or tagged value that spells false, zero or null, which `tm` cannot tell from the
    plain one.
- **Failed when an options file is present.** A `.markdownlint-cli2.jsonc`, `.markdownlint-cli2.yaml`,
  `.markdownlint-cli2.cjs` or `.markdownlint-cli2.mjs` in the rule file's folder, or in a folder between it and a
  file of the rule, would override `tm`'s own options: it could turn fixing on, ignore files or change the output.
  The reason names it and says to remove it. It is found by name, tracked or not.
- **Inline comments silence nothing:** `<!-- markdownlint-disable -->` and its other forms, and
  `<!-- markdownlint-configure-file -->`, are turned off by `tm`'s options.
- **Failed** when what markdownlint-cli2 prints is not that JSON array, as for a config it cannot load. It exits 0
  when clean, 1 with findings and 2 for a usage or config error, so its exit code is not read.
- **`params`:** none. Any param fails the rule before markdownlint-cli2 runs, with the reason
  `markdownlint param <name>: a markdownlint rule takes no params`.
- **A finding** is placed at the result's line, and at the column of its range when markdownlint gives one. Its
  message is worded as markdownlint-cli2's own output: the rule's names joined by `/`, its description, then the
  detail and the context in brackets when there are any.
- **Runs alone.** A markdownlint rule never shares a run.

## tessera

A rule may name `tessera` as its engine. The rule file accepts it, and a Tessera rule that covers a file is reported
"not available" with the reason `Tessera's slop records are not defined yet`. This is so whether or not Tessera is
installed: `tm` neither runs it nor looks for it on `PATH`.

Two cases are settled first, as for any engine ([the order](#the-order-a-rule-settles-in)). A Tessera rule takes
no params: one with any param is failed (exit 5), with the reason
`tessera param <name>: a tessera rule takes no params`. And a Tessera rule that covers no file git lists runs
nothing and covers nothing ([above](#a-rule-that-covers-no-file)).

A run with a not-available Tessera rule ends with exit 3, unless a quality error (1) or a failed engine (5) comes
first. The `next:` line says `Tessera's checks are not available yet` and does not ask you to install it. To get an
exit 0, leave Tessera rules out of the rule file, or check with a rule file that has none
(`--rules <path>`).

A Tessera rule with `only_with_base = true`, run without `--base`, is not applicable rather than not available, as
any such rule is ([`OUTPUT.md`](OUTPUT.md#new-findings-only---base)).

## The engines version 0.1.0 was accepted with

The acceptance checks for 0.1.0 ran against clippy 0.1.95, SwiftLint 0.65.1, ruff 0.17.4, ast-grep 0.51.1,
golangci-lint 2.15.0 (built with go 1.27.2), pylint 4.1.4, clang-tidy 23.1.6 (Homebrew LLVM), vulture 2.16,
Periphery 3.8.0, gitleaks 8.30.1 and cargo-audit 0.23.0.

ESLint, mypy, PMD, detekt, RuboCop, PHPStan, dart, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint and
markdownlint are not among them. Each of them is tested on output recorded from one version of the
engine: ESLint 10.15.0, mypy 2.4.1, PMD 7.29.0, detekt 1.23.8, RuboCop 1.92.3, PHPStan 2.3.7, dart 3.14.1,
actionlint 1.7.12, sqlfluff 4.5.0, hadolint 2.15.1, yamllint 1.38.0, Vale 3.28.1, stylelint 18.1.0 and
markdownlint-cli2 0.24.2 (which carries markdownlint 0.42.2).

Temperare reads each engine's own output, so another version of an engine may print something it reads
differently. If a rule stops reporting after an engine upgrade, try it on a file you know breaks it.

# Security Policy

Code Chi Tao (`cctao`) runs other programs over your code, with your privileges, and tells you what they found. If you find a
problem that touches that trust, please report it privately.

## Security by construction

These are properties of the code, not of how it is deployed.

- **Programs are run directly, never through a shell.** Each argument reaches the program as one argument, as
  written. A `--base` value is always read by git as a revision, never as an option, and the paths you give on the
  command line are never passed to git or to an engine: they only cut the result.
- **A rule's value is passed as a value, not as an option.** A `check`, `config` or `params.allow` value that
  begins with `-` is a problem in the rule file, and nothing runs. Each value reaches its engine joined to its own
  flag, as in `--select=<check>`. Where an engine has no such flag, the `check` goes inside text `cctao` writes
  (ESLint's `--rule` JSON, yamllint's `-d` config, Vale's `--filter` expression and wrapper config, detekt's config
  layer, the options of dart's probe package), and it is held to a narrow set of characters first, so it cannot end
  that text. A `config` path written into such text is quoted there (PHPStan's wrapper config, yamllint's `-d`
  config), and a PHPStan config path that holds `'`, `%` or a line break fails the rule.
- **An ast-grep rule file is read only from inside the rule file's folder.** A `check` that is an absolute path, or
  that resolves outside the folder through `..` or a symlink, fails the rule and ast-grep is not run. On Unix the
  file is opened one path component at a time from that folder, following no symlink, and a file with a second
  hard link is refused. On other systems only the path checks apply.
- **The scripts listed for a ShellCheck rule never include a symlink.** `cctao` picks them among the files git lists
  in the rule file's folder and leaves out every path that is a symlink or lies under a symlinked folder, whatever
  it points at, and everything that is not a regular file. To tell a script that has no extension it reads at most
  the file's first 257 bytes, on Unix only, opened without following a symlink and without waiting on a FIFO; on
  other systems it reads none and such a file is left out.
- **The sources listed for a clang-tidy rule never include a symlink.** `cctao` picks them among the files git lists
  in the rule file's folder by their extension, keeps only regular files that are not symlinks, and reads none of
  them itself.
- **The files handed to mypy, PMD, detekt, PHPStan, actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint and
  markdownlint-cli2 never include a symlink.** `cctao` picks them by name or extension among the files git lists in
  the rule file's folder (the tracked files, and the untracked files git does not ignore), keeps only regular
  files that are not symlinks, and reads none of them itself. A file's name is kept apart from the options: the
  files come after `--` for mypy, PHPStan, actionlint, sqlfluff, hadolint, yamllint and Vale, as `./<file>` for
  stylelint, as `:<file>` for markdownlint-cli2, in a list file for PMD, and as the value of `--input` for detekt.
  ESLint, RuboCop and dart are handed the folder instead, and their own config decides which files they read.
- **Two places start programs.** The engines, the question each is asked for its version, and the git that
  `cctao baseline test` runs in the case repositories it makes go through one small crate of Tao's, `procio`.
  The git questions `cctao check` and `cctao rules` ask of your repository (its top, a revision, the files git lists, the
  lines a change adds, a file at the base) go through `ccvcslib`, the Code Chi version-control library, which
  starts git itself. The repository's own rule file holds the line for Tao's own code: a rule reports any code
  outside `procio` that starts a process. It does not read `ccvcslib`, which is a dependency.
- **It does not edit what it checks.** `cctao check` and `cctao rules` write to stdout and stderr. `cctao check` creates
  temporary files in the system's temporary folder for twelve kinds of rule, and only for them:
  - an ast-grep rule that gets as far as its scan (each one, with or without `params`): a copy of its rule file
    with its params filled in, `cctao-ast-grep-<pid>-<n>.yml`;
  - a SwiftLint rule with `max_complexity` or `max_lines`: the threshold as a SwiftLint config,
    `cctao-swiftlint-<pid>-<n>.yml`;
  - a clippy rule with `max_nesting`: a new folder `cctao-clippy-<pid>-<n>` holding one `clippy.toml` with the
    threshold;
  - a Periphery rule that gets as far as its run: a new folder `cctao-periphery-<pid>-<n>` holding the package's
    build and its index, a scratch path for each local package's manifest reading, SwiftPM's description of the
    package and an empty Periphery config. Nothing is built in
    the repository or in DerivedData;
  - a PMD rule that gets as far as its run: the list of the files PMD is to read, `cctao-pmd-files-<pid>-<n>.txt`;
  - a detekt rule that gets as far as its run: a config layer, `cctao-detekt-<pid>-<n>.yml`, and an empty file,
    `cctao-detekt-report-<pid>-<n>.sarif`, which detekt is told to write its report to;
  - a PHPStan rule that gets as far as its run: a new folder `cctao-phpstan-<pid>-<n>` holding a wrapper config
    `cctao.neon`, and beside it the `cache` folder PHPStan is told to keep its own temporary files in;
  - a dart rule that gets as far as its probe: a new folder `cctao-dart-probe-<pid>-<n>` holding a `pubspec.yaml` and
    an `analysis_options.yaml`, a package of Tao's own that dart is run in once to learn what the check is;
  - a hadolint rule that gets as far as its run with no `config` of its own and no `.hadolint.yaml` or
    `.hadolint.yml` in the rule file's folder: an empty config, `cctao-hadolint-config-<pid>-<n>.yaml`;
  - a Vale rule that gets as far as its check run: a new folder `cctao-vale-<pid>-<n>` holding a wrapper config
    `.vale.ini`;
  - a stylelint rule that gets as far as its check run: an empty suppressions file,
    `cctao-stylelint-suppressions-<pid>-<n>.json`;
  - a markdownlint rule that gets as far as its run: a new folder `cctao-markdownlint-<pid>-<n>` holding an options
    module `cctao.markdownlint-cli2.cjs`, a few lines of JavaScript of Tao's own that markdownlint-cli2 loads.

  Each is created new, never opened over an existing file or folder, and removed when the run of that rule ends
  (the dart package as soon as its probe has run). A folder is removed with the files Tao wrote in it;
  PHPStan's `cache` and everything in the Periphery folder are removed too. If an engine left anything else in one
  of the other folders, that folder stays, with what the engine left. If `cctao` is killed in between, it is left
  there. No config file of the repository is written. A ruff threshold is passed as an argument, and a ShellCheck,
  git, pylint, clang-tidy, golangci-lint, gitleaks, cargo-audit, vulture, ESLint, mypy, RuboCop, actionlint,
  sqlfluff or yamllint rule writes no file. The runs are told to write no cache: mypy with
  `--cache-dir=/dev/null`, PMD with `--no-cache`, RuboCop with `--cache false` (and `--no-server`, so it starts no
  server) and stylelint with `--no-cache`. What an engine keeps for itself outside the repository, as dart keeps
  its analysis cache under `~/.dartServer`, is that engine's own. A
  golangci-lint run passes `--fix=false`, so a repository config that asks golangci-lint to fix cannot edit the
  code, and runs go with `GOFLAGS` removed, so go does not rewrite `go.mod` or `go.sum`. A stylelint config that
  turns on `fix` would make stylelint rewrite the files it checks, so `cctao` asks stylelint for the config of each
  file first and fails the rule before the check runs. markdownlint-cli2 is run with fixing off in Tao's
  options module, and a markdownlint-cli2 options file of the repository's, which could turn it back on, fails the
  rule before the check runs. A RuboCop rule fails before its check runs when the rule file's folder holds a
  `.rubocop` file, which RuboCop reads as more options, and `RUBOCOP_OPTS` is set empty for the check and the runs
  after it, so neither can add an option to them, such as one that writes a file. A rule that is refused, or
  that covers no file, writes none either. Besides these, `cctao check` appends one line to its savings log,
  `~/.local/state/cctao/savings.jsonl`, when an engine ran for a rule other than a `watch` rule, making that
  folder when it is missing.
  The line holds the time, the rule file's folder as an absolute path, the verb and two token counts; no code and no
  finding. And each `cctao check` that ends in a report (every exit but 2) appends one line to its repository's
  telemetry file, `~/.local/state/cctao/telemetry/<root>.jsonl`, making those folders when they are missing
  ([`OUTPUT.md`](docs/reference/OUTPUT.md#the-telemetry-file)). That line does hold findings: the rule file's folder as
  an absolute path, the commit ids of HEAD and the base, and each rule's kept findings with their paths, lines and
  the engine's message, which can name what the engine found in the code. A gitleaks finding's message is still
  Tao's fixed text, never the secret. Both files are made with the permissions your umask gives.
- **No network code.** Tao opens no connection. Its dependencies are parsers (TOML, JSON, YAML, globs,
  regular-expression syntax) and their support crates, and on Unix `libc`, for the calls that open a file without
  following a symlink: an ast-grep rule file, the first bytes of a file that may be a shell script, the savings
  log and the telemetry file.
- **A check reads nothing back between runs.** There is no cache and no background process. The files Tao
  keeps are the record `cctao install` writes (see [What install touches](#what-install-touches)) and the savings
  log and the telemetry files above, which it only appends to and never reads. It reads one file of the user's,
  `~/.config/cctao/config.toml`, for the switch that turns the savings line off, and never writes it.
- **git is pointed at one repository.** git runs with the variables that could redirect it to another repository,
  index or object store removed, with credential prompts turned off, and without taking the index lock.
- **A program it runs cannot read your terminal.** The caller's stdin is never passed on.
- **"Not available" is never "clean".** A rule that has a file to check and whose engine is missing, or that cannot
  be run, is reported as such, is counted, and has its own exit code. A rule that failed has another, and a rule
  `cctao` refuses from the rule file alone fails whether or not its engine is installed. A rule that covers no file is
  not run, and the report says so ([below](#what-this-does-not-defend-against)).

## What this does not defend against

Limits, as plainly as possible.

**The rule file is trusted input.** `tao.toml` decides which engines run, what each is asked, and which config
file each is given. Tao keeps a rule's value from being read as an engine option and keeps an ast-grep rule
file inside the rule file's folder, as above. It does not judge what a value asks for. A `config` is handed to
SwiftLint, ruff, ShellCheck, pylint, clang-tidy, golangci-lint, gitleaks, ESLint, RuboCop, sqlfluff or stylelint as
written, and to mypy, detekt, PHPStan, actionlint, hadolint or yamllint as a path taken from the rule file's
folder. Either way it may name any file, inside the repository or not, and Tao itself opens the one given to
mypy, actionlint or hadolint, to look for a setting that would hide findings. A Vale `config` is the exception:
one outside the repository makes the rule not available. PMD and dart are given no `config`, and a markdownlint
rule that names one fails. An ast-grep rule file is the
repository's own, and ast-grep runs whatever it holds over the code. Running `cctao check` in a repository is running
that repository's instructions.

**The engines are trusted programs.** Tao runs each engine's program with your privileges: git, cargo (with
`cargo-clippy` and `clippy-driver`), SwiftLint, ruff, ast-grep, ShellCheck, pylint, clang-tidy, golangci-lint and
go, gitleaks, cargo-audit, vulture, Periphery and swift, ESLint, mypy, PMD, detekt, RuboCop, PHPStan and php, dart,
actionlint, sqlfluff, hadolint, yamllint, Vale, stylelint and markdownlint-cli2 (the program of the engine
`markdownlint`). It looks for each in the absolute entries of your `PATH`, skipping an empty or relative entry, and
runs whatever is there, by the full path it found. On Windows it tries each name with the extensions your `PATHEXT`
lists, in order (`.COM;.EXE;.BAT;.CMD` when it lists none). A name it does not find is never started, so the system
does not go looking for it in the working folder. Two are found another way:

- **ESLint** is an executable `eslint` in the rule file's folder's `node_modules/.bin` when there is one (on
  Windows, `eslint` with each extension your `PATHEXT` lists, so npm's `eslint.cmd`), run by its absolute path, and
  the `eslint` on `PATH` only when there is none. The folder's `eslint` is the repository's own code, trusted as
  its ESLint config and the plugins it loads are ([below](#running-it-on-code-you-do-not-trust)). stylelint and
  markdownlint-cli2 are taken from `PATH` only.
- **git, for the questions asked through `ccvcslib`,** is found by `ccvcslib`, once per run: the first `git` in
  the absolute entries of your `PATH`, skipping an empty or relative entry (on Windows, `git` with each extension
  your `PATHEXT` lists, `.EXE` when it is not set). `ccvcslib` starts that full path, so a `git` in the working
  folder is never run.

Some of them start programs of their own, which they find on `PATH` themselves: the `eslint`, `stylelint` and
`markdownlint-cli2` scripts are run by `node`, and the `phpstan` launcher by `php` (`cctao` checks that `node` or `php`
is on `PATH`, and the rule is not available when it is not); actionlint runs `shellcheck` or `pyflakes` for a rule
whose check is that name, and neither for any other check. PMD and detekt each start a Java they find for
themselves; `cctao` looks for none. Whoever controls `PATH` (on Windows, `PATHEXT` too) controls what runs. There is
no sandbox. The twenty-eighth engine, Tessera, is the exception: a rule that names it is reported not available, and
`cctao` neither looks for it nor runs it.

**The engines get your environment, with a few variables changed.** Tao sets and removes the variables
listed here and no others, so every other variable reaches the engine as you set it:

- git, for the questions asked through `ccvcslib`: the seventeen `GIT_*` variables that could point it at another
  repository, work tree, index, object store or ref namespace, change where it stops looking for the repository or
  how it reads paths, or give it config for that one run (`GIT_CONFIG`, `GIT_CONFIG_PARAMETERS`,
  `GIT_CONFIG_COUNT`) are removed, and so is each `GIT_CONFIG_KEY_<n>` and `GIT_CONFIG_VALUE_<n>`; your own config
  files (`GIT_CONFIG_GLOBAL`, `GIT_CONFIG_SYSTEM`, `GIT_CONFIG_NOSYSTEM`) are kept. `GIT_TERMINAL_PROMPT=0` and
  `GIT_OPTIONAL_LOCKS=0` are set, so it asks for no credentials and takes no index lock, and `LC_ALL=C` is set, so
  its messages read the same in every locale. The git that `cctao baseline test` runs in its case repositories goes
  through `procio`, with twelve of those seventeen removed and the same three set;
- clippy, for a rule with `max_nesting`: `CLIPPY_CONF_DIR` is set to Tao's temporary folder, which holds the
  `clippy.toml` clippy then reads;
- ShellCheck: `SHELLCHECK_OPTS` is removed, since ShellCheck reads it as more options, which could silence a rule;
- golangci-lint: `GOTOOLCHAIN=local`, `GOPROXY=off` and `GOSUMDB=off` are set and `GOFLAGS` is removed, which
  keeps go local and its module files unwritten, and `PWD` is set to the folder golangci-lint runs in, which it
  makes its paths from;
- gitleaks: `GITLEAKS_CONFIG` and `GITLEAKS_CONFIG_TOML` are removed, so a config named there cannot take the
  place of the rule's or the folder's;
- vulture: `PYTHONWARNINGS=ignore` is set, so a warning Python prints for a file vulture read is not taken for a
  failure, and a `PYTHONWARNINGS=error` of yours does not turn one into an error;
- PMD: `CLASSPATH` is removed, since PMD's launcher would put it ahead of PMD's own rulesets;
- detekt: `JAVA_TOOL_OPTIONS`, `JDK_JAVA_OPTIONS` and `_JAVA_OPTIONS` are removed, since each could load an agent
  into detekt's JVM or change what it prints; `JAVA_HOME` is kept;
- RuboCop, for the check, the runs after it that ask what the config holds, and the question it is asked for its
  version: `RUBOCOP_OPTS` is set empty, since
  RuboCop reads it as more options; `http_proxy`, `https_proxy`, `HTTP_PROXY` and `HTTPS_PROXY` are set to
  `http://127.0.0.1:9`, where nothing listens, and `no_proxy` and `NO_PROXY` are set empty;
- actionlint: `SHELLCHECK_OPTS` is removed, for the ShellCheck it may start;
- sqlfluff: `HOME` is set to the top of the repository, so what sqlfluff reads from `~` it reads from the
  repository: the configs there, and in the folders between `~` and each file, are the repository's own.
  `XDG_CONFIG_HOME`, `PYTHONPATH` and `PYTHONUSERBASE` are removed, so no user config and no Python package is
  taken from where they point;
- hadolint: every variable whose name starts with `HADOLINT_` is removed, whatever follows, so neither one that
  hadolint 2.15.1 reads (`HADOLINT_IGNORE` and `HADOLINT_DISABLE_IGNORE_PRAGMA` among them) nor one a later
  hadolint adds reaches it;
- Vale: the run that reads the repository's config (`vale ls-config`) has `PWD` removed, so Vale takes its folder
  from the system. The check run's environment is not changed.

ESLint, mypy, PHPStan, dart, yamllint, stylelint and markdownlint-cli2 run with your environment as it is, and so
do the engines not named above; no `NODE_PATH` is set for the three that run on Node. The question each engine is
asked for its version is put with your environment as it is, too, RuboCop's apart (above): none of the other
changes is made for it.

**A clippy rule builds the code.** `cargo clippy` compiles the crate. A build runs that crate's build scripts and
procedural macros, and cargo may download its dependencies. That is code from the repository running as you.

**A Periphery rule builds the package.** Before Periphery scans, `swift` reads the package's `Package.swift`, and
the `Package.swift` of each local package it depends on; each is Swift code that runs as you. `swift build` then
compiles the package and its tests, running any build plugins it declares. The package is looked for no higher
than the top of the git repository, and a local package outside the repository, or one that does not resolve,
makes the rule not available before anything is built, so Tao asks swift to read no manifest outside the
repository. The build goes to Tao's temporary folder, and with `--disable-automatic-resolution` it fetches
no dependency; a source-control or registry dependency, or a binary target with a `url`, in any of those manifests
makes the rule not available before anything is built.

**A golangci-lint rule loads the code with go.** go runs with `GOTOOLCHAIN=local`, `GOPROXY=off` and `GOSUMDB=off`,
so it downloads no toolchain and no module and looks up no checksum; code that needs a module not already on the
machine fails its rule.

**A RuboCop config is not fetched from the network.** The RuboCop runs that read the repository's config have
their proxy variables pointed at an address where nothing listens (above), so a config whose `inherit_from` names
a URL fails its rule instead of being fetched. RuboCop's version is asked with `--force-default-config`, which
reads no config file. Other single doors are shut in the same way: a PMD `check` that is a ruleset file or a URL
is refused, mypy is never passed `--install-types`, dart runs with `--suppress-analytics`, and Tao never
runs `dart pub get` or `vale sync`. A Dart package whose dependencies were never fetched is reported not
available where dart needs them, and a Vale style must already be on the repository's `StylesPath`, or be Vale's
built-in one. None of this is a sandbox: an engine that reaches the network some other way is not stopped.

**An engine's config can name code, and the engine runs it.** ESLint's config is a JavaScript module that ESLint
loads. A stylelint config can name plugins, a PHPStan config bootstrap files, a sqlfluff config a templater, and
each engine loads what its config names, as you. Tao passes no flag that stops this and does not read a
config for what it would load. It sets no `NODE_PATH`, so the plugins an ESLint or stylelint config names are
resolved as they are when you run that engine yourself. Two cases are refused: a markdownlint rule fails before
its check runs when a `.markdownlint.cjs` or `.mjs` config applies to one of its files, or when a
markdownlint-cli2 options file is where markdownlint-cli2 would read it; and for sqlfluff, `PYTHONPATH` and
`PYTHONUSERBASE` are removed, as above.

**A cargo-audit rule is as current as its advisory database.** cargo-audit runs with `--no-fetch` and
`--no-yanked`, so it reaches no network: it reads the RustSec advisory database already at
`$CARGO_HOME/advisory-db` (else `$HOME/.cargo/advisory-db`), and Tao never fetches or updates it. An advisory
published after that copy was fetched is not reported, and the report does not say how old the copy is. Fetch it
yourself (`cargo audit update-db`) as often as you need. With no copy there the rule is not available, never clean.

**A gitleaks rule never prints the secret.** gitleaks runs with `--redact`, every gitleaks finding's message is a
fixed text of Tao's, never gitleaks' own, and of gitleaks' stderr only its error lines are kept, so a matched
secret does not reach the text or the JSON report. The finding's path, line and column are still given, and the
secret is still in the file.

**The engines read their own configuration.** Each engine looks for its config in the code it checks, as it always
does. Except for the exceptions below, Tao passes no flag that turns that off, so a repository's own linter config shapes what its linter
reports. The exceptions:

- for a clippy rule with `max_nesting`, `CLIPPY_CONF_DIR` points that run at Tao's temporary `clippy.toml`,
  which clippy reads in place of the repository's own;
- a SwiftLint run given `--config` (the rule's own `config`, or Tao's threshold file) reads those files in
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
- a Periphery run passes `--config` naming an empty file Tao writes, so the package's `.periphery.yml` is not
  read;
- an ESLint run passes `--no-inline-config` and `--no-ignore`, so an `eslint-disable` comment and the config's
  `ignores` silence nothing; a finding that an `eslint-suppressions.json` hides is read from ESLint's output and
  reported like any other;
- a mypy run always names its config with `--config-file=`: the rule's `config`, else the one Tao finds
  from the rule file's folder up to the top of the repository (`mypy.ini`, `.mypy.ini`, a `pyproject.toml` with a
  `[tool.mypy]` table, a `setup.cfg` with a `[mypy]` section), else none, so mypy makes no search of its own and
  reads no config from your home folder. The rule fails before mypy runs when that config sets `ignore_errors`
  true in any section, lists the check under `disable_error_code` in a per-module section, or sets
  `ignore_missing_imports` true when the check is `import-not-found` or `import-untyped`;
- a PMD run reads no ruleset file: a rule's `config` does not reach PMD, and `--suppress-marker` renames PMD's
  `// NOPMD` marker to a text no code carries, so that comment silences nothing;
- a detekt run is given, after the rule's `config` or the folder's `detekt.yml`, a config layer Tao writes,
  which wins key by key: it turns the rule's checks and their rule sets on and empties their `excludes` and
  `includes`, and each check's `ignoreAnnotated`. No `--baseline` is passed;
- a RuboCop run passes `--ignore-disable-comments`, so `# rubocop:disable` and `# rubocop:todo` comments silence
  nothing. The rule fails when a `.rubocop_todo.yml` the config reads names a cop of the check, when the config
  gives such a cop an `Exclude` entry RuboCop's stock config does not have, or when its `AllCops` `Exclude` or
  `Include` leaves out a file the rule covers that the stock config inspects;
- a PHPStan run is pointed at a wrapper config Tao writes, which includes the repository's own (the rule's
  `config`, else the first PHPStan config file in the rule file's folder) and empties its `ignoreErrors`, a
  baseline's entries with them, and its `excludePaths`;
- a dart rule fails when an `analysis_options.yaml` that governs its folder, or a file one of them includes, holds
  an `exclude` key or an `ignore` value, or is read from outside the repository (a `package:` include apart),
  since dart cannot be told to pass over any of these;
- an actionlint run always names its config with `-config-file=`: the rule's `config`, else
  `.github/actionlint.yaml` or `.github/actionlint.yml` at the top of the repository, else `/dev/null`. The rule
  fails when that config holds an `ignore`, which actionlint would apply whatever it is told;
- a sqlfluff run passes `--disable-noqa` and `--disregard-sqlfluffignores`, and empty `--exclude-rules=` and
  `--ignore=`, so `-- noqa` comments, `.sqlfluffignore` files and a config's `exclude_rules` and `ignore` silence
  nothing;
- a hadolint run always names its config with `--config=`: the rule's `config`, else the rule file's folder's
  `.hadolint.yaml` or `.hadolint.yml`, else an empty file Tao writes, so no hadolint config elsewhere on
  your machine is read. It passes `--disable-ignore-pragma`, so `# hadolint ignore=` comments silence nothing, and
  the rule fails before hadolint runs when the config lists anything under `ignored`;
- a yamllint run is given its whole config as text with `-d`, in place of yamllint's own search. The text extends
  the rule's `config`, else the nearest `.yamllint`, `.yamllint.yaml` or `.yamllint.yml` from the rule file's
  folder up to the top of the repository, else yamllint's `default`; it turns the check on and empties the
  config's `ignore` and `ignore-from-file`, and the check's own;
- a Vale run passes `--no-global` and is pointed at a wrapper config Tao writes, which takes only the
  `StylesPath` and the `Vocab` from the repository's config, so that config's skip keys, its sections and their
  `Style.Rule = NO` lines do not apply. A config Vale would read from outside the repository makes the rule not
  available;
- a stylelint run passes `--ignore-disables`, `--ignore-path=/dev/null` and `--disable-default-ignores` and is
  pointed at an empty suppressions file Tao writes, so `stylelint-disable` comments, a `.stylelintignore`
  and a `stylelint-suppressions.json` silence nothing. The rule fails when the config turns the check off for
  some of its files, turns on `quiet` or `fix`, or leaves one of its files out through `ignoreFiles`;
- a markdownlint-cli2 run is given an options module Tao writes, which turns inline config off, so
  `<!-- markdownlint-disable -->` comments and their other forms silence nothing. The rule fails when the
  repository has an options file of its own (`.markdownlint-cli2.jsonc`, `.yaml`, `.cjs` or `.mjs`) in the rule
  file's folder or between it and one of the rule's files, or a config that applies to those files and that
  Tao cannot read (a `.jsonc`, `.cjs` or `.mjs` one, or one with `extends`). A config that turns the rule
  off makes it not available.

For some of these engines Tao reads the repository's config itself, as data and never as code, to find a
setting that would hide findings: the mypy, actionlint and hadolint config it names, dart's
`analysis_options.yaml` files and what they include, markdownlint's `.markdownlint.json`, `.yaml` and `.yml`, and
for RuboCop the `.rubocop` file and each `.rubocop_todo.yml` the config reads. For RuboCop, Vale and stylelint it
asks the engine what the config says (`rubocop --show-cops`, `vale ls-config`, `stylelint --print-config`).

**Some suppressions written in the code cannot be turned off.** vulture honours a `# noqa` comment on an item's
line, and reads a whitelist module the repository keeps as ordinary code that uses names. Periphery honours
`// periphery:ignore` (with `:all` or `:parameters`) and `// periphery:override`. Neither engine has a switch for
these, so a finding they silence is not reported and not counted. A repository that takes `standard@1` gets each
`# noqa` reported by `tm-python-suppression`; no released set reports the Periphery comments.

The same holds for these, which their engines still honour and Tao cannot turn off:

- mypy: `# type: ignore` comments, the inline `# mypy: ignore-errors` and `# mypy: disable-error-code=` comments,
  and `@typing.no_type_check`;
- PMD: `@SuppressWarnings` naming `"PMD"`, `"PMD.<Rule>"`, `"all"` or `"unused"`;
- detekt: `@Suppress`;
- PHPStan: `@phpstan-ignore`, `@phpstan-ignore-line` and `@phpstan-ignore-next-line` comments;
- dart: `// ignore: <code>` and `// ignore_for_file: <code>` comments, and their `type=lint` form;
- actionlint and hadolint: a `# shellcheck disable=` comment inside a workflow's `run:` script or a Dockerfile's
  `RUN` script;
- sqlfluff: `-- sqlfluff:` directives written in a file, such as `-- sqlfluff:exclude_rules:` and
  `-- sqlfluff:rules:`;
- yamllint: `# yamllint disable`, `disable-line` and `disable-file` comments;
- Vale: `<!-- vale off -->` and `<!-- vale Style.Rule = NO -->` comments.

`tm-python-suppression` reports each `# type: ignore`; no released set reports the others. The comments of
ESLint, RuboCop, stylelint and markdownlint are turned off, as above, and so are PMD's `// NOPMD`, sqlfluff's
`-- noqa` and hadolint's `# hadolint ignore=`.

**Some settings in a repository's config are still applied.** Each is a stated limit and does not fail the rule:

- RuboCop: a `.rubocop.yml` in a subfolder applies its own todo file and `Exclude` lists to the files under it,
  and only the config of the rule file's folder is examined. A cop's `Include` can narrow it to fewer files than
  RuboCop's stock one, and only `Exclude` lists are compared;
- PHPStan: a config's `stubFiles` that names a file handed over keeps that file from being analysed, and a service
  the config tags `phpstan.ignoreErrorExtension` can drop errors;
- Vale: the repository's `Vocab` still applies;
- yamllint: a config above the rule file's folder has its relative `extends` and `ignore-from-file` resolved from
  the rule file's folder, where yamllint runs, not from beside the config;
- sqlfluff, yamllint and stylelint: `cctao` picks the files by the engine's stock names, so a file a config adds
  through `sql_file_exts` or `yaml-files` is not checked, and stylelint is handed `.css` files only;
- ESLint: `--no-ignore` turns the config's `ignores` off, and ESLint's own defaults still leave out
  `node_modules`.

**There is no time limit on an engine.** Tao waits for an engine for as long as it runs. An engine that
hangs, hangs the check. The git questions asked through `ccvcslib` are bounded: git is stopped once it is stuck for
10 seconds or has run for 60, and the question fails. Where `cctao` needs the answer, the rule that asked fails or `cctao`
stops with an error, never clean. One answer it can do without: the list of the files in the rule file's folder,
which `cctao` uses to skip a rule that covers no file and to drop ESLint findings in files git ignores. When that list
fails, nothing is skipped or dropped: no rule is skipped for covering no file, and every ESLint finding is kept.
The one git question that `ccvcslib` waits on for as long as it runs is the list of untracked files.

**An exception is as trustworthy as the review of the rule file.** Anyone who can change `tao.toml` can add
an `[[exception]]`, remove a rule or lower its severity. `approved` is text that Tao records and does not
verify. Every report counts the exceptions in force so that a new one is visible, but only a review of the file
stops it.

**A security finding does not stop anything.** A finding of kind `security` is reported with its severity, its
place and its owner, and a run whose only errors are security findings ends with exit code 4, which is made to be
different from the code a caller refuses on. If your process must stop on a security finding, the caller has to do
that.

**Exit code 1 can hide a rule that did not run.** When a quality error is found, the exit code is 1 even if another
rule was not available or its engine failed. The counts in the output say so; the exit code alone does not.

**A rule can be written so that it matches nothing.** A `check` Tao cannot match back to findings fails its
rule and is never clean. A rule whose ast-grep pattern matches no code reports no findings and is not flagged. A
rule that covers none of the files git lists in the rule file's folder (the tracked files in the work tree and the
untracked files git does not ignore) is not run at all: its engine runs no check for it (it is still asked its
version), and whether the engine is installed does not change the result. One of the repository's own rules is
then reported as covering nothing, and makes the exit code 6 when nothing ranks above it; a baseline set's rule is
only counted as not applicable and does not change the exit code. A ShellCheck rule that covers no shell script is
the same, and so is a cargo-audit rule whose folder has no `Cargo.lock` it covers (only the rule file's folder's own
`Cargo.lock` is read), and so is a clang-tidy rule that covers files but no C-family source, whether or not
clang-tidy is installed. So is a mypy, PMD, detekt, PHPStan, dart, actionlint, sqlfluff, hadolint, yamllint, Vale,
stylelint or markdownlint rule that covers files but none of the kind its engine reads. ESLint and RuboCop
differ: their own config decides which files they read, so a rule that covers files, none of which the engine
read, is settled after the engine has run, and with the engine missing it is not available. A check an engine
would run nothing for is not available either, never clean: a detekt, hadolint or markdownlint name that is not on
the list built into `cctao`, a detekt rule that needs a classpath, a dart lint that an `analysis_options.yaml`
governing the folder does not enable, a stylelint rule its config does not turn on, a markdownlint rule its config
turns off. Try a new rule on a file you know breaks it.

**Engine messages are printed as given.** In the text output a finding's message is the engine's own text
(gitleaks' apart, as above). The JSON output escapes it.

## Running it on code you do not trust

- Read `tao.toml` first, and the files it names: each ast-grep rule file and each `config`.
- Read the engines' own config files in the repository too. An engine loads the code its config names: an ESLint
  config and its plugins, a stylelint plugin, a PHPStan bootstrap file, a sqlfluff templater.
- Look for an `eslint` in the rule file's folder's `node_modules/.bin` (on Windows, `eslint.cmd` and the other
  `PATHEXT` names too). When it is there and executable (on Windows any file counts), an ESLint rule runs it, not
  the `eslint` on your `PATH`: it is the repository's code, as its ESLint config is. Remove the folder's
  `node_modules`, or leave ESLint rules out with `--rules <path>`, if you would not run that code.
- Expect a clippy or Periphery rule to build the code. If you would not build the repository, do not check it with
  either.
- Use a rule file of your own with `--rules <path>` when you want your rules and not the repository's.
- Run it where a build of that code would be safe: a container, a virtual machine or a throwaway account.

## What install touches

`cctao install` needs no root. It copies the binary to `~/.local/bin/cctao`, writes a skill file for each AI
coding harness you choose, and records what it placed in `~/.local/state/cctao/installed`. It makes no link, makes
the folders those paths need and writes nowhere else.

- It replaces only the files its own record names. Anything else already at the binary's path or at a skill path
  is kept. Install says so.
- Each file is written whole to a new temporary file beside it and then renamed into place, so a symlink at the
  path is not written through.
- A symlink, or anything else that is not a plain file, at the record's path refuses the install or the uninstall
  before anything is placed or removed.
- `cctao uninstall` removes what the record names and nothing else: the files, and the folders install made only
  once they are empty.

## Reporting a vulnerability

**Do not open a public issue for a security problem.**

Use GitHub's [Report a vulnerability](https://github.com/codedaptive/temperare/security/advisories/new) form under
this repository's Security tab, or email <security@codedaptive.com>.

A useful report gives the output of `cctao --version`, the versions of the engines involved, the smallest rule file
and input that show the problem, and what you saw.

## What counts

Anything that breaks a property under "Security by construction" or "What install touches", for example:

- a way to make Tao run a program, or pass an argument, that the rule file and `PATH` did not name;
- a way to make it write or remove a file other than the ones named above;
- a rule that has a file to check reported as clean when its engine did not run, or a silenced finding that is not
  counted;
- a `--base` value read as an option by git, or a path on the command line reaching git or an engine;
- a rule's `check`, `config` or `allow` value read by an engine as an option;
- an ast-grep rule file read from outside the rule file's folder;
- an install that replaces a file its record does not name, or an uninstall that removes something its record
  does not name.

The limits under "What this does not defend against" are how the tool is meant to work. A report that a hostile
rule file can run a hostile build is correct, and is not a vulnerability in Tao.

## What to expect

Tao is maintained on a best-effort basis. We read security reports, work confirmed problems privately, and
disclose once a fix is out. We are glad to credit reporters. There is no bug bounty and no response-time guarantee.

## Supported versions

| Version | Supported |
|---|---|
| The `main` branch (0.1.x) | Yes. Fixes land here |

There are no release branches yet.

//! `GOLANGCI_LINT`: the golangci-lint engine as an entry.

use super::{golangci_lint_refusal, parse_golangci_lint, run_golangci_lint, unknown_linters};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `golangci-lint run --output.json.path=stdout <pinned flags> --enable-only=<check> [--config=<config>] ./...` run
/// in the rule file's folder, read by `parse_golangci_lint`. Each pinned flag holds against a repo's own config:
/// - `--show-stats=false`: else a stats line ("N issues:") follows the JSON on stdout;
/// - `--output.text.path=stderr`: a repo config's text output to stdout would break the JSON;
/// - `--max-issues-per-linter=0`, `--max-same-issues=0`, `--uniq-by-line=false`: else issues are capped and folded;
/// - `--allow-parallel-runners`: else a second golangci-lint fails on its start-up lock;
/// - `--path-mode=abs`, `--path-prefix=`: absolute paths, which tm can make relative to the folder;
/// - `--fix=false`: a repo config's `issues.fix` would edit the checked code;
/// - `--issues-exit-code=1`: the reader reads exit 0 and 1 as answers, any other exit as none.
///
/// A check is golangci-lint linter names (`staticcheck`, or a comma list `staticcheck,govet`) passed to
/// `--enable-only`, not a linter's own checks such as staticcheck's `SA1019`: those can be picked only in a config
/// file, golangci-lint leaves some off by default, and filtering a linter's text by code would read a disabled or
/// unknown code as clean with no word from the engine. `--enable-only` is the engine's own selector, it names an
/// unknown linter itself (`unknown_linters`), and findings match back by their linter. A check tm cannot match back
/// is refused before running (`golangci_lint_refusal`). It takes no params.
///
/// `run_golangci_lint` runs the check with GOTOOLCHAIN=local, GOPROXY=off and GOSUMDB=off (no toolchain or module
/// download, no checksum lookup), GOFLAGS removed (nothing written to go.mod or go.sum) and PWD set to the folder
/// (golangci-lint makes its paths from $PWD, so they start with the folder tm knows). It needs `go` too, since
/// golangci-lint loads packages with it.
pub static GOLANGCI_LINT: Entry = Entry {
    name: "golangci-lint",
    engine: Engine::GolangciLint,
    program: "golangci-lint",
    needs: &["golangci-lint", "go"],
    never: None,
    args: &[
        "run",
        "--output.json.path=stdout",
        "--output.text.path=stderr",
        "--show-stats=false",
        "--issues-exit-code=1",
        "--max-issues-per-linter=0",
        "--max-same-issues=0",
        "--uniq-by-line=false",
        "--allow-parallel-runners",
        "--path-mode=abs",
        "--path-prefix=",
        "--fix=false",
    ],
    pick: Pick::Arg("--enable-only="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &["./..."],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: golangci_lint_refusal,
    unknown: unknown_linters,
    read: parse_golangci_lint,
    run: run_golangci_lint,
};

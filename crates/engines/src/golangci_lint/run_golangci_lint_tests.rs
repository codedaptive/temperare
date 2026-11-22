//! ZUT for run_golangci_lint.rs: `run_golangci_lint(&GOLANGCI_LINT, ..)` on a fake host: the environment the command
//! reaches the host with, and the outcome read from captured golangci-lint output (fixtures/*, described in
//! fixtures/how_made.txt). Nothing runs and nothing is written. `DIR` is the scratch folder the captured fixture's
//! absolute paths start with; no folder is read, so it need not exist.

use super::{run_golangci_lint, GOLANGCI_LINT};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::{RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::Path;

const DIR: &str = "/private/tmp/gcl-fix/issues";

fn hit(line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new("probe.go"), line: Some(line), col: Some(col), what: what.to_string() }
}

fn os_pair(name: &str, value: &str) -> (OsString, OsString) {
    (OsString::from(name), OsString::from(value))
}

#[test]
fn run_golangci_lint_keeps_go_local_and_reads_the_engines_answer() {
    let rows: [(&str, &str, Output, Result<Vec<Hit>, Stop>); 3] = [
        (
            "findings",
            "staticcheck,govet",
            Output { code: Some(1), stdout: include_bytes!("fixtures/staticcheck_and_govet.json").to_vec(), stderr: Vec::new() },
            Ok(vec![
                hit(16, 16, "copylocks: Locked passes lock by value: sync.Mutex"),
                hit(9, 9, "SA1019: strings.Title has been deprecated since Go 1.18 and an alternative has been available since Go 1.0: The rule Title uses for word boundaries does not handle Unicode punctuation properly. Use golang.org/x/text/cases instead."),
                hit(13, 9, "SA4000: identical expressions on the left and right side of the '==' operator"),
            ]),
        ),
        (
            "unknown linter",
            "nosuchlinter",
            Output { code: Some(3), stdout: Vec::new(), stderr: include_bytes!("fixtures/unknown_linters.stderr").to_vec() },
            Err(Stop::UnknownCheck {
                said: "Error: unknown linters: 'nosuchlinter', run 'golangci-lint help linters' to see the list of supported linters".to_string(),
            }),
        ),
        (
            "no module",
            "staticcheck",
            Output {
                code: Some(7),
                stdout: include_bytes!("fixtures/no_module.json").to_vec(),
                stderr: include_bytes!("fixtures/no_module.stderr").to_vec(),
            },
            Err(Stop::Failed {
                code: Some(7),
                stderr: "level=error msg=\"[linters_context] typechecking error: pattern ./...: directory prefix . does not contain main module or its selected dependencies\"\n".to_string(),
            }),
        ),
    ];
    for (case, check, answer, want) in rows {
        let text = format!("version = 1\n[[rule]]\nid = \"go-static\"\nengine = \"golangci-lint\"\ncheck = \"{check}\"\nfix = \"f\"\n");
        let file = RuleFile::parse(&text, Path::new(DIR)).unwrap();
        let seen = RefCell::new(None);
        let run = |cmd: &Cmd| {
            *seen.borrow_mut() = Some(cmd.clone());
            Ok(answer.clone())
        };
        let on_path = |_: &str| -> bool { panic!("golangci-lint's run looks nothing up on PATH") };
        let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
        let got = run_golangci_lint(&GOLANGCI_LINT, &file.rules[0], Path::new(DIR), &host);
        assert_eq!(got, want, "{case}");
        let cmd = seen.into_inner().unwrap();
        let env = vec![
            os_pair("GOTOOLCHAIN", "local"),
            os_pair("GOPROXY", "off"),
            os_pair("GOSUMDB", "off"),
            os_pair("PWD", "/private/tmp/gcl-fix/issues"),
        ];
        assert_eq!(cmd.env, env, "{case}");
        assert_eq!(cmd.env_remove, vec![OsString::from("GOFLAGS")], "{case}");
    }
}

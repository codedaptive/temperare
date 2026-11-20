//! ZUT for rule_host.rs: a rule's host keeps the check's `on_path` and `base` and uses the `run` and `raw_chars` it
//! is given.

use crate::host::Host;
use crate::rule_host::rule_host;
use procio::{Cmd, Output, ProcError};
use std::cell::{Cell, RefCell};

#[test]
fn rule_host_copies_on_path_and_base_and_takes_run_and_raw_chars() {
    for base in [Some("abc123"), None] {
        let check_run = |_: &Cmd| -> Result<Output, ProcError> { panic!("the rule's own run is used") };
        let on_path = |name: &str| name == "ruff";
        let check_chars = Cell::new(None);
        let host = Host { run: &check_run, on_path: &on_path, raw_chars: &check_chars, base };

        let seen = RefCell::new(Vec::new());
        let run = |cmd: &Cmd| -> Result<Output, ProcError> {
            seen.borrow_mut().push(cmd.program.clone());
            Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() })
        };
        let raw_chars = Cell::new(None);
        let got = rule_host(&host, &run, &raw_chars);

        assert_eq!(got.base, base, "base {base:?}");
        assert!((got.on_path)("ruff"), "base {base:?}");
        assert!(!(got.on_path)("swiftlint"), "base {base:?}");
        (got.run)(&Cmd::new("ruff")).unwrap();
        assert_eq!(*seen.borrow(), vec!["ruff"], "base {base:?}");
        got.raw_chars.set(Some(5));
        assert_eq!(raw_chars.get(), Some(5), "base {base:?}");
        assert_eq!(check_chars.get(), None, "base {base:?}");
    }
}

use super::local_dependencies;
use procio::Output;
use std::path::PathBuf;

fn check_local_dependencies(stdout: &[u8], expected: Option<Vec<&str>>) {
    let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: vec![] };
    match expected {
        Some(expected) => assert_eq!(local_dependencies(&out).unwrap(), expected.into_iter().map(PathBuf::from).collect::<Vec<_>>()),
        None => assert!(local_dependencies(&out).is_err()),
    }
}

macro_rules! cases {
    ($( $name:ident => ($stdout:expr, $expected:expr) ),+ $(,)?) => {
        $(#[test] fn $name() { check_local_dependencies($stdout, $expected); })+
    };
}

cases! {
    remote_and_local => (include_bytes!("fixtures/dump_package_remote.json"), Some(vec!["/private/var/folders/zc/zl4w__wx2xdfmd5lps9dp2000000gn/T/local-dep"])),
    two_local_in_order => (include_bytes!("fixtures/dump_package_two_local.json"), Some(vec!["/Users/bob/devlop/team-temperare/workcrews/engines/scratchpad/periphery-local-fixture-jd0gvfna/package/First", "/Users/bob/devlop/team-temperare/workcrews/engines/scratchpad/periphery-local-fixture-jd0gvfna/package/Second"])),
    no_local_dependencies => (include_bytes!("fixtures/dump_package_clean.json"), Some(vec![])),
    not_json => (b"not JSON", None),
}

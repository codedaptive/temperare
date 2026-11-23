//! ZUT for hadolint_cmd.rs: the argument list built for `HADOLINT` and the variables removed from its environment.
//! Nothing runs.

use super::{hadolint_cmd, HADOLINT};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn hadolint_cmd_args_and_env() {
    let files = [PathBuf::from("Dockerfile"), PathBuf::from("-x.Dockerfile"), PathBuf::from("sub dir/Containerfile")];
    let cmd = hadolint_cmd(&HADOLINT, "DL3006", Path::new("/w/.hadolint.yaml"), &files, Path::new("/w"));
    let args = [
        "--no-color",
        "-f",
        "sarif",
        "--disable-ignore-pragma",
        "--error=DL3006",
        "--config=/w/.hadolint.yaml",
        "--",
        "Dockerfile",
        "-x.Dockerfile",
        "sub dir/Containerfile",
    ];
    assert_eq!(cmd.program, OsString::from("hadolint"));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w")));
    assert_eq!(cmd.args, args.map(OsString::from));
    let removed = [
        "HADOLINT_DISABLE_IGNORE_PRAGMA",
        "HADOLINT_FAILURE_THRESHOLD",
        "HADOLINT_FORMAT",
        "HADOLINT_IGNORE",
        "HADOLINT_NOFAIL",
        "HADOLINT_OVERRIDE_ERROR",
        "HADOLINT_OVERRIDE_INFO",
        "HADOLINT_OVERRIDE_STYLE",
        "HADOLINT_OVERRIDE_WARNING",
        "HADOLINT_REQUIRE_LABELS",
        "HADOLINT_STRICT_LABELS",
        "HADOLINT_TRUSTED_REGISTRIES",
        "HADOLINT_VERBOSE",
    ];
    let mut got = cmd.env_remove.clone();
    got.sort();
    assert_eq!(got, removed.map(OsString::from));
}

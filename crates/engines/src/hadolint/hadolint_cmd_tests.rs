//! Unit test for hadolint_cmd.rs: the argument list built for `HADOLINT` and the variables it is given to
//! remove from its environment. Nothing runs.

use super::{hadolint_cmd, HADOLINT};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn hadolint_cmd_args_and_env() {
    let files = [PathBuf::from("Dockerfile"), PathBuf::from("-x.Dockerfile"), PathBuf::from("sub dir/Containerfile")];
    let removed = ["HADOLINT_IGNORE", "HADOLINT_FOO", "HADOLINT_NOFAIL"].map(OsString::from);
    let cmd = hadolint_cmd(&HADOLINT, "DL3006", Path::new("/w/.hadolint.yaml"), &files, Path::new("/w"), &removed);
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
    assert_eq!(cmd.env_remove, ["HADOLINT_IGNORE", "HADOLINT_FOO", "HADOLINT_NOFAIL"].map(OsString::from));
}

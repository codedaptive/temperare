//! Unit tests for install_host_real.rs: `host_from`, the decision `InstallHost::real` makes from the values it reads (no
//! environment is read here): each value in its field, and each cause's own `what` and `fix`.

use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};

use super::install_host_real::host_from;
use super::InstallHost;
use crate::UsageError;

fn exe() -> io::Result<PathBuf> {
    Ok(PathBuf::from("/opt/tools/cctao"))
}

fn gone() -> io::Result<PathBuf> {
    Err(io::Error::new(io::ErrorKind::NotFound, "gone"))
}

fn home_not_set() -> UsageError {
    UsageError {
        what: "HOME is not set".to_string(),
        fix: "Set HOME to your home folder, then run the command again.".to_string(),
    }
}

#[test]
fn every_value_lands_in_its_field() {
    let host = host_from(
        Some(OsString::from("/home/ann")),
        Some(OsString::from("/usr/bin:/bin")),
        exe(),
        Path::new("/work/project"),
        true,
    );
    assert_eq!(
        host,
        Ok(InstallHost {
            home: PathBuf::from("/home/ann"),
            path_var: Some(OsString::from("/usr/bin:/bin")),
            exe: PathBuf::from("/opt/tools/cctao"),
            cwd: PathBuf::from("/work/project"),
            prefixes: vec![PathBuf::from("/opt/homebrew"), PathBuf::from("/usr/local")],
            interactive: true,
        })
    );
}

#[test]
fn home_unset_or_empty_says_to_set_home() {
    assert_eq!(host_from(None, None, exe(), Path::new("/w"), false), Err(home_not_set()));
    assert_eq!(host_from(Some(OsString::new()), None, exe(), Path::new("/w"), false), Err(home_not_set()));
}

#[test]
fn home_is_checked_before_the_binary() {
    assert_eq!(host_from(None, None, gone(), Path::new("/w"), false), Err(home_not_set()));
}

fn home_not_absolute() -> UsageError {
    UsageError {
        what: "HOME is not an absolute path: home/ann".to_string(),
        fix: "Set HOME to the absolute path of your home folder, then run the command again.".to_string(),
    }
}

#[test]
fn a_relative_home_says_to_set_an_absolute_home() {
    let host = host_from(Some(OsString::from("home/ann")), None, exe(), Path::new("/w"), false);
    assert_eq!(host, Err(home_not_absolute()));
}

#[test]
fn a_relative_home_is_checked_before_the_binary() {
    let host = host_from(Some(OsString::from("home/ann")), None, gone(), Path::new("/w"), false);
    assert_eq!(host, Err(home_not_absolute()));
}

#[test]
fn an_unset_path_is_none() {
    let host = host_from(Some(OsString::from("/home/ann")), None, exe(), Path::new("/w"), false).unwrap();
    assert_eq!(host.path_var, None);
    assert!(!host.interactive);
}

#[test]
fn an_empty_path_stays_an_empty_path() {
    let host = host_from(Some(OsString::from("/home/ann")), Some(OsString::new()), exe(), Path::new("/w"), false);
    assert_eq!(host.map(|host| host.path_var), Ok(Some(OsString::from(""))));
}

#[test]
fn a_missing_binary_says_to_run_cctao_from_its_file_not_to_set_home() {
    let host = host_from(Some(OsString::from("/home/ann")), None, gone(), Path::new("/w"), false);
    assert_eq!(
        host,
        Err(UsageError {
            what: "cannot find the running binary: gone".to_string(),
            fix: "Run cctao from its file (not through a deleted or moved path), then try again.".to_string(),
        })
    );
}

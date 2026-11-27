//! Unit test for plan_install.rs: placement, skill paths and the PATH note, from a made-up host.

use std::ffi::OsString;
use std::path::PathBuf;

use super::plan_install;
use crate::{Harness, InstallArgs, InstallHost, InstallPlan};

fn host() -> InstallHost {
    InstallHost {
        home: PathBuf::from("/home/ann"),
        path_var: Some(OsString::from("/usr/bin:/home/ann/.local/bin")),
        exe: PathBuf::from("/tmp/dl/cctao"),
        cwd: PathBuf::from("/work/project"),
        prefixes: vec![PathBuf::from("/opt/homebrew"), PathBuf::from("/usr/local")],
        interactive: false,
    }
}

fn p(path: &str) -> PathBuf {
    PathBuf::from(path)
}

#[test]
fn a_plain_install_places_the_binary() {
    assert_eq!(
        plan_install(&InstallArgs::default(), &host(), &[]),
        InstallPlan { place: Some(p("/home/ann/.local/bin/cctao")), skills: vec![], notes: vec![] }
    );
}

#[test]
fn under_a_package_prefix_nothing_is_copied() {
    let host = InstallHost { exe: p("/opt/homebrew/bin/cctao"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.place, None);
    assert_eq!(
        plan.notes,
        vec!["cctao not copied: /opt/homebrew/bin/cctao is under /opt/homebrew, which a package manager owns"]
    );
}

#[test]
fn a_prefix_is_matched_by_whole_folders_only() {
    let host = InstallHost { exe: p("/usr/localtools/cctao"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.place, Some(p("/home/ann/.local/bin/cctao")));
}

#[test]
fn no_place_copies_nothing() {
    let args = InstallArgs { no_place: true, ..Default::default() };
    let plan = plan_install(&args, &host(), &[]);
    assert_eq!(plan.place, None);
    assert_eq!(plan.notes, vec!["--no-place: cctao not copied"]);
}

#[test]
fn running_the_placed_binary_copies_nothing() {
    let host = InstallHost { exe: p("/home/ann/.local/bin/cctao"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.place, None);
    assert!(plan.notes.is_empty(), "{:?}", plan.notes);
}

#[test]
fn skills_go_under_home_or_under_the_project_with_local() {
    let targets = [Harness::Claude, Harness::Cursor];
    let home = plan_install(&InstallArgs::default(), &host(), &targets);
    assert_eq!(
        home.skills,
        vec![p("/home/ann/.claude/skills/cctao/SKILL.md"), p("/home/ann/.cursor/skills/cctao/SKILL.md")]
    );
    let args = InstallArgs { local: true, ..Default::default() };
    let local = plan_install(&args, &host(), &[Harness::Claude]);
    assert_eq!(local.skills, vec![p("/work/project/.claude/skills/cctao/SKILL.md")]);
}

#[test]
fn a_bin_folder_off_path_is_noted() {
    let host = InstallHost { path_var: Some(OsString::from("/usr/bin:/bin")), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.notes, vec!["/home/ann/.local/bin is not on PATH; add it to run cctao by name"]);
}

#[test]
fn an_unset_path_notes_the_bin_folder_is_not_on_path() {
    let host = InstallHost { path_var: None, ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.notes, vec!["/home/ann/.local/bin is not on PATH; add it to run cctao by name"]);
}

#[test]
fn nothing_in_the_bin_folder_needs_no_path_note() {
    let host = InstallHost { exe: p("/usr/local/bin/cctao"), path_var: Some(OsString::new()), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[]);
    assert_eq!(plan.place, None);
    assert_eq!(
        plan.notes,
        vec!["cctao not copied: /usr/local/bin/cctao is under /usr/local, which a package manager owns"]
    );
}

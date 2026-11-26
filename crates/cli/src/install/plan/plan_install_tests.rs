//! ZUT for plan_install.rs: placement, the tm link and its clash, skill paths and the PATH note, from a made-up host.

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::plan_install;
use crate::{Harness, InstallArgs, InstallHost, InstallPlan};

/// What an absolute path starts with before its first `/`: nothing on unix; off unix a path needs a drive to be
/// absolute, and `C:/home/ann` is.
#[cfg(unix)]
const DRIVE: &str = "";
#[cfg(not(unix))]
const DRIVE: &str = "C:";

/// HOME: `/home/ann` on unix.
fn home() -> PathBuf {
    PathBuf::from(format!("{DRIVE}/home/ann"))
}

/// HOME's `.local/bin`: `/home/ann/.local/bin` on unix.
fn bin() -> PathBuf {
    home().join(".local").join("bin")
}

/// A PATH value of `dirs`, in this platform's separator (`:` on unix).
fn path_of(dirs: &[PathBuf]) -> Option<OsString> {
    Some(env::join_paths(dirs).unwrap())
}

/// The note that `bin()` is not on PATH.
fn off_path_note() -> String {
    format!("{} is not on PATH; add it to run temperare and tm by name", bin().display())
}

fn host() -> InstallHost {
    InstallHost {
        home: home(),
        path_var: path_of(&[p("/usr/bin"), bin()]),
        exe: PathBuf::from("/tmp/dl/temperare"),
        cwd: PathBuf::from("/work/project"),
        prefixes: vec![PathBuf::from("/opt/homebrew"), PathBuf::from("/usr/local")],
        interactive: false,
    }
}

fn p(path: &str) -> PathBuf {
    PathBuf::from(path)
}

fn link_to(target: PathBuf) -> Option<(PathBuf, PathBuf)> {
    Some((bin().join("tm"), target))
}

#[test]
fn a_plain_install_places_the_binary_and_links_tm_to_it() {
    assert_eq!(
        plan_install(&InstallArgs::default(), &host(), &[], None),
        InstallPlan {
            place: Some(bin().join("temperare")),
            link: link_to(bin().join("temperare")),
            skills: vec![],
            notes: vec![],
        }
    );
}

#[test]
fn under_a_package_prefix_nothing_is_copied_and_tm_points_at_the_running_binary() {
    let host = InstallHost { exe: p("/opt/homebrew/bin/temperare"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.place, None);
    assert_eq!(plan.link, link_to(p("/opt/homebrew/bin/temperare")));
    assert_eq!(
        plan.notes,
        vec!["temperare not copied: /opt/homebrew/bin/temperare is under /opt/homebrew, which a package manager owns"]
    );
}

#[test]
fn a_prefix_is_matched_by_whole_folders_only() {
    let host = InstallHost { exe: p("/usr/localtools/temperare"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.place, Some(bin().join("temperare")));
}

#[test]
fn no_place_copies_nothing_and_links_to_the_running_binary() {
    let args = InstallArgs { no_place: true, ..Default::default() };
    let plan = plan_install(&args, &host(), &[], None);
    assert_eq!(plan.place, None);
    assert_eq!(plan.link, link_to(p("/tmp/dl/temperare")));
    assert_eq!(plan.notes, vec!["--no-place: temperare not copied; tm points at /tmp/dl/temperare"]);
}

#[test]
fn running_the_placed_binary_copies_nothing_and_still_links_to_it() {
    let host = InstallHost { exe: bin().join("temperare"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.place, None);
    assert_eq!(plan.link, link_to(bin().join("temperare")));
    assert!(plan.notes.is_empty(), "{:?}", plan.notes);
}

#[test]
fn another_tm_on_path_skips_the_link_and_names_the_owner_and_the_override() {
    let plan = plan_install(&InstallArgs::default(), &host(), &[], Some(Path::new("/usr/bin/tm")));
    assert_eq!(plan.place, Some(bin().join("temperare")));
    assert_eq!(plan.link, None);
    assert_eq!(
        plan.notes,
        vec!["tm not linked: /usr/bin/tm is already tm on PATH; run temperare install --short-name to link it anyway"]
    );
}

#[test]
fn short_name_links_despite_the_clash_and_says_so() {
    let args = InstallArgs { short_name: true, ..Default::default() };
    let plan = plan_install(&args, &host(), &[], Some(Path::new("/usr/bin/tm")));
    assert_eq!(plan.link, link_to(bin().join("temperare")));
    assert_eq!(plan.notes, vec!["tm linked with --short-name, though /usr/bin/tm is also tm on PATH"]);
}

#[test]
fn skills_go_under_home_or_under_the_project_with_local() {
    let targets = [Harness::Claude, Harness::Cursor];
    let global = plan_install(&InstallArgs::default(), &host(), &targets, None);
    assert_eq!(
        global.skills,
        vec![
            home().join(".claude").join("skills").join("temperare").join("SKILL.md"),
            home().join(".cursor").join("skills").join("temperare").join("SKILL.md"),
        ]
    );
    let args = InstallArgs { local: true, ..Default::default() };
    let local = plan_install(&args, &host(), &[Harness::Claude], None);
    assert_eq!(local.skills, vec![p("/work/project/.claude/skills/temperare/SKILL.md")]);
}

#[test]
fn a_bin_folder_off_path_is_noted() {
    let host = InstallHost { path_var: path_of(&[p("/usr/bin"), p("/bin")]), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.notes, vec![off_path_note()]);
}

#[test]
fn an_unset_path_notes_the_bin_folder_is_not_on_path() {
    let host = InstallHost { path_var: None, ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.notes, vec![off_path_note()]);
}

#[test]
fn nothing_in_the_bin_folder_needs_no_path_note() {
    let host = InstallHost { exe: p("/usr/local/bin/temperare"), path_var: Some(OsString::new()), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], Some(Path::new("/usr/local/bin/tm")));
    assert_eq!((plan.place, plan.link), (None, None));
    assert_eq!(plan.notes.len(), 2, "{:?}", plan.notes);
    assert!(!plan.notes.iter().any(|note| note.contains("not on PATH")), "{:?}", plan.notes);
}

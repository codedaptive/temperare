//! ZUT for plan_install.rs: placement, the tm link and its clash, skill paths and the PATH note, from a made-up host.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::plan_install;
use crate::{Harness, InstallArgs, InstallHost, InstallPlan};

fn host() -> InstallHost {
    InstallHost {
        home: PathBuf::from("/home/ann"),
        path_var: Some(OsString::from("/usr/bin:/home/ann/.local/bin")),
        exe: PathBuf::from("/tmp/dl/temperare"),
        cwd: PathBuf::from("/work/project"),
        prefixes: vec![PathBuf::from("/opt/homebrew"), PathBuf::from("/usr/local")],
        interactive: false,
    }
}

fn p(path: &str) -> PathBuf {
    PathBuf::from(path)
}

fn link_to(target: &str) -> Option<(PathBuf, PathBuf)> {
    Some((p("/home/ann/.local/bin/tm"), p(target)))
}

#[test]
fn a_plain_install_places_the_binary_and_links_tm_to_it() {
    assert_eq!(
        plan_install(&InstallArgs::default(), &host(), &[], None),
        InstallPlan {
            place: Some(p("/home/ann/.local/bin/temperare")),
            link: link_to("/home/ann/.local/bin/temperare"),
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
    assert_eq!(plan.link, link_to("/opt/homebrew/bin/temperare"));
    assert_eq!(
        plan.notes,
        vec!["temperare not copied: /opt/homebrew/bin/temperare is under /opt/homebrew, which a package manager owns"]
    );
}

#[test]
fn a_prefix_is_matched_by_whole_folders_only() {
    let host = InstallHost { exe: p("/usr/localtools/temperare"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.place, Some(p("/home/ann/.local/bin/temperare")));
}

#[test]
fn no_place_copies_nothing_and_links_to_the_running_binary() {
    let args = InstallArgs { no_place: true, ..Default::default() };
    let plan = plan_install(&args, &host(), &[], None);
    assert_eq!(plan.place, None);
    assert_eq!(plan.link, link_to("/tmp/dl/temperare"));
    assert_eq!(plan.notes, vec!["--no-place: temperare not copied; tm points at /tmp/dl/temperare"]);
}

#[test]
fn running_the_placed_binary_copies_nothing_and_still_links_to_it() {
    let host = InstallHost { exe: p("/home/ann/.local/bin/temperare"), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.place, None);
    assert_eq!(plan.link, link_to("/home/ann/.local/bin/temperare"));
    assert!(plan.notes.is_empty(), "{:?}", plan.notes);
}

#[test]
fn another_tm_on_path_skips_the_link_and_names_the_owner_and_the_override() {
    let plan = plan_install(&InstallArgs::default(), &host(), &[], Some(Path::new("/usr/bin/tm")));
    assert_eq!(plan.place, Some(p("/home/ann/.local/bin/temperare")));
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
    assert_eq!(plan.link, link_to("/home/ann/.local/bin/temperare"));
    assert_eq!(plan.notes, vec!["tm linked with --short-name, though /usr/bin/tm is also tm on PATH"]);
}

#[test]
fn skills_go_under_home_or_under_the_project_with_local() {
    let targets = [Harness::Claude, Harness::Cursor];
    let home = plan_install(&InstallArgs::default(), &host(), &targets, None);
    assert_eq!(
        home.skills,
        vec![p("/home/ann/.claude/skills/temperare/SKILL.md"), p("/home/ann/.cursor/skills/temperare/SKILL.md")]
    );
    let args = InstallArgs { local: true, ..Default::default() };
    let local = plan_install(&args, &host(), &[Harness::Claude], None);
    assert_eq!(local.skills, vec![p("/work/project/.claude/skills/temperare/SKILL.md")]);
}

#[test]
fn a_bin_folder_off_path_is_noted() {
    let host = InstallHost { path_var: Some(OsString::from("/usr/bin:/bin")), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.notes, vec!["/home/ann/.local/bin is not on PATH; add it to run temperare and tm by name"]);
}

#[test]
fn an_unset_path_notes_the_bin_folder_is_not_on_path() {
    let host = InstallHost { path_var: None, ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], None);
    assert_eq!(plan.notes, vec!["/home/ann/.local/bin is not on PATH; add it to run temperare and tm by name"]);
}

#[test]
fn nothing_in_the_bin_folder_needs_no_path_note() {
    let host = InstallHost { exe: p("/usr/local/bin/temperare"), path_var: Some(OsString::new()), ..host() };
    let plan = plan_install(&InstallArgs::default(), &host, &[], Some(Path::new("/usr/local/bin/tm")));
    assert_eq!((plan.place, plan.link), (None, None));
    assert_eq!(plan.notes.len(), 2, "{:?}", plan.notes);
    assert!(!plan.notes.iter().any(|note| note.contains("not on PATH")), "{:?}", plan.notes);
}

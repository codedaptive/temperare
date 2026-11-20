//! Unit tests for unrecordable_path.rs: the first path, in the order record, place, link, link target, skills, that is not
//! UTF-8 or holds a newline, a carriage return or a tab; none when every path is clean.

use std::path::{Path, PathBuf};

use super::unrecordable_path;
use crate::InstallPlan;

const RECORD: &str = "/h/.local/state/temperare/installed";

fn plan(place: &str, link: (&str, &str), skills: &[&str]) -> InstallPlan {
    InstallPlan {
        place: Some(PathBuf::from(place)),
        link: Some((PathBuf::from(link.0), PathBuf::from(link.1))),
        skills: skills.iter().map(PathBuf::from).collect(),
        notes: vec!["a note\twith a tab is not a path".to_string()],
    }
}

fn clean() -> InstallPlan {
    plan("/h/bin/temperare", ("/h/bin/tm", "/h/bin/temperare"), &["/h/.claude/skills/temperare/SKILL.md"])
}

#[test]
fn clean_paths_give_none() {
    assert_eq!(unrecordable_path(&clean(), Path::new(RECORD)), None);
}

#[test]
fn an_empty_plan_with_a_clean_record_gives_none() {
    let empty = InstallPlan { place: None, link: None, skills: vec![], notes: vec![] };
    assert_eq!(unrecordable_path(&empty, Path::new(RECORD)), None);
}

#[test]
fn a_newline_in_the_record_path_is_found() {
    assert_eq!(unrecordable_path(&clean(), Path::new("/h\n/installed")), Some(PathBuf::from("/h\n/installed")));
}

#[test]
fn a_newline_in_the_place_is_found() {
    let p = plan("/h\nfile /victim/temperare", ("/h/bin/tm", "/h/bin/temperare"), &[]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h\nfile /victim/temperare")));
}

#[test]
fn a_tab_in_the_link_is_found() {
    let p = plan("/h/bin/temperare", ("/h/b\tin/tm", "/h/bin/temperare"), &[]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h/b\tin/tm")));
}

#[test]
fn a_return_in_the_link_target_is_found() {
    let p = plan("/h/bin/temperare", ("/h/bin/tm", "/h/bin/temp\rerare"), &[]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h/bin/temp\rerare")));
}

#[test]
fn a_newline_in_a_later_skill_is_found() {
    let p = plan("/h/bin/temperare", ("/h/bin/tm", "/h/bin/temperare"), &["/h/a/SKILL.md", "/p\nq/SKILL.md"]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/p\nq/SKILL.md")));
}

#[test]
fn the_first_in_order_is_given_when_several_hold_one() {
    let p = plan("/h/b\tin/temperare", ("/h/bin/tm", "/h/bin/temperare"), &["/p\nq/SKILL.md"]);
    assert_eq!(unrecordable_path(&p, Path::new("/s\r/installed")), Some(PathBuf::from("/s\r/installed")));
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h/b\tin/temperare")));
}

#[test]
fn a_space_or_other_control_character_is_recordable() {
    let p = plan("/h/my bin/temperare", ("/h/bin/tm", "/h/bin/te\u{1}mperare"), &["/h/skills\u{b}/SKILL.md"]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), None);
}

#[cfg(unix)]
#[test]
fn a_place_that_is_not_utf8_is_found() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let lossy = PathBuf::from(OsStr::from_bytes(b"/h/\xff"));
    let mut p = clean();
    p.place = Some(lossy.clone());
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(lossy));
}

#[cfg(unix)]
#[test]
fn a_record_path_that_is_not_utf8_is_found() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let lossy = Path::new(OsStr::from_bytes(b"/h/\xff/installed"));
    assert_eq!(unrecordable_path(&clean(), lossy), Some(lossy.to_path_buf()));
}

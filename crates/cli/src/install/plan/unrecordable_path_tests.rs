//! Unit test for unrecordable_path.rs: the first path, in the order record, place, skills, that is not UTF-8 or holds a
//! newline, a carriage return or a tab; none when every path is clean.

use std::path::{Path, PathBuf};

use super::unrecordable_path;
use crate::InstallPlan;

const RECORD: &str = "/h/.local/state/cctao/installed";

fn plan(place: &str, skills: &[&str]) -> InstallPlan {
    InstallPlan {
        place: Some(PathBuf::from(place)),
        skills: skills.iter().map(PathBuf::from).collect(),
        notes: vec!["a note\twith a tab is not a path".to_string()],
    }
}

fn clean() -> InstallPlan {
    plan("/h/bin/cctao", &["/h/.claude/skills/cctao/SKILL.md"])
}

#[test]
fn clean_paths_give_none() {
    assert_eq!(unrecordable_path(&clean(), Path::new(RECORD)), None);
}

#[test]
fn an_empty_plan_with_a_clean_record_gives_none() {
    let empty = InstallPlan { place: None, skills: vec![], notes: vec![] };
    assert_eq!(unrecordable_path(&empty, Path::new(RECORD)), None);
}

#[test]
fn a_newline_in_the_record_path_is_found() {
    assert_eq!(unrecordable_path(&clean(), Path::new("/h\n/installed")), Some(PathBuf::from("/h\n/installed")));
}

#[test]
fn a_newline_in_the_place_is_found() {
    let p = plan("/h\nfile /victim/cctao", &[]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h\nfile /victim/cctao")));
}

#[test]
fn a_newline_in_a_later_skill_is_found() {
    let p = plan("/h/bin/cctao", &["/h/a/SKILL.md", "/p\nq/SKILL.md"]);
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/p\nq/SKILL.md")));
}

#[test]
fn the_first_in_order_is_given_when_several_hold_one() {
    let p = plan("/h/b\tin/cctao", &["/p\nq/SKILL.md"]);
    assert_eq!(unrecordable_path(&p, Path::new("/s\r/installed")), Some(PathBuf::from("/s\r/installed")));
    assert_eq!(unrecordable_path(&p, Path::new(RECORD)), Some(PathBuf::from("/h/b\tin/cctao")));
}

#[test]
fn a_space_or_other_control_character_is_recordable() {
    let p = plan("/h/my bin/cc\u{1}tao", &["/h/skills\u{b}/SKILL.md"]);
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

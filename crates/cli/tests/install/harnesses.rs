use crate::support::{detected, interactive_install, skill, Fixture};
use cli::{Harness, InstallArgs, SKILL_TEXT};
use std::fs;

#[test]
fn foreign_file_at_skill_path_is_kept() {
    let f = Fixture::new();
    let file = skill(&f.host.home, ".claude");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"user's skill").unwrap();
    let args = InstallArgs { yes: true, ..Default::default() };
    let (status, out, _) = f.install(&args);
    assert_eq!(status, 0);
    assert_eq!(fs::read(&file).unwrap(), b"user's skill");
    assert!(out.lines().any(|line| line.starts_with("kept ") && line.contains(file.to_str().unwrap())), "{out}");
    assert_eq!(f.uninstall().0, 0);
    assert_eq!(fs::read(&file).unwrap(), b"user's skill");
}

#[test]
fn reinstall_replaces_what_install_placed() {
    let f = Fixture::new();
    fs::create_dir(f.host.home.join(".claude")).unwrap();
    let args = InstallArgs { yes: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    fs::write(skill(&f.host.home, ".claude"), b"stale skill").unwrap();
    fs::write(&f.host.exe, b"updated executable").unwrap();
    let (status, _, _) = f.install(&args);
    assert_eq!(status, 0);
    let binary = f.host.home.join(".local/bin/temperare");
    let file = skill(&f.host.home, ".claude");
    assert_eq!(fs::read(&binary).unwrap(), b"updated executable");
    assert_eq!(fs::read_to_string(&file).unwrap(), SKILL_TEXT);
    assert_eq!(f.uninstall().0, 0);
    assert!(!binary.exists());
    assert!(!file.exists());
}

#[test]
fn yes_wires_every_detected_harness_with_the_skill_text() {
    let f = Fixture::new();
    detected(&f);
    let args = InstallArgs { yes: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    assert_eq!(fs::read_to_string(skill(&f.host.home, ".claude")).unwrap(), SKILL_TEXT);
    assert_eq!(fs::read_to_string(skill(&f.host.home, ".cursor")).unwrap(), SKILL_TEXT);
    assert!(!f.host.home.join(".codex").exists());
}

#[test]
fn undetected_explicit_target_is_not_wired_and_is_noted() {
    let f = Fixture::new();
    detected(&f);
    let args = InstallArgs { targets: Some(vec![Harness::Codex]), ..Default::default() };
    let (status, out, _) = f.install(&args);
    assert_eq!(status, 0);
    assert!(!skill(&f.host.home, ".claude").exists());
    assert!(!skill(&f.host.home, ".cursor").exists());
    assert!(!f.host.home.join(".codex").exists());
    assert!(out.to_lowercase().contains("codex"), "{out}");
}

#[test]
fn local_wires_project_instead_of_home() {
    let f = Fixture::new();
    fs::create_dir(f.host.home.join(".claude")).unwrap();
    let args = InstallArgs { yes: true, local: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    assert_eq!(fs::read_to_string(skill(&f.host.cwd, ".claude")).unwrap(), SKILL_TEXT);
    assert!(!skill(&f.host.home, ".claude").exists());
}

#[test]
fn interactive_picker_receives_detected_harnesses_and_wires_only_selection() {
    let mut f = Fixture::new();
    detected(&f);
    f.host.interactive = true;
    let mut calls = 0;
    let (status, _) = interactive_install(&f, &mut |choices| {
        calls += 1;
        assert_eq!(choices, &[Harness::Claude, Harness::Cursor]);
        vec![Harness::Cursor]
    });
    assert_eq!(status, 0);
    assert_eq!(calls, 1);
    assert!(!skill(&f.host.home, ".claude").exists());
    assert_eq!(fs::read_to_string(skill(&f.host.home, ".cursor")).unwrap(), SKILL_TEXT);
    assert!(!f.host.home.join(".codex").exists());
}

#[test]
fn noninteractive_without_yes_or_targets_wires_nothing_and_explains_how() {
    let f = Fixture::new();
    detected(&f);
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert!(!skill(&f.host.home, ".claude").exists());
    assert!(!skill(&f.host.home, ".cursor").exists());
    assert!(!f.host.home.join(".codex").exists());
    assert!(out.contains("--target"), "{out}");
    assert!(out.contains("-y"), "{out}");
}

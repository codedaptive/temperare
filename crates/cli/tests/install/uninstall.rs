use crate::support::{skill, snapshot, Fixture};
use cli::{run_uninstall, InstallArgs};
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

struct RemovalHook<F> {
    trigger: String,
    pending: Vec<u8>,
    action: F,
    result: Option<io::Result<()>>,
}

impl<F: FnMut() -> io::Result<()>> Write for RemovalHook<F> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.pending.extend_from_slice(bytes);
        while let Some(end) = self.pending.iter().position(|byte| *byte == b'\n') {
            if self.result.is_none() && self.pending[..end] == *self.trigger.as_bytes() {
                self.result = Some((self.action)());
            }
            self.pending.drain(..=end);
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct RestoreMode {
    path: PathBuf,
    original: fs::Permissions,
}

impl Drop for RestoreMode {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.path, self.original.clone());
    }
}

#[test]
fn a_newline_in_home_refuses_install() {
    let mut f = Fixture::new();
    let injected_prefix = f.root.join("victim");
    let victim = injected_prefix.join(".local/bin/cctao");
    fs::create_dir_all(victim.parent().unwrap()).unwrap();
    fs::write(&victim, b"victim").unwrap();
    f.host.home = f.root.join(format!("h\nfile {}", injected_prefix.display()));
    fs::create_dir_all(&f.host.home).unwrap();
    let (status, _, err) = f.install(&InstallArgs::default());
    assert_eq!(status, 1);
    assert!(err.contains("newline"), "{err}");
    assert!(!f.host.home.join(".local").exists());
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
    let before = snapshot(&f.host.home);
    assert_eq!(f.uninstall().0, 0);
    assert_eq!(snapshot(&f.host.home), before);
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
}

#[test]
fn a_failed_folder_removal_keeps_the_record() {
    let f = Fixture::new();
    fs::create_dir(f.host.home.join(".claude")).unwrap();
    let args = InstallArgs { yes: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    let parent = f.host.home.join(".claude/skills");
    let mut out = RemovalHook {
        trigger: format!("removed {}", skill(&f.host.home, ".claude").display()),
        pending: Vec::new(),
        action: || {
            fs::rename(&parent, f.root.join("saved-skills"))?;
            fs::write(&parent, b"block")
        },
        result: None,
    };
    let mut err = Vec::new();
    let status = run_uninstall(&f.host, &mut out, &mut err);
    assert!(matches!(out.result, Some(Ok(()))), "hook did not succeed: {:?}", out.result);
    let err = String::from_utf8(err).unwrap();
    let record_exists = f.host.home.join(".local/state/cctao/installed").exists();
    assert_eq!(status, 1);
    assert!(record_exists);
    assert!(err.contains("run cctao uninstall again"), "{err}");
}

#[test]
fn a_failed_state_folder_removal_names_the_state_folder() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let parent = f.host.home.join(".local/state");
    let state = parent.join("cctao");
    let mut out = RemovalHook {
        trigger: format!("removed {}", state.join("installed").display()),
        pending: Vec::new(),
        action: || {
            fs::rename(&parent, f.root.join("saved-state"))?;
            fs::write(&parent, b"block")
        },
        result: None,
    };
    let mut err = Vec::new();
    let status = run_uninstall(&f.host, &mut out, &mut err);
    assert!(matches!(out.result, Some(Ok(()))), "hook did not succeed: {:?}", out.result);
    let err = String::from_utf8(err).unwrap();
    let record_exists = state.join("installed").exists();
    assert_eq!(status, 1);
    assert!(!record_exists);
    let expected = format!(
        "next: The record is removed; remove each of these folders by hand only if it is empty: {}, {}, {}.",
        f.host.home.join(".local/state/cctao").display(),
        f.host.home.join(".local/state").display(),
        f.host.home.join(".local").display(),
    );
    assert!(err.lines().any(|line| line == expected), "{err}");
}

#[test]
fn a_failed_local_folder_removal_names_the_remaining_folder() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let _restore = RestoreMode {
        path: f.host.home.clone(),
        original: fs::metadata(&f.host.home).unwrap().permissions(),
    };
    let mut out = RemovalHook {
        trigger: format!("removed {}", f.host.home.join(".local/state").display()),
        pending: Vec::new(),
        // ENOTDIR cannot block only the final .local removal; root can bypass this mode and must fail the test.
        action: || fs::set_permissions(&f.host.home, fs::Permissions::from_mode(0o555)),
        result: None,
    };
    let mut err = Vec::new();
    let status = run_uninstall(&f.host, &mut out, &mut err);
    assert!(matches!(out.result, Some(Ok(()))), "hook did not succeed: {:?}", out.result);
    let err = String::from_utf8(err).unwrap();
    assert_eq!(status, 1);
    assert!(!f.host.home.join(".local/state").exists());
    assert!(f.host.home.join(".local").is_dir());
    let expected = format!(
        "next: The record is removed; remove each of these folders by hand only if it is empty: {}.",
        f.host.home.join(".local").display(),
    );
    assert!(err.lines().any(|line| line == expected), "{err}");
}

#[test]
fn a_carriage_return_in_home_refuses_install() {
    let mut f = Fixture::new();
    f.host.home = f.root.join("h\rx");
    fs::create_dir(&f.host.home).unwrap();
    let (status, _, err) = f.install(&InstallArgs::default());
    assert_eq!(status, 1);
    assert!(err.contains("carriage return"), "{err}");
}

#[test]
fn uninstall_restores_home_snapshot_and_preserves_preexisting_bin() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::write(bin.join("unrelated"), b"other binary").unwrap();
    let other = f.host.home.join(".claude/skills/other/SKILL.md");
    fs::create_dir_all(other.parent().unwrap()).unwrap();
    fs::write(&other, b"other skill").unwrap();
    let before = snapshot(&f.host.home);
    let args = InstallArgs { yes: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    assert!(f.host.home.join(".claude/skills/cctao/SKILL.md").exists());
    assert_eq!(f.uninstall().0, 0);
    assert!(bin.is_dir());
    assert_eq!(snapshot(&f.host.home), before);
}

#[test]
fn uninstall_removes_bin_directory_created_by_install() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    assert!(!bin.exists());
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert!(bin.join("cctao").is_file());
    assert_eq!(f.uninstall().0, 0);
    assert!(!bin.exists());
}

#[test]
fn uninstall_without_record_succeeds_without_removing_anything() {
    let f = Fixture::new();
    let file = f.host.home.join(".local/bin/tm");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"user's tm").unwrap();
    let before = snapshot(&f.host.home);
    let (status, out, _) = f.uninstall();
    assert_eq!(status, 0);
    assert!(out.contains("nothing to remove"), "{out}");
    assert_eq!(snapshot(&f.host.home), before);
}

#[test]
fn a_user_tm_in_the_bin_folder_survives_install_and_uninstall() {
    let f = Fixture::new();
    let file = f.host.home.join(".local/bin/tm");
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, b"user's tm").unwrap();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert!(fs::symlink_metadata(&file).unwrap().file_type().is_file());
    assert_eq!(fs::read(&file).unwrap(), b"user's tm");
    assert_eq!(f.uninstall().0, 0);
    assert!(fs::symlink_metadata(&file).unwrap().file_type().is_file());
    assert_eq!(fs::read(&file).unwrap(), b"user's tm");
}

use crate::support::{skill, snapshot, Fixture};
use cli::{run_install, InstallArgs};
use std::ffi::OsStr;
use std::fs;
use std::io::{self, Write};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

#[test]
fn a_home_that_is_not_utf8_refuses_install() {
    let mut f = Fixture::new();
    f.host.home = f.root.join(OsStr::from_bytes(b"h\xff"));
    let before = snapshot(&f.root);
    let (status, _, err) = f.install(&InstallArgs::default());
    assert_eq!(status, 1);
    assert!(err.contains("UTF-8"), "{err}");
    assert_eq!(snapshot(&f.root), before);
}

struct BlockRecord {
    record: PathBuf,
    saved: PathBuf,
    output: Vec<u8>,
    blocked: bool,
    moved: bool,
}

impl Write for BlockRecord {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(bytes);
        if !self.blocked && self.output.split(|byte| *byte == b'\n').any(|line| line.starts_with(b"placed ")) {
            if self.record.exists() {
                fs::rename(&self.record, &self.saved)?;
                self.moved = true;
            }
            fs::create_dir(&self.record)?;
            fs::write(self.record.join("blocker"), b"keep")?;
            self.blocked = true;
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn install_with_blocked_record(f: &Fixture, args: &InstallArgs) -> (String, String) {
    let state = f.host.home.join(".local/state/temperare");
    let mut out = BlockRecord {
        record: state.join("installed"),
        saved: f.root.join("saved-record"),
        output: Vec::new(),
        blocked: false,
        moved: false,
    };
    let mut err = Vec::new();
    let status = run_install(args, &f.host, &mut |_| panic!("unexpected picker"), &mut out, &mut err);
    assert_eq!(status, 1);
    assert!(out.blocked, "the placed line must trigger the record blocker");
    assert_eq!(fs::read(out.record.join("blocker")).unwrap(), b"keep");
    assert!(fs::read_dir(&state).unwrap().all(|entry| !entry.unwrap().file_name().to_string_lossy().ends_with(".installing")));
    fs::remove_dir_all(&out.record).unwrap();
    if out.moved {
        fs::rename(&out.saved, &out.record).unwrap();
    }
    (String::from_utf8(out.output).unwrap(), String::from_utf8(err).unwrap())
}

#[test]
fn first_install_failed_record_names_every_uncovered_path() {
    let f = Fixture::new();
    let (_, err) = install_with_blocked_record(&f, &InstallArgs::default());
    let binary = f.host.home.join(".local/bin/temperare");
    assert_eq!(f.uninstall().0, 0);
    assert!(binary.exists());
    let expected = format!(
        "next: The install record was not written, so temperare uninstall will not remove what this run placed. Remove these by hand: {}, {}; then remove each of these folders only if it is empty: {}, {}, {}, {}; or fix the cause and run temperare install again.",
        binary.display(),
        f.host.home.join(".local/bin/tm").display(),
        f.host.home.join(".local/bin").display(),
        f.host.home.join(".local/state/temperare").display(),
        f.host.home.join(".local/state").display(),
        f.host.home.join(".local").display(),
    );
    assert_eq!(err.lines().find(|line| line.starts_with("next: ")).unwrap(), expected);
}

#[test]
fn reinstall_failed_record_with_nothing_added_remains_covered() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let (_, err) = install_with_blocked_record(&f, &InstallArgs::default());
    let binary = f.host.home.join(".local/bin/temperare");
    assert_eq!(f.uninstall().0, 0);
    assert!(!binary.exists());
    assert_eq!(
        err.lines().find(|line| line.starts_with("next: ")).unwrap(),
        "next: The install record was not written; the earlier record is kept and names everything this run placed, so temperare uninstall still covers it. Fix the cause and run temperare install again."
    );
}

#[test]
fn reinstall_failed_record_names_new_skill_paths_as_uncovered() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    fs::create_dir(f.host.home.join(".claude")).unwrap();
    let args = InstallArgs { yes: true, ..Default::default() };
    let (out, err) = install_with_blocked_record(&f, &args);
    let file = skill(&f.host.home, ".claude");
    assert!(out.lines().any(|line| line == format!("wired skill {}", file.display())), "{out}");
    assert_eq!(f.uninstall().0, 0);
    assert!(!f.host.home.join(".local/bin/temperare").exists());
    assert!(file.exists());
    let expected = format!(
        "next: The install record was not written; the earlier record is kept, so temperare uninstall covers what it names but not what this run added. Remove these by hand: {}; then remove each of these folders only if it is empty: {}, {}; or fix the cause and run temperare install again.",
        file.display(),
        f.host.home.join(".claude/skills/temperare").display(),
        f.host.home.join(".claude/skills").display(),
    );
    assert_eq!(err.lines().find(|line| line.starts_with("next: ")).unwrap(), expected);
}

#[test]
fn a_symlink_at_the_record_path_is_refused_and_its_target_untouched() {
    let f = Fixture::new();
    let victim2 = f.root.join("victim2");
    fs::write(&victim2, b"keep").unwrap();
    let victim = f.root.join("victim");
    let victim_contents = format!("file {}\n", victim2.display());
    fs::write(&victim, &victim_contents).unwrap();
    let record = f.host.home.join(".local/state/temperare/installed");
    fs::create_dir_all(record.parent().unwrap()).unwrap();
    symlink(&victim, &record).unwrap();

    let (status, _, err) = f.install(&InstallArgs::default());
    assert_eq!(status, 1);
    assert!(err.contains("symbolic link"), "{err}");
    assert_eq!(fs::read(&victim).unwrap(), victim_contents.as_bytes());
    assert_eq!(fs::read(&victim2).unwrap(), b"keep");
    assert!(!f.host.home.join(".local/bin/temperare").exists());
    assert!(fs::symlink_metadata(&record).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_link(&record).unwrap(), victim);

    assert_eq!(f.uninstall().0, 1);
    assert_eq!(fs::read(&victim2).unwrap(), b"keep");
    assert_eq!(fs::read(&victim).unwrap(), victim_contents.as_bytes());
}

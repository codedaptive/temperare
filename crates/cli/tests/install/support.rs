use cli::{run_install, run_uninstall, Harness, InstallArgs, InstallHost};
use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT: AtomicUsize = AtomicUsize::new(0);
pub const EXE_BYTES: &[u8] = b"fake temperare executable\n";

pub struct Fixture {
    pub root: PathBuf,
    pub host: InstallHost,
}

impl Fixture {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "temperare-install-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).unwrap();
        let home = root.join("home");
        let cwd = root.join("project");
        let path = root.join("path");
        fs::create_dir(&home).unwrap();
        fs::create_dir(&cwd).unwrap();
        fs::create_dir(&path).unwrap();
        let exe = root.join("source/temperare");
        executable(&exe, EXE_BYTES);
        let host = InstallHost {
            home,
            path_var: Some(path.into_os_string()),
            exe,
            cwd,
            prefixes: vec![],
            interactive: false,
        };
        Self { root, host }
    }

    pub fn install(&self, args: &InstallArgs) -> (u8, String, String) {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = run_install(args, &self.host, &mut |_| panic!("unexpected picker"), &mut out, &mut err);
        (status, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
    }

    pub fn uninstall(&self) -> (u8, String, String) {
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status = run_uninstall(&self.host, &mut out, &mut err);
        (status, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).unwrap();
    }
}

pub fn executable(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

pub fn detected(fixture: &Fixture) {
    fs::create_dir(fixture.host.home.join(".claude")).unwrap();
    fs::create_dir(fixture.host.home.join(".cursor")).unwrap();
}

pub fn skill(base: &Path, folder: &str) -> PathBuf {
    base.join(folder).join("skills/temperare/SKILL.md")
}

#[derive(Debug, PartialEq, Eq)]
pub enum Entry {
    Dir,
    File(Vec<u8>),
    Link(PathBuf),
}

pub fn snapshot(root: &Path) -> BTreeMap<PathBuf, Entry> {
    fn visit(root: &Path, dir: &Path, result: &mut BTreeMap<PathBuf, Entry>) {
        for item in fs::read_dir(dir).unwrap() {
            let path = item.unwrap().path();
            let kind = fs::symlink_metadata(&path).unwrap().file_type();
            let entry = if kind.is_symlink() {
                Entry::Link(fs::read_link(&path).unwrap())
            } else if kind.is_dir() {
                visit(root, &path, result);
                Entry::Dir
            } else {
                Entry::File(fs::read(&path).unwrap())
            };
            result.insert(path.strip_prefix(root).unwrap().to_path_buf(), entry);
        }
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result);
    result
}

pub fn interactive_install(
    fixture: &Fixture,
    pick: &mut dyn FnMut(&[Harness]) -> Vec<Harness>,
) -> (u8, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    let status = run_install(&InstallArgs::default(), &fixture.host, pick, &mut out, &mut err);
    (status, String::from_utf8(out).unwrap())
}

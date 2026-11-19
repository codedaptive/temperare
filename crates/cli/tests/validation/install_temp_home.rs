//! Validation (creator #265; S-32, S-33, S-37): the built binary's `install -y` and `uninstall` in a throwaway HOME
//! that holds a Claude Code folder, with a PATH of ~/.local/bin, /usr/bin and /bin. Install places the real file
//! `temperare` and the link `tm` in ~/.local/bin and writes the skill; `tm --version` through the link answers as the
//! built binary does, and the install record is written. Uninstall removes the binary, the link, the skill, the record
//! and the folders install made, and leaves what was there before: the harness folder and another skill in it.

use std::fs;
use std::path::{Path, PathBuf};

use crate::run_clean::run_clean;

/// The throwaway folder; removed when the case ends, pass or fail.
struct Throwaway(PathBuf);

impl Drop for Throwaway {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn vars(home: &Path) -> Vec<(String, String)> {
    let mut vars = vec![
        ("HOME".to_string(), home.display().to_string()),
        ("PATH".to_string(), format!("{}/.local/bin:/usr/bin:/bin", home.display())),
        ("LANG".to_string(), "C".to_string()),
    ];
    if let Ok(tmp) = std::env::var("TMPDIR") {
        vars.push(("TMPDIR".to_string(), tmp));
    }
    vars
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).to_string()
}

#[cfg(unix)]
#[test]
fn install_y_places_temperare_tm_and_the_skill_and_uninstall_removes_them() {
    use std::os::unix::fs::PermissionsExt;

    let built = env!("CARGO_BIN_EXE_temperare");
    let root = std::env::temp_dir().join(format!("temperare-validation-install-{}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = Throwaway(fs::canonicalize(&root).unwrap());
    let home = root.0.join("home");
    let other_skill = home.join(".claude/skills/other/SKILL.md");
    fs::create_dir_all(other_skill.parent().unwrap()).unwrap();
    fs::write(&other_skill, "another tool's skill\n").unwrap();
    let bin = home.join(".local/bin/temperare");
    let link = home.join(".local/bin/tm");
    let skill = home.join(".claude/skills/temperare/SKILL.md");
    let record = home.join(".local/state/temperare/installed");

    let out = run_clean(built, &["install", "-y"], &root.0, vars(&home));
    let said = format!("stdout: {}\nstderr: {}", text(&out.stdout), text(&out.stderr));
    assert_eq!(out.code, Some(0), "install -y succeeds: {said}");

    let meta = fs::symlink_metadata(&bin).unwrap_or_else(|e| panic!("no {}: {e}\n{said}", bin.display()));
    assert!(meta.is_file(), "temperare is a real file, not a link: {said}");
    assert_ne!(meta.permissions().mode() & 0o111, 0, "temperare is executable");
    assert_eq!(fs::read(&bin).unwrap(), fs::read(built).unwrap(), "the placed file is the built binary");
    let link_meta = fs::symlink_metadata(&link).unwrap_or_else(|e| panic!("no {}: {e}\n{said}", link.display()));
    assert!(link_meta.file_type().is_symlink(), "tm is a symlink, never a hard link (S-32)");
    assert_eq!(fs::canonicalize(&link).unwrap(), bin, "tm points at the placed temperare");
    let skill_text = fs::read_to_string(&skill).unwrap_or_else(|e| panic!("no {}: {e}\n{said}", skill.display()));
    assert!(skill_text.starts_with("---\nname: temperare\n"), "the skill names temperare: {skill_text}");
    assert!(skill_text.contains("`temperare check"), "the skill tells the agent to run temperare check");

    assert!(record.is_file(), "install writes its record at {}: {said}", record.display());

    let want = run_clean(built, &["--version"], &root.0, vars(&home));
    let got = run_clean(link.clone(), &["--version"], &root.0, vars(&home));
    assert_eq!(got.code, Some(0), "tm --version through the link: {}", text(&got.stderr));
    assert!(text(&got.stdout).starts_with("temperare "), "the version names temperare: {}", text(&got.stdout));
    assert_eq!(text(&got.stdout), text(&want.stdout), "the installed tm answers as the built binary");

    let gone = run_clean(bin.clone(), &["uninstall"], &root.0, vars(&home));
    let said = format!("stdout: {}\nstderr: {}", text(&gone.stdout), text(&gone.stderr));
    assert_eq!(gone.code, Some(0), "uninstall succeeds: {said}");
    for placed in [&bin, &link, &skill, &record] {
        let err = fs::symlink_metadata(placed).expect_err(&format!("uninstall removed {}: {said}", placed.display()));
        assert_eq!(err.kind(), std::io::ErrorKind::NotFound, "{} is gone: {err}", placed.display());
    }
    let left = everything_under(&home);
    let want_left: Vec<PathBuf> = [".claude", ".claude/skills", ".claude/skills/other", ".claude/skills/other/SKILL.md"]
        .iter()
        .map(|p| home.join(p))
        .collect();
    assert_eq!(left, want_left, "uninstall leaves only what was there before install: {said}");
    assert_eq!(fs::read_to_string(&other_skill).unwrap(), "another tool's skill\n", "the other skill is untouched");
}

/// Every path under `dir`, links not followed, sorted.
fn everything_under(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        found.push(path.clone());
        if fs::symlink_metadata(&path).unwrap().is_dir() {
            found.extend(everything_under(&path));
        }
    }
    found.sort();
    found
}

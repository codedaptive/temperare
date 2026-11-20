//! Integration test for savings_line_on, on a HOME folder the test made: config reads, defaults, diagnostics and
//! unchanged files.

use crate::home_files::savings_line_on;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

struct Home(PathBuf);
impl Home {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!("tm-config-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn config(&self, bytes: &[u8]) -> PathBuf {
        let path = self.0.join(".config/temperare/config.toml");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, bytes).unwrap();
        path
    }
}
impl Drop for Home {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

#[test]
fn savings_contract_absent_or_empty_home_defaults_silently() {
    let mut stderr = Vec::new();
    assert!(savings_line_on(None, &mut stderr));
    assert!(savings_line_on(Some(Path::new("")), &mut stderr));
    assert!(stderr.is_empty());
}

#[test]
fn savings_contract_missing_file_defaults_silently() {
    let home = Home::new();
    let mut stderr = Vec::new();
    assert!(savings_line_on(Some(&home.0), &mut stderr));
    assert!(stderr.is_empty());
}

#[test]
fn savings_contract_false_switch_disables_line_without_changing_file() {
    let home = Home::new();
    let path = home.config(b"savings_line = false\n");
    let mut stderr = Vec::new();
    assert!(!savings_line_on(Some(&home.0), &mut stderr));
    assert!(stderr.is_empty());
    assert_eq!(std::fs::read(path).unwrap(), b"savings_line = false\n");
}

#[test]
fn savings_contract_bad_toml_defaults_with_one_diagnostic_and_preserves_file() {
    let home = Home::new();
    let path = home.config(b"savings_line = ");
    let mut stderr = Vec::new();
    assert!(savings_line_on(Some(&home.0), &mut stderr));
    let text = std::str::from_utf8(&stderr).unwrap();
    let reason = text.strip_prefix("tm: ignoring ~/.config/temperare/config.toml: ").unwrap();
    assert!(!reason.trim().is_empty(), "{text:?}");
    assert!(text.ends_with('\n'));
    assert_eq!(text.lines().count(), 1);
    assert_eq!(std::fs::read(path).unwrap(), b"savings_line = ");
}

#[test]
fn savings_contract_unknown_key_defaults_with_exact_diagnostic_and_preserves_file() {
    let home = Home::new();
    let path = home.config(b"colour = 1\n");
    let mut stderr = Vec::new();
    assert!(savings_line_on(Some(&home.0), &mut stderr));
    assert_eq!(stderr, b"tm: ignoring ~/.config/temperare/config.toml: unknown key `colour`\n");
    assert_eq!(std::fs::read(path).unwrap(), b"colour = 1\n");
}

#[test]
fn savings_contract_invalid_utf8_defaults_with_one_diagnostic() {
    let home = Home::new();
    let path = home.config(b"\xff");
    let mut stderr = Vec::new();
    assert!(savings_line_on(Some(&home.0), &mut stderr));
    let text = std::str::from_utf8(&stderr).unwrap();
    let reason = text.strip_prefix("tm: ignoring ~/.config/temperare/config.toml: ").unwrap();
    assert!(!reason.trim().is_empty(), "{text:?}");
    assert!(text.ends_with('\n'));
    assert_eq!(text.lines().count(), 1);
    assert_eq!(std::fs::read(path).unwrap(), b"\xff");
}

#[test]
fn savings_contract_ignores_stderr_write_failure() {
    struct Broken;
    impl std::io::Write for Broken {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> { Err(std::io::Error::other("broken")) }
        fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
    }
    let home = Home::new();
    home.config(b"colour = 1");
    assert!(savings_line_on(Some(&home.0), &mut Broken));
}

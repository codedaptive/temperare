use crate::support::{executable, Fixture, EXE_BYTES};
use cli::InstallArgs;
use std::fs;
use std::os::unix::fs::{symlink, MetadataExt, PermissionsExt};

#[test]
fn places_executable_with_source_bytes() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let binary = f.host.home.join(".local/bin/cctao");
    let placed = fs::symlink_metadata(&binary).unwrap();
    let source = fs::metadata(&f.host.exe).unwrap();
    assert!(placed.file_type().is_file());
    assert_ne!((placed.dev(), placed.ino()), (source.dev(), source.ino()));
    assert_eq!(fs::read(&binary).unwrap(), EXE_BYTES);
    assert_ne!(placed.permissions().mode() & 0o100, 0);
}

#[test]
fn foreign_file_at_binary_path_is_kept() {
    let f = Fixture::new();
    let binary = f.host.home.join(".local/bin/cctao");
    fs::create_dir_all(binary.parent().unwrap()).unwrap();
    fs::write(&binary, b"user's cctao").unwrap();
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert_eq!(fs::read(&binary).unwrap(), b"user's cctao");
    assert!(out.lines().any(|line| line.starts_with("kept ") && line.contains(binary.to_str().unwrap())), "{out}");
    assert_eq!(f.uninstall().0, 0);
    assert_eq!(fs::read(&binary).unwrap(), b"user's cctao");
}

#[test]
fn failed_reinstall_cleans_up_the_temporary_file() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    let binary = bin.join("cctao");
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    fs::remove_file(&binary).unwrap();
    fs::create_dir(&binary).unwrap();

    assert_eq!(f.install(&InstallArgs::default()).0, 1);
    assert!(fs::symlink_metadata(&binary).unwrap().file_type().is_dir());
    assert_eq!(fs::read_dir(&binary).unwrap().count(), 0);
    assert!(fs::read_dir(&bin).unwrap().all(|entry| {
        !entry.unwrap().file_name().to_string_lossy().ends_with(".installing")
    }));
}

#[test]
fn a_symlink_at_the_old_temp_name_is_not_followed() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    let victim = f.root.join("victim");
    fs::write(&victim, b"victim").unwrap();
    let temp = bin.join(".cctao.installing");
    symlink(&victim, &temp).unwrap();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
    assert!(fs::symlink_metadata(&temp).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_link(&temp).unwrap(), victim);
    assert_eq!(fs::read(bin.join("cctao")).unwrap(), EXE_BYTES);
}

#[test]
fn a_symlink_at_the_first_temp_name_is_not_followed_and_install_moves_on() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    let victim = f.root.join("victim");
    fs::write(&victim, b"victim").unwrap();
    let temp = bin.join(format!(".cctao.{}.0.installing", std::process::id()));
    symlink(&victim, &temp).unwrap();

    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
    assert!(fs::symlink_metadata(&temp).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_link(&temp).unwrap(), victim);
    let binary = bin.join("cctao");
    assert!(fs::symlink_metadata(&binary).unwrap().file_type().is_file());
    assert_eq!(fs::read(&binary).unwrap(), EXE_BYTES);
}

#[test]
fn default_install_places_only_cctao() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let bin = f.host.home.join(".local/bin");
    let mut names = fs::read_dir(&bin).unwrap().map(|entry| entry.unwrap().file_name()).collect::<Vec<_>>();
    names.sort();
    assert_eq!(names, ["cctao"]);
    assert!(fs::symlink_metadata(bin.join("cctao")).unwrap().file_type().is_file());
}

#[test]
fn package_prefix_skips_placement() {
    let mut f = Fixture::new();
    let prefix = f.root.join("prefix");
    f.host.exe = prefix.join("bin/cctao");
    executable(&f.host.exe, EXE_BYTES);
    f.host.prefixes = vec![prefix];
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert!(!f.host.home.join(".local/bin").exists());
}

#[test]
fn no_place_skips_placement() {
    let f = Fixture::new();
    let args = InstallArgs { no_place: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    assert!(!f.host.home.join(".local/bin").exists());
}

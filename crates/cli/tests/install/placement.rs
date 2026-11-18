use crate::support::{executable, Fixture, EXE_BYTES};
use cli::InstallArgs;
use std::fs;
use std::os::unix::fs::{symlink, MetadataExt, PermissionsExt};

#[test]
fn places_executable_with_source_bytes() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    let binary = f.host.home.join(".local/bin/temperare");
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
    let binary = f.host.home.join(".local/bin/temperare");
    fs::create_dir_all(binary.parent().unwrap()).unwrap();
    fs::write(&binary, b"user's temperare").unwrap();
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert_eq!(fs::read(&binary).unwrap(), b"user's temperare");
    assert!(out.lines().any(|line| line.starts_with("kept ") && line.contains(binary.to_str().unwrap())), "{out}");
    assert!(fs::symlink_metadata(f.host.home.join(".local/bin/tm")).is_err());
    assert!(out.contains("tm not linked"), "{out}");
    assert_eq!(f.uninstall().0, 0);
    assert_eq!(fs::read(&binary).unwrap(), b"user's temperare");
}

#[test]
fn failed_reinstall_cleans_up_the_temporary_file() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    let binary = bin.join("temperare");
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
    let temp = bin.join(".temperare.installing");
    symlink(&victim, &temp).unwrap();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
    assert!(fs::symlink_metadata(&temp).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_link(&temp).unwrap(), victim);
    assert_eq!(fs::read(bin.join("temperare")).unwrap(), EXE_BYTES);
}

#[test]
fn a_symlink_at_the_first_temp_name_is_not_followed_and_install_moves_on() {
    let f = Fixture::new();
    let bin = f.host.home.join(".local/bin");
    fs::create_dir_all(&bin).unwrap();
    let victim = f.root.join("victim");
    fs::write(&victim, b"victim").unwrap();
    let temp = bin.join(format!(".temperare.{}.0.installing", std::process::id()));
    symlink(&victim, &temp).unwrap();

    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert_eq!(fs::read(&victim).unwrap(), b"victim");
    assert!(fs::symlink_metadata(&temp).unwrap().file_type().is_symlink());
    assert_eq!(fs::read_link(&temp).unwrap(), victim);
    let binary = bin.join("temperare");
    assert!(fs::symlink_metadata(&binary).unwrap().file_type().is_file());
    assert_eq!(fs::read(&binary).unwrap(), EXE_BYTES);
}

#[test]
fn an_empty_path_is_the_working_folder_alone() {
    let mut f = Fixture::new();
    let owner = f.host.cwd.join("tm");
    executable(&owner, b"another tm");
    f.host.path_var = Some("".into());
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert!(fs::symlink_metadata(f.host.home.join(".local/bin/tm")).is_err());
    assert!(out.contains(owner.to_str().unwrap()), "{out}");
    assert!(out.contains("--short-name"), "{out}");
}

#[test]
fn an_unset_path_is_no_search() {
    let mut f = Fixture::new();
    executable(&f.host.cwd.join("tm"), b"another tm");
    f.host.path_var = None;
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert_eq!(fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(),
        f.host.home.join(".local/bin/temperare"));
    assert!(!out.contains("--short-name"), "{out}");
}

#[test]
fn an_empty_path_entry_is_the_working_folder() {
    let mut f = Fixture::new();
    let owner = f.host.cwd.join("tm");
    executable(&owner, b"another tm");
    f.host.path_var = Some(format!(":{}", f.root.join("path").display()).into());
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert!(fs::symlink_metadata(f.host.home.join(".local/bin/tm")).is_err());
    assert!(out.contains(owner.to_str().unwrap()), "{out}");
    assert!(out.contains("--short-name"), "{out}");
}

#[test]
fn a_relative_path_entry_is_read_from_the_working_folder() {
    let mut f = Fixture::new();
    let owner = f.host.cwd.join("bin/tm");
    executable(&owner, b"another tm");
    f.host.path_var = Some("bin".into());
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert!(fs::symlink_metadata(f.host.home.join(".local/bin/tm")).is_err());
    assert!(out.contains(owner.to_str().unwrap()), "{out}");
    assert!(out.contains("--short-name"), "{out}");
}

#[test]
fn links_tm_to_the_placed_binary() {
    let f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert_eq!(
        fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(),
        f.host.home.join(".local/bin/temperare")
    );
}

#[test]
fn clash_skips_link_and_names_owner_and_override() {
    let f = Fixture::new();
    let owner = f.root.join("path/tm");
    executable(&owner, b"another tm");
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert!(fs::symlink_metadata(f.host.home.join(".local/bin/tm")).is_err());
    assert!(out.contains(owner.to_str().unwrap()), "{out}");
    assert!(out.contains("--short-name"), "{out}");
}

#[test]
fn short_name_overrides_a_path_clash() {
    let f = Fixture::new();
    executable(&f.root.join("path/tm"), b"another tm");
    let args = InstallArgs { short_name: true, ..Default::default() };
    let (status, _, _) = f.install(&args);
    assert_eq!(status, 0);
    assert_eq!(fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(),
        f.host.home.join(".local/bin/temperare"));
}

#[test]
fn existing_own_link_on_path_is_not_a_clash() {
    let mut f = Fixture::new();
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    f.host.path_var = Some(f.host.home.join(".local/bin").into_os_string());
    let (status, out, _) = f.install(&InstallArgs::default());
    assert_eq!(status, 0);
    assert_eq!(fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(),
        f.host.home.join(".local/bin/temperare"));
    assert!(!out.contains("--short-name"), "{out}");
}

#[test]
fn package_prefix_skips_placement_and_links_to_running_exe() {
    let mut f = Fixture::new();
    let prefix = f.root.join("prefix");
    f.host.exe = prefix.join("bin/temperare");
    executable(&f.host.exe, EXE_BYTES);
    f.host.prefixes = vec![prefix];
    assert_eq!(f.install(&InstallArgs::default()).0, 0);
    assert!(!f.host.home.join(".local/bin/temperare").exists());
    assert_eq!(fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(), f.host.exe);
}

#[test]
fn no_place_skips_placement_and_links_to_running_exe() {
    let f = Fixture::new();
    let args = InstallArgs { no_place: true, ..Default::default() };
    assert_eq!(f.install(&args).0, 0);
    assert!(!f.host.home.join(".local/bin/temperare").exists());
    assert_eq!(fs::read_link(f.host.home.join(".local/bin/tm")).unwrap(), f.host.exe);
}

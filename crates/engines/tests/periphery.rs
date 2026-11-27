#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::sync::Mutex;

use engines::{run_rules_versioned_at, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());

struct TempFolder(PathBuf);
impl TempFolder {
    fn new(case: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-periphery-{case}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}
impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);
impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

fn offline_swift(outer: &TempFolder, saved_path: &PathGuard) {
    let swift = procio::run(&procio::Cmd {
        args: vec!["--find".into(), "swift".into()],
        ..procio::Cmd::new("/usr/bin/xcrun")
    }).unwrap();
    assert_eq!(swift.code, Some(0));
    let swift = String::from_utf8(swift.stdout).unwrap();
    let bin = outer.0.join("offline-bin");
    fs::create_dir(&bin).unwrap();
    let wrapper = bin.join("swift");
    fs::write(&wrapper, format!("#!/bin/sh\ncommand=\"$1\"; shift\ncase \"$command\" in package|build) set -- --disable-sandbox \"$@\";; esac\nexec /usr/bin/sandbox-exec -p '(version 1)(allow default)(deny network-outbound (remote ip))(deny network-outbound (remote unix-socket (path-regex #\"^/private/var/run/mDNSResponder\")))' '{}' \"$command\" \"$@\"\n", swift.trim())).unwrap();
    fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).unwrap();
    let mut paths = vec![bin];
    paths.extend(std::env::split_paths(saved_path.0.as_ref().unwrap()));
    std::env::set_var("PATH", std::env::join_paths(paths).unwrap());
}

fn library(path: &std::path::Path, name: &str, dependency: &str) {
    fs::create_dir_all(path.join(format!("Sources/{name}"))).unwrap();
    fs::write(path.join(format!("Sources/{name}/source.swift")), format!("public struct {name} {{}}\n")).unwrap();
    fs::write(path.join("Package.swift"), format!("// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"{name}\", products: [.library(name: \"{name}\", targets: [\"{name}TheTarget\"])], dependencies: [{dependency}], targets: [.target(name: \"{name}TheTarget\", path: \"Sources/{name}\")])\n")).unwrap();
}

fn assert_no_build(path: &std::path::Path) {
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        assert_ne!(entry.file_name(), ".build", "{}", entry.path().display());
        if entry.file_type().unwrap().is_dir() { assert_no_build(&entry.path()); }
    }
}

#[derive(Clone, Copy)]
enum Case { Found, Clean, NoPackage, Remote, NoEngine, RemoteBinary, PackageAboveRepo, NestedRemote, LocalOutsideRepo, LocalCycle }

fn check_case(name: &str, case: Case) {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let saved_path = PathGuard(std::env::var_os("PATH"));
    let outer = TempFolder::new(name);
    let isolated = matches!(case, Case::RemoteBinary | Case::NestedRemote | Case::LocalOutsideRepo | Case::LocalCycle);
    let repo = if matches!(case, Case::PackageAboveRepo) || isolated {
        let path = outer.0.join("repo");
        fs::create_dir(&path).unwrap();
        path
    } else {
        outer.0.clone()
    };
    if isolated { offline_swift(&outer, &saved_path); }
    git_support::git(&repo, &["init", "-q"], b"");
    fs::create_dir_all(repo.join("Sources/Probe")).unwrap();
    let source = if matches!(case, Case::Found) {
        "func unusedHelper() {}\nprint(\"hello\")\n"
    } else {
        "print(\"hello\")\n"
    };
    fs::write(repo.join("Sources/Probe/main.swift"), source).unwrap();
    let manifest = if matches!(case, Case::Remote) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", dependencies: [.package(url: \"https://example.invalid/dep.git\", from: \"1.0.0\")], targets: [.executableTarget(name: \"Probe\")])\n"
    } else if matches!(case, Case::RemoteBinary) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", targets: [.executableTarget(name: \"Probe\", dependencies: [\"Bin\"]), .binaryTarget(name: \"Bin\", url: \"https://example.invalid/Bin.xcframework.zip\", checksum: \"0000000000000000000000000000000000000000000000000000000000000000\")])\n"
    } else if matches!(case, Case::NestedRemote) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", dependencies: [.package(path: \"Inner\")], targets: [.executableTarget(name: \"Probe\")])\n"
    } else if matches!(case, Case::LocalOutsideRepo) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", dependencies: [.package(path: \"../Outside\")], targets: [.executableTarget(name: \"Probe\")])\n"
    } else if matches!(case, Case::LocalCycle) {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", dependencies: [.package(path: \"A\")], targets: [.executableTarget(name: \"Probe\")])\n"
    } else {
        "// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: \"Probe\", targets: [.executableTarget(name: \"Probe\")])\n"
    };
    if matches!(case, Case::PackageAboveRepo) {
        fs::create_dir_all(outer.0.join("Sources/Probe")).unwrap();
        fs::write(outer.0.join("Sources/Probe/main.swift"), source).unwrap();
        fs::write(outer.0.join("Package.swift"), manifest).unwrap();
    } else if !matches!(case, Case::NoPackage) {
        fs::write(repo.join("Package.swift"), manifest).unwrap();
    }
    match case {
        Case::NestedRemote => library(&repo.join("Inner"), "Inner", ".package(url: \"https://example.invalid/dep.git\", from: \"1.0.0\")"),
        Case::LocalOutsideRepo => library(&outer.0.join("Outside"), "Outside", ""),
        Case::LocalCycle => {
            library(&repo.join("A"), "A", ".package(path: \"../B\")");
            library(&repo.join("B"), "B", ".package(path: \"../A\")");
        }
        _ => {}
    }
    fs::write(repo.join("tao.toml"), "version = 1\n[[rule]]\nid = \"unused-code\"\nengine = \"periphery\"\ncheck = \"unused\"\npaths = [\"**\"]\nkind = \"quality\"\nseverity = \"warning\"\nfix = \"Remove unused code.\"\n").unwrap();
    git_support::git(&repo, &["add", "."], b"");
    if matches!(case, Case::NoEngine) {
        let filtered = std::env::join_paths(std::env::split_paths(saved_path.0.as_ref().unwrap()).filter(|dir| {
            !dir.join("periphery").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        })).unwrap();
        std::env::set_var("PATH", filtered);
    }
    let file = RuleFile::read(&repo.join("tao.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo, &[], None).runs;
    if matches!(case, Case::NestedRemote | Case::LocalOutsideRepo | Case::LocalCycle) { assert_no_build(&outer.0); }
    assert_eq!(runs.len(), 1, "{name}: {runs:?}");
    let run = &runs[0];
    assert_eq!(run.rule, "unused-code");
    assert_eq!(run.engine, Engine::Periphery);
    match case {
        Case::Found => {
            let RuleOutcome::Findings(findings) = &run.outcome else { panic!("{name}: {run:?}") };
            assert_eq!(findings.len(), 1, "{findings:?}");
            assert_eq!(findings[0].rule, "unused-code");
            assert_eq!(findings[0].path, RelPath::new("Sources/Probe/main.swift"));
            assert_eq!(findings[0].line(), Some(1));
            assert_eq!(findings[0].col, Some(6));
            assert_eq!(findings[0].fix, "Remove unused code.");
            let mut names = fs::read_dir(&repo).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            names.sort();
            assert_eq!(names, [".git", "Package.swift", "Sources", "tao.toml"].map(OsString::from));
            let sources = fs::read_dir(repo.join("Sources")).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            assert_eq!(sources, [OsString::from("Probe")]);
            let target = fs::read_dir(repo.join("Sources/Probe")).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
            assert_eq!(target, [OsString::from("main.swift")]);
            assert_eq!(fs::read_to_string(repo.join("Package.swift")).unwrap(), manifest);
            assert_eq!(fs::read_to_string(repo.join("Sources/Probe/main.swift")).unwrap(), source);
            let prefix = format!("cctao-periphery-{}-", std::process::id());
            assert!(!fs::read_dir(std::env::temp_dir()).unwrap().any(|e| e.unwrap().file_name().to_string_lossy().starts_with(&prefix)));
        }
        Case::Clean | Case::LocalCycle => assert_eq!(run.outcome, RuleOutcome::Findings(vec![])),
        Case::NoPackage | Case::Remote | Case::NoEngine | Case::RemoteBinary | Case::PackageAboveRepo | Case::NestedRemote | Case::LocalOutsideRepo => {
            let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
            match case {
                Case::NoPackage => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "no Package.swift at or above the rule file's folder: Periphery builds the project, and cctao runs it only on a Swift package");
                }
                Case::Remote => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "the package depends on dep, which cctao would have to fetch: cctao never fetches packages, so Periphery cannot build it here");
                    assert!(!repo.join(".build").exists());
                }
                Case::RemoteBinary => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "the package depends on binary target Bin (https://example.invalid/Bin.xcframework.zip), which cctao would have to fetch: cctao never fetches packages, so Periphery cannot build it here");
                    assert!(!repo.join(".build").exists());
                }
                Case::PackageAboveRepo => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "no Package.swift at or above the rule file's folder: Periphery builds the project, and cctao runs it only on a Swift package");
                    let mut names = fs::read_dir(&outer.0).unwrap().map(|e| e.unwrap().file_name()).collect::<Vec<_>>();
                    names.sort();
                    assert_eq!(names, ["Package.swift", "Sources", "repo"].map(OsString::from));
                    assert_eq!(fs::read_to_string(outer.0.join("Package.swift")).unwrap(), manifest);
                    assert_eq!(fs::read_to_string(outer.0.join("Sources/Probe/main.swift")).unwrap(), source);
                    assert!(!outer.0.join(".build").exists());
                }
                Case::NestedRemote => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, "the local package Inner depends on dep, which cctao would have to fetch: cctao never fetches packages, so Periphery cannot build it here");
                }
                Case::LocalOutsideRepo => {
                    assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other));
                    assert_eq!(why, &format!("the package depends on the local package {}, outside the repository: cctao cannot vouch for it, so Periphery cannot build it here", fs::canonicalize(outer.0.join("Outside")).unwrap().display()));
                }
                Case::NoEngine => assert_eq!(run.not_available_cause, Some(NotAvailableCause::EngineNotOnPath)),
                _ => unreachable!(),
            }
        }
    }
}

macro_rules! cases {
    ($( $name:ident => $case:ident ),+ $(,)?) => {
        $(#[test] fn $name() { check_case(stringify!($name), Case::$case); })+
    };
}

cases! {
    found_unused_function => Found,
    clean_package => Clean,
    no_package_is_not_available => NoPackage,
    remote_dependency_is_not_available => Remote,
    missing_periphery_is_not_available => NoEngine,
    bounds_remote_binary_is_not_available => RemoteBinary,
    bounds_package_above_repo_is_not_available => PackageAboveRepo,
    local_deps_nested_remote_is_not_available => NestedRemote,
    local_deps_outside_repo_is_not_available => LocalOutsideRepo,
    local_deps_cycle_builds_cleanly => LocalCycle,
}

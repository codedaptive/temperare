//! `plan_install`: what `cctao install` will place and wire, from the host and the flags.

use std::path::PathBuf;

use crate::{Harness, InstallArgs, InstallHost, InstallPaths, InstallPlan};

/// The plan for `host`. Placement: none with `--no-place` or when the running binary is under a package-manager
/// prefix (each a note), none when it already is the placed binary; else copy it to `.local/bin/cctao`. Skills: each
/// target's skill file under HOME, or under `host.cwd` with `--location local`. A note when the binary goes into
/// `.local/bin` and that folder is not on `host.path_var` (never on it when PATH is unset).
pub(crate) fn plan_install(args: &InstallArgs, host: &InstallHost, targets: &[Harness]) -> InstallPlan {
    let paths = InstallPaths::under(&host.home);
    let mut notes = Vec::new();
    let place = placement(args, host, &paths, &mut notes);
    let on_path = host.path_var.as_ref().is_some_and(|var| std::env::split_paths(var).any(|dir| dir == paths.bin_dir));
    if place.is_some() && !on_path {
        notes.push(format!("{} is not on PATH; add it to run cctao by name", paths.bin_dir.display()));
    }
    let base = if args.local { &host.cwd } else { &host.home };
    let skills = targets.iter().map(|harness| harness.skill_file(base)).collect();
    InstallPlan { place, skills, notes }
}

/// Where to copy the running binary, or `None` with the reason noted (nothing is noted when it is already there).
fn placement(args: &InstallArgs, host: &InstallHost, paths: &InstallPaths, notes: &mut Vec<String>) -> Option<PathBuf> {
    if args.no_place {
        notes.push("--no-place: cctao not copied".to_string());
        return None;
    }
    if let Some(prefix) = host.prefixes.iter().find(|prefix| host.exe.starts_with(prefix)) {
        notes.push(format!(
            "cctao not copied: {} is under {}, which a package manager owns",
            host.exe.display(),
            prefix.display()
        ));
        return None;
    }
    (host.exe != paths.binary).then(|| paths.binary.clone())
}

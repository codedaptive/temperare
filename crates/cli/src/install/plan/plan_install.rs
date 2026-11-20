//! `plan_install`: what `temperare install` will place, link and wire, from the host and the flags.

use std::path::{Path, PathBuf};

use crate::{Harness, InstallArgs, InstallHost, InstallPaths, InstallPlan};

/// The plan for `host`. Placement: none with `--no-place` or when the running binary is under a package-manager
/// prefix (each a note), none when it already is the placed binary; else copy it to `.local/bin/temperare`. The `tm`
/// link points at the placed binary when there is one, else at the running binary. `tm_owner`, another program
/// that answers to `tm` on PATH: no link, with a note naming it and `--short-name`; with `--short-name`, the link
/// anyway, noted. Skills: each target's skill file under HOME, or under `host.cwd` with `--location local`. A
/// note when something goes into `.local/bin` and that folder is not on `host.path_var` (never on it when PATH is
/// unset).
pub(crate) fn plan_install(
    args: &InstallArgs,
    host: &InstallHost,
    targets: &[Harness],
    tm_owner: Option<&Path>,
) -> InstallPlan {
    let paths = InstallPaths::under(&host.home);
    let mut notes = Vec::new();
    let place = placement(args, host, &paths, &mut notes);
    let target = if place.is_some() || host.exe == paths.binary { paths.binary.clone() } else { host.exe.clone() };
    let link = match tm_owner {
        Some(owner) if !args.short_name => {
            notes.push(format!(
                "tm not linked: {} is already tm on PATH; run temperare install --short-name to link it anyway",
                owner.display()
            ));
            None
        }
        Some(owner) => {
            notes.push(format!("tm linked with --short-name, though {} is also tm on PATH", owner.display()));
            Some((paths.link.clone(), target))
        }
        None => Some((paths.link.clone(), target)),
    };
    let on_path = host.path_var.as_ref().is_some_and(|var| std::env::split_paths(var).any(|dir| dir == paths.bin_dir));
    if (place.is_some() || link.is_some()) && !on_path {
        notes.push(format!("{} is not on PATH; add it to run temperare and tm by name", paths.bin_dir.display()));
    }
    let base = if args.local { &host.cwd } else { &host.home };
    let skills = targets.iter().map(|harness| harness.skill_file(base)).collect();
    InstallPlan { place, link, skills, notes }
}

/// Where to copy the running binary, or `None` with the reason noted (nothing is noted when it is already there).
fn placement(args: &InstallArgs, host: &InstallHost, paths: &InstallPaths, notes: &mut Vec<String>) -> Option<PathBuf> {
    if args.no_place {
        notes.push(format!("--no-place: temperare not copied; tm points at {}", host.exe.display()));
        return None;
    }
    if let Some(prefix) = host.prefixes.iter().find(|prefix| host.exe.starts_with(prefix)) {
        notes.push(format!(
            "temperare not copied: {} is under {}, which a package manager owns",
            host.exe.display(),
            prefix.display()
        ));
        return None;
    }
    (host.exe != paths.binary).then(|| paths.binary.clone())
}

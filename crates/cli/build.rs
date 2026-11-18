//! cli's build script: compiles the commit into the binary as `TEMPERARE_COMMIT`, for `temperare --version`
//! (S-32, S-33), and whether it was built in a git checkout as `TEMPERARE_CHECKOUT` (`yes` or `no`), which the
//! version tests read. git runs through procio (S-26); this file only reads the `.git` file's `gitdir:` line and the
//! git folder's `commondir`, to tell cargo which paths change when the commit does.

use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    println!("cargo:rustc-env=TEMPERARE_COMMIT={}", commit(&manifest));
    let dot_git = manifest.ancestors().map(|dir| dir.join(".git")).find(|p| p.exists());
    println!("cargo:rustc-env=TEMPERARE_CHECKOUT={}", if dot_git.is_some() { "yes" } else { "no" });
    println!("cargo:rerun-if-changed=build.rs");
    for file in dot_git.map(|dot_git| watched(&dot_git)).unwrap_or_default() {
        println!("cargo:rerun-if-changed={}", file.display());
    }
}

/// The first 12 hex characters of the commit HEAD names, or `unknown` when git cannot say (no git, no repository,
/// no commit yet).
fn commit(manifest: &Path) -> String {
    match procio::rev_parse(manifest, "HEAD") {
        Ok(Some(oid)) => oid.as_str().chars().take(12).collect(),
        _ => "unknown".to_string(),
    }
}

/// The paths whose change means HEAD may name another commit, those that exist, for the first `.git` above the
/// manifest. Its git folder is `.git` itself in a main checkout, or, when `.git` is a file (a linked worktree,
/// `gitdir: <path>`), the folder it names. The common folder is the one the git folder's `commondir` file names
/// (relative to the git folder; a linked worktree has one), else the git folder itself. Watched: the git folder's
/// HEAD (a checkout of another branch or commit), the common folder's packed-refs (a branch held only there) and the
/// common folder's `refs` folder, which cargo scans for any change, so a commit that writes or moves a loose ref,
/// including the first loose ref of a branch held only in packed-refs, reruns the script. A path cargo is told to
/// watch that does not exist makes it rerun the script on every build, so only existing paths are named.
fn watched(dot_git: &Path) -> Vec<PathBuf> {
    let Some(git_dir) = git_dir(dot_git) else {
        return Vec::new();
    };
    let common = match fs::read_to_string(git_dir.join("commondir")) {
        Ok(text) => git_dir.join(text.trim()),
        Err(_) => git_dir.clone(),
    };
    [git_dir.join("HEAD"), common.join("packed-refs"), common.join("refs")]
        .into_iter()
        .filter(|path| path.exists())
        .collect()
}

/// `dot_git` when it is a folder; else the gitdir its `gitdir: <path>` line names, a relative one read from the
/// `.git` file's folder.
fn git_dir(dot_git: &Path) -> Option<PathBuf> {
    if dot_git.is_dir() {
        return Some(dot_git.to_path_buf());
    }
    let text = fs::read_to_string(dot_git).ok()?;
    let gitdir = text.trim().strip_prefix("gitdir: ")?;
    Some(dot_git.parent()?.join(gitdir))
}

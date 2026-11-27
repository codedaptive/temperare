//! The environments acceptance runs programs in, built from nothing (no tool-specific variable set).
//! `clean_env` (git and the PATH lookup) passes only HOME, PATH, TMPDIR, LANG and CARGO_NET_OFFLINE; PATH is the
//! caller's PATH with ~/.cargo/bin moved to the front. `cctao_env` (`cctao` and the report head's version lookups)
//! is the same with HOME a folder the test made, plus CARGO_HOME and RUSTUP_HOME.

use std::path::Path;

/// The variables for one run. `path` replaces PATH whole (the absent-engine case runs with a PATH that lacks an engine).
pub fn clean_env(path: Option<&str>) -> Vec<(String, String)> {
    let home = std::env::var("HOME").expect("HOME is set");
    let path = match path {
        Some(p) => p.to_string(),
        None => default_path(&home),
    };
    let mut env = vec![
        ("HOME".to_string(), home),
        ("PATH".to_string(), path),
        ("LANG".to_string(), "C".to_string()),
        // The throwaway crates have no dependencies, so clippy's cargo needs no network; offline, a fetch fails.
        ("CARGO_NET_OFFLINE".to_string(), "true".to_string()),
    ];
    if let Ok(tmp) = std::env::var("TMPDIR") {
        env.push(("TMPDIR".to_string(), tmp));
    }
    env
}

/// The variables for one run of `cctao`: `clean_env(path)` with HOME replaced by `home`, so the run reads no real
/// ~/.config/cctao/config.toml and appends to no real ~/.local/state/cctao/savings.jsonl and no real telemetry
/// file under ~/.local/state/cctao/telemetry. PATH still comes from the real HOME (its ~/.cargo/bin first), and
/// CARGO_HOME and RUSTUP_HOME are pinned to the real ones (the variable if set, else ~/.cargo and ~/.rustup of the real
/// HOME), so rustup's cargo and clippy still find their toolchain. Git run
/// by `cctao` here reads no real ~/.gitconfig; `clean_env` keeps the real HOME for the fixture's own commits.
pub fn cctao_env(path: Option<&str>, home: &Path) -> Vec<(String, String)> {
    let real = std::env::var("HOME").expect("HOME is set");
    let pinned = |var: &str, dir: &str| std::env::var(var).unwrap_or_else(|_| format!("{real}/{dir}"));
    let cargo_home = pinned("CARGO_HOME", ".cargo");
    let rustup_home = pinned("RUSTUP_HOME", ".rustup");
    let mut env: Vec<(String, String)> = clean_env(path).into_iter().filter(|(k, _)| k != "HOME").collect();
    env.push(("HOME".to_string(), home.display().to_string()));
    env.push(("CARGO_HOME".to_string(), cargo_home));
    env.push(("RUSTUP_HOME".to_string(), rustup_home));
    env
}

fn default_path(home: &str) -> String {
    let cargo_bin = format!("{home}/.cargo/bin");
    let mut dirs: Vec<String> = std::env::var("PATH")
        .unwrap_or_default()
        .split(':')
        .filter(|d| !d.is_empty())
        .map(str::to_string)
        .collect();
    // Other folders on PATH can hold a `cargo` wrapper; the real cargo goes first so clippy's build does not queue
    // behind the test run that started it.
    dirs.retain(|d| d != &cargo_bin);
    dirs.insert(0, cargo_bin);
    dirs.join(":")
}

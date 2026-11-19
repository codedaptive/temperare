//! The environment `temperare`, git and the report head's version lookups run in for acceptance, built from nothing
//! (Done 6: no tool-specific variable set).
//! Only HOME, PATH, TMPDIR, LANG and CARGO_NET_OFFLINE are passed. PATH is the caller's PATH with ~/.cargo/bin moved
//! to the front.

/// The variables for one run. `path` replaces PATH whole (Done 4 runs with a PATH that lacks an engine).
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

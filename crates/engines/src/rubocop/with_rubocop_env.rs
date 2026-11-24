//! `with_rubocop_env`: a rubocop command given the environment that keeps it to tm's options and off the network.

use procio::Cmd;
use std::ffi::OsString;

/// The proxy every web fetch is sent through: port 9 on the loopback, where nothing listens, so a fetch rubocop
/// tries (a config's `inherit_from: https://...`, which it reads with Net::HTTP, and Net::HTTP honours these
/// variables) fails at once with "Connection refused" and nothing leaves the machine.
const DEAD_PROXY: &str = "http://127.0.0.1:9";

/// `cmd` with RUBOCOP_OPTS set empty (rubocop reads that variable as extra command-line options, which could add
/// `--except`, `--out` or `--display-only-correctable` and change what it prints; empty means none),
/// http_proxy, https_proxy, HTTP_PROXY and HTTPS_PROXY set to `DEAD_PROXY`, and no_proxy and NO_PROXY set empty, so
/// no host the caller's environment lists skips that proxy. The variables are added after `cmd`'s own.
pub(crate) fn with_rubocop_env(mut cmd: Cmd) -> Cmd {
    let vars = [
        ("RUBOCOP_OPTS", ""),
        ("http_proxy", DEAD_PROXY),
        ("https_proxy", DEAD_PROXY),
        ("HTTP_PROXY", DEAD_PROXY),
        ("HTTPS_PROXY", DEAD_PROXY),
        ("no_proxy", ""),
        ("NO_PROXY", ""),
    ];
    cmd.env.extend(vars.map(|(name, value)| (OsString::from(name), OsString::from(value))));
    cmd
}

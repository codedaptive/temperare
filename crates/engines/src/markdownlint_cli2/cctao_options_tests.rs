//! Unit test for cctao_options.rs: the module's name is one a `--config` options module may have, and its text sets each option
//! the contract names (inline config off, no fix, nothing but the results on stdout, the JSON formatter in two
//! brackets). The run itself is proven by the ZIT, tests/markdownlint_cli2.rs.

use super::cctao_options;

#[test]
fn cctao_options_names_and_sets_the_module() {
    let (file, text) = cctao_options();
    assert!(file.ends_with(".markdownlint-cli2.cjs") && !file.contains('/'), "{file}");
    let settings = [
        "noInlineConfig: true",
        "fix: false",
        "noBanner: true",
        "noProgress: true",
        "outputFormatters: [[ (o) => { o.logMessage(JSON.stringify(o.results)); } ]]",
    ];
    for setting in settings {
        assert!(text.contains(setting), "{setting}");
    }
}

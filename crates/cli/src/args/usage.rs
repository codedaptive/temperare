//! `USAGE`: the text `tm help` prints.

/// tm's usage: one line per command form, then where the rule file comes from, and that `temperare` is the program
/// while `tm` answers only where `temperare install` made the `tm` link, which install does not always make.
pub const USAGE: &str = "\
usage:
tm check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]
tm rules [--rules <path>] [--json]
tm install [--no-place] [--short-name] [--target claude,codex,cursor] [--mode skill] [--location local] [-y]
tm uninstall
tm help
tm --version

The rule file is --rules <path>, else temperare.toml at the top of the git repository.
temperare is the program; tm answers the same commands only where temperare install made the tm link.
";

//! `InstallArgs`: the flags of `temperare install`.

use crate::Harness;

/// `install [--no-place] [--short-name] [--target <list>] [--mode skill] [--location local] [-y]`: `no_place`
/// copies no binary; `short_name` links `tm` even when another tm is on PATH; `targets` are the harnesses
/// `--target` names (`None` when not given); `local` wires the project's folders instead of HOME's; `yes` wires
/// every detected harness without asking. `--mode skill` is the only mode, so it has no field.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallArgs {
    pub no_place: bool,
    pub short_name: bool,
    pub targets: Option<Vec<Harness>>,
    pub local: bool,
    pub yes: bool,
}

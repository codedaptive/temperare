//! `InstallArgs`: the flags of `cctao install`.

use crate::Harness;

/// `install [--no-place] [--target <list>] [--mode skill] [--location local] [-y]`: `no_place` copies no binary;
/// `targets` are the harnesses `--target` names (`None` when not given); `local` wires the project's folders instead
/// of HOME's; `yes` wires every detected harness without asking. `--mode skill` is the only mode, so it has no field.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct InstallArgs {
    pub no_place: bool,
    pub targets: Option<Vec<Harness>>,
    pub local: bool,
    pub yes: bool,
}

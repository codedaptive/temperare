//! `choose_targets`: which detected harnesses install wires, and a note for each one it does not.

use crate::{Harness, InstallArgs};

/// The harnesses to wire and the notes to print. `--target` given: those of them `detected`, in order and deduped;
/// each one not detected is a note and is not wired. No `--target`: nothing detected is a note naming the folders
/// looked for; `-y` is every detected harness; at a terminal, what `pick` returns from the detected list; else none,
/// with a note saying the command that wires them. Wiring is offered, never required: no path here is an error.
pub(crate) fn choose_targets(
    args: &InstallArgs,
    detected: &[Harness],
    interactive: bool,
    pick: &mut dyn FnMut(&[Harness]) -> Vec<Harness>,
) -> (Vec<Harness>, Vec<String>) {
    if let Some(targets) = &args.targets {
        return named_targets(targets, detected);
    }
    if detected.is_empty() {
        let folders: Vec<&str> = Harness::ALL.iter().map(|harness| harness.folder()).collect();
        let note = format!("no harness found (looked for {} in HOME); no skill wired", folders.join(", "));
        return (Vec::new(), vec![note]);
    }
    if args.yes {
        return (detected.to_vec(), Vec::new());
    }
    if interactive {
        return (pick(detected), Vec::new());
    }
    let names: Vec<&str> = detected.iter().map(|harness| harness.name()).collect();
    let note = format!(
        "no terminal to ask which harnesses to wire, so none was; to wire them, run cctao install --target {} -y",
        names.join(",")
    );
    (Vec::new(), vec![note])
}

/// The `--target` harnesses that are detected, in order and deduped, and a note for each that is not.
fn named_targets(targets: &[Harness], detected: &[Harness]) -> (Vec<Harness>, Vec<String>) {
    let mut chosen = Vec::new();
    let mut notes = Vec::new();
    for &harness in targets {
        if !detected.contains(&harness) {
            let note = format!("{} not found (no {} folder in HOME); not wired", harness.name(), harness.folder());
            if !notes.contains(&note) {
                notes.push(note);
            }
        } else if !chosen.contains(&harness) {
            chosen.push(harness);
        }
    }
    (chosen, notes)
}

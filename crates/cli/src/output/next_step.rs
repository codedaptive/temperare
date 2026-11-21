//! `next_step`: the text after `next: ` for an exit code.

use super::serde_name::serde_name;
use engines::NotAvailableCause;
use report::NotAvailable;
use rules::Engine;
use std::path::Path;

/// Tessera's checks never run yet (S-20): every tessera entry is not available whatever its why, so exit 3 never
/// tells the user to install it.
const TESSERA_NOT_YET: &str = "Tessera's checks are not available yet";

/// What the user does next for each exit code: 0 has nothing to do; 1 fixes the findings or adds an exception to
/// `rule_file`; 2 gives the error's fix ("see the error above" when there is none); 3 says what to do about each
/// entry of `not_available`, each paired with its rule run's cause (see `not_available_step`); 4 raises the
/// security findings; 5 reads the engine output; 6 fixes the paths of each rule that covers nothing, or removes it
/// from `rule_file`: as text it points at those rules above, and when `json` (the text is the --json `next`, which
/// has no output above) it names the `covers_nothing` field instead; any other code points at the output above. Each
/// argument is read only for the code that uses it: `json` only for 6.
pub fn next_step(
    exit: u8,
    fix: Option<&str>,
    rule_file: &Path,
    not_available: &[(&NotAvailable, Option<NotAvailableCause>)],
    json: bool,
) -> String {
    match exit {
        0 => "nothing to do".to_string(),
        1 => format!("fix the findings above, or add an approved [[exception]] to {}", rule_file.display()),
        2 => fix.unwrap_or("see the error above").to_string(),
        3 => not_available_step(not_available),
        4 => "raise the security findings with their owner; work goes on".to_string(),
        5 => "see the engine output above".to_string(),
        6 if json => format!(
            "fix the paths of each rule in covers_nothing, or remove it from {}",
            rule_file.display()
        ),
        6 => format!(
            "fix the paths of each rule that covers nothing above, or remove it from {}",
            rule_file.display()
        ),
        _ => "see the output above".to_string(),
    }
}

/// Exit 3's text: the engines whose cause is `EngineNotOnPath`, to install, then each other entry's cause text once,
/// then Tessera's checks not being available yet, joined with "; ". Only Tessera, or no entries at all, keep their own
/// wording.
fn not_available_step(not_available: &[(&NotAvailable, Option<NotAvailableCause>)]) -> String {
    if not_available.is_empty() {
        return "install the engines that are not available (or run without those rules)".to_string();
    }
    let (parts, tessera) = parts(not_available);
    if parts.is_empty() {
        return format!("{TESSERA_NOT_YET} (run without those rules)");
    }
    let tail = if tessera { vec![TESSERA_NOT_YET.to_string()] } else { vec![] };
    format!("{} (or run without those rules)", [parts, tail].concat().join("; "))
}

/// Groups the entries, keeping their order: the install part (each engine name once, for the entries whose cause is
/// `EngineNotOnPath`) followed by the other entries' cause texts (each once, a missing helper such as cargo or an entry
/// with no cause included; an entry whose cause is `NoCargoProject` gets the Cargo text), and whether any entry is
/// Tessera's, whatever its cause.
fn parts(not_available: &[(&NotAvailable, Option<NotAvailableCause>)]) -> (Vec<String>, bool) {
    let (mut install, mut causes, mut tessera) = (Vec::new(), Vec::new(), false);
    for &(entry, entry_cause) in not_available {
        let name = serde_name(serde_json::to_value(entry.engine));
        if entry.engine == Engine::Tessera {
            tessera = true;
        } else if entry_cause == Some(NotAvailableCause::EngineNotOnPath) {
            push_once(&mut install, name);
        } else {
            push_once(&mut causes, cause(&name, &entry.why, entry_cause));
        }
    }
    if !install.is_empty() {
        causes.insert(0, format!("install {}", install.join(", ")));
    }
    (causes, tessera)
}

/// The cause text for an engine `name` and its `why`; when `entry_cause` is `NoCargoProject` it also says where the
/// rule file goes.
fn cause(name: &str, why: &str, entry_cause: Option<NotAvailableCause>) -> String {
    if entry_cause == Some(NotAvailableCause::NoCargoProject) {
        return format!("{name}: {why}, so put the rule file inside the Cargo project it checks");
    }
    format!("{name}: {why}")
}

/// Adds `item` to `list` unless it is already there, keeping first-seen order.
fn push_once(list: &mut Vec<String>, item: String) {
    if !list.contains(&item) {
        list.push(item);
    }
}

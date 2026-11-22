//! `unknown_message`: pylint's words saying the rule's check is no message it knows.

use procio::Output;
use serde::Deserialize;

/// The ids of pylint's messages about an option value: `unknown-option-value` (W0012, "Unknown option value for
/// '--enable', expected a valid pylint message and got 'ZZ9999'") and `useless-option-value` (R0022, "Useless option
/// value for '--enable', 'C0330' was removed from pylint, ...").
const OPTION_VALUE_IDS: [&str; 2] = ["W0012", "R0022"];

/// `pylint --output-format=json2`'s one object, read only for its messages.
#[derive(Deserialize)]
struct Report {
    messages: Vec<Message>,
}

/// One json2 message, read only for its id and text.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    message_id: String,
    message: String,
}

/// The text of the first json2 message in `out`'s stdout whose id is W0012 or R0022 and whose text contains
/// `'<check>'`, single-quoted as pylint quotes the value; `None` when stdout is not json2 or no message does. A
/// W0012 naming another value (one in the rule's rcfile, say) is not about the check, so it is left to the reader.
pub(crate) fn unknown_message(out: &Output, check: &str) -> Option<String> {
    let report: Report = serde_json::from_slice(&out.stdout).ok()?;
    let quoted = format!("'{check}'");
    report
        .messages
        .into_iter()
        .find(|message| OPTION_VALUE_IDS.contains(&message.message_id.as_str()) && message.message.contains(&quoted))
        .map(|message| message.message)
}

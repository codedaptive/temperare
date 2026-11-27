//! `error_lines`: the error lines of gitleaks' stderr, the only part of it cctao passes on.

/// Of gitleaks' `stderr` (lossy UTF-8, ANSI escape sequences removed first), the text of each line whose first
/// whitespace-separated word is gitleaks' log time ("9:37PM") and whose second is the level ERR or FTL (gitleaks logs
/// "<time> <LEVEL> <text>"), trimmed, joined with "\n"; empty when there is none. INF, WRN, DBG and TRC lines, and any
/// line not in that shape (gitleaks' verbose finding blocks, which hold the match), are dropped. A run's stderr can
/// reach the report as a Failed rule's stderr, where gitleaks' `--redact` would be the only guard; keeping error lines
/// alone leaves a config or rule error readable while a finding block does not ride stderr.
pub(crate) fn error_lines(stderr: &[u8]) -> Vec<u8> {
    without_escapes(&String::from_utf8_lossy(stderr))
        .lines()
        .filter_map(error_text)
        .collect::<Vec<_>>()
        .join("\n")
        .into_bytes()
}

/// The text of an ERR or FTL log line; None for any other line.
fn error_text(line: &str) -> Option<&str> {
    let (time, rest) = line.trim_start().split_once(char::is_whitespace)?;
    let (level, text) = rest.trim_start().split_once(char::is_whitespace)?;
    (is_log_time(time) && matches!(level, "ERR" | "FTL")).then(|| text.trim())
}

/// Whether `word` is gitleaks' log time: one or two ASCII digits, ':', two ASCII digits, then AM or PM ("9:37PM").
fn is_log_time(word: &str) -> bool {
    let Some(clock) = word.strip_suffix("AM").or_else(|| word.strip_suffix("PM")) else {
        return false;
    };
    let Some((hours, minutes)) = clock.split_once(':') else {
        return false;
    };
    let digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    (1..=2).contains(&hours.len()) && minutes.len() == 2 && digits(hours) && digits(minutes)
}

/// `text` without its ANSI escape sequences: ESC '[' up to and including the final letter.
fn without_escapes(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' && chars.clone().next() == Some('[') {
            chars.by_ref().find(char::is_ascii_alphabetic);
        } else {
            out.push(c);
        }
    }
    out
}

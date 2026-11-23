//! `tm_lines`: what one `temperare` run showed, as lines for the `make accept` report. Every value comes from `temperare`'s
//! `--json` answer: a finding's place, rule, engine, kind, severity and owner are written bare, every other string
//! as a JSON string, and `counts` as its JSON object.

use serde_json::Value;

/// `tm <args>`, its exit code and counts; the base, when there is one, and the exceptions in force; then one line for
/// each kept finding, silenced finding, rule not available and rule that failed. A usage error answers with no
/// report, only its `error` and `next`, and is shown as those.
pub fn tm_lines(args: &[&str], code: i32, json: &Value) -> Vec<String> {
    let run = format!("tm {} -> exit {code}", args.join(" "));
    if json.get("error").is_some() {
        return vec![format!("{run}; error {}; next {}", json["error"], json["next"])];
    }
    let mut lines = vec![format!("{run}; counts {}", json["counts"])];
    if let Some(base) = json["base"].as_str() {
        lines.push(format!("base {base}"));
    }
    lines.push(format!("exceptions in force {}", json["exceptions_in_force"]));
    for f in list(json, "findings") {
        lines.push(format!("finding {}", finding(f)));
    }
    for s in list(json, "silenced") {
        let exception = &s["exception"];
        lines.push(format!(
            "silenced {} by the exception for {} (reason {}, approved {})",
            finding(&s["finding"]),
            exception["path"],
            exception["reason"],
            exception["approved"]
        ));
    }
    for n in list(json, "not_available") {
        lines.push(format!("not available {} ({}): {}", n["rule"], n["engine"], n["why"]));
    }
    for f in list(json, "failed") {
        lines.push(format!("failed {} ({}), code {}: {}", f["rule"], f["engine"], f["code"], f["stderr_tail"]));
    }
    lines
}

/// The entries of the list `key`; none when the answer has no such list.
fn list<'a>(json: &'a Value, key: &str) -> &'a [Value] {
    json[key].as_array().map_or(&[], Vec::as_slice)
}

/// `path[:line[:col]] rule engine kind severity [owner o] what "…" fix "…"`; a finding with no line shows its path
/// alone, since a column without a line means nothing.
fn finding(f: &Value) -> String {
    let place = match f["line"].as_u64() {
        Some(line) => {
            let col = f["col"].as_u64().map(|col| format!(":{col}")).unwrap_or_default();
            format!("{}:{line}{col}", text(f, "path"))
        }
        None => text(f, "path").to_string(),
    };
    let owner = f["owner"].as_str().map_or(String::new(), |o| format!(" owner {o}"));
    format!(
        "{place} {} {} {} {}{owner} what {} fix {}",
        text(f, "rule"),
        text(f, "engine"),
        text(f, "kind"),
        text(f, "severity"),
        f["what"],
        f["fix"]
    )
}

/// A string field as written, or `?` when it is missing.
fn text<'a>(f: &'a Value, key: &str) -> &'a str {
    f[key].as_str().unwrap_or("?")
}

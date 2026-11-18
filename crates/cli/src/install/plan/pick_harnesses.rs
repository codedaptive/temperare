//! `pick_harnesses`: the interactive picker, asking which detected harnesses to wire.

use std::io::{BufRead, Write};

use crate::Harness;

/// Writes the detected harnesses to `out`, numbered from 1, and reads one answer line from `input`. Empty (or end
/// of input, or a read error) is none; `all` is every detected harness; else commas between numbers or names of
/// detected harnesses, each kept once in the order given. Any token that is neither is none, with a line on `out`
/// naming it: a wrong answer wires nothing rather than a guess. Write errors are ignored, as everywhere in tm.
pub(crate) fn pick_harnesses(detected: &[Harness], input: &mut dyn BufRead, out: &mut dyn Write) -> Vec<Harness> {
    let _ = writeln!(out, "Wire the temperare skill into which harnesses?");
    for (number, harness) in detected.iter().enumerate() {
        let _ = writeln!(out, "  {}. {}", number + 1, harness.name());
    }
    let _ = write!(out, "Numbers or names, separated by commas; all; or Enter for none: ");
    let _ = out.flush();
    let mut answer = String::new();
    if input.read_line(&mut answer).is_err() {
        return Vec::new();
    }
    let answer = answer.trim();
    if answer.is_empty() {
        return Vec::new();
    }
    if answer == "all" {
        return detected.to_vec();
    }
    let mut chosen = Vec::new();
    for token in answer.split(',').map(str::trim) {
        let Some(harness) = choice(detected, token) else {
            let _ = writeln!(out, "'{token}' is not one of the choices; no harness wired");
            return Vec::new();
        };
        if !chosen.contains(&harness) {
            chosen.push(harness);
        }
    }
    chosen
}

/// The detected harness `token` names, by its number from 1 or by its name.
fn choice(detected: &[Harness], token: &str) -> Option<Harness> {
    match token.parse::<usize>() {
        Ok(number) => detected.get(number.checked_sub(1)?).copied(),
        Err(_) => Harness::from_name(token).filter(|harness| detected.contains(harness)),
    }
}

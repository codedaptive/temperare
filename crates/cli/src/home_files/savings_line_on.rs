//! `savings_line_on`: whether cctao check prints its savings line, from the user's config file under HOME.

use super::parse_user_config;
use std::io::ErrorKind;
use std::path::Path;

/// Reads `<home>/.config/cctao/config.toml` and returns its `savings_line`. True with nothing written when there
/// is no HOME or no file; true with one line `cctao: ignoring ~/.config/cctao/config.toml: <why>` on `stderr` when
/// the file cannot be read or parsed. Never writes the config file; a failed `stderr` write is ignored.
pub fn savings_line_on(home: Option<&Path>, stderr: &mut dyn std::io::Write) -> bool {
    let Some(home) = home.filter(|home| !home.as_os_str().is_empty()) else {
        return true;
    };
    let why = match std::fs::read(home.join(".config/cctao/config.toml")) {
        Err(error) if error.kind() == ErrorKind::NotFound => return true,
        Err(error) => error.to_string(),
        Ok(bytes) => match String::from_utf8(bytes) {
            Err(error) => error.to_string(),
            Ok(text) => match parse_user_config(&text) {
                Ok(on) => return on,
                Err(why) => why,
            },
        },
    };
    let _ = stderr.write_all(format!("cctao: ignoring ~/.config/cctao/config.toml: {why}\n").as_bytes());
    true
}

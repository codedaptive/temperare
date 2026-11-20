//! `is_shell_shebang`: whether a file's first bytes start with a shebang that runs a shell shellcheck checks.

/// The longest first line read, in bytes before its `\n`.
const MAX_LINE: usize = 256;

/// The shells shellcheck checks, as the last part of the interpreter's path.
const SHELLS: [&[u8]; 4] = [b"sh", b"bash", b"dash", b"ksh"];

/// env's flags whose value is the next word: unset a variable, change directory, search a path.
const ENV_VALUE_FLAGS: [&[u8]; 3] = [b"-u", b"-C", b"-P"];

/// Whether `head`'s first line is a shell shebang. The first line is the bytes before the first `\n`, or all of
/// `head` when it has none; a line over 256 bytes is not read as one, and a trailing `\r` is dropped. The line must
/// start with `#!`; its words (split on ASCII whitespace, so spaces after `#!` are allowed) must be an interpreter
/// whose part after the last `/` is `sh`, `bash`, `dash` or `ksh`, or `env` followed by such an interpreter. Between
/// `env` and the interpreter, the words starting with `-` are env's flags and skipped; `-u`, `-C` and `-P` take their
/// value as the next word (`-P` in BSD env only), so that word is skipped too (`env -u NAME bash` is bash). `-S` takes
/// the command itself as its value, so `env -S bash -e` reads on to bash, and `--unset=NAME` is one word. Any other
/// shell (zsh, fish) is false.
pub(crate) fn is_shell_shebang(head: &[u8]) -> bool {
    let line = head.split(|&b| b == b'\n').next().unwrap_or(head);
    if line.len() > MAX_LINE {
        return false;
    }
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    let Some(rest) = line.strip_prefix(b"#!") else {
        return false;
    };
    let mut words = rest.split(u8::is_ascii_whitespace).filter(|word| !word.is_empty());
    let Some(first) = words.next() else {
        return false;
    };
    if is_shell(first) {
        return true;
    }
    last_part(first) == b"env" && env_command(words).is_some_and(is_shell)
}

/// The first word after env's flags and the values of the flags that take one.
fn env_command<'a>(mut words: impl Iterator<Item = &'a [u8]>) -> Option<&'a [u8]> {
    while let Some(word) = words.next() {
        if !word.starts_with(b"-") {
            return Some(word);
        }
        if ENV_VALUE_FLAGS.contains(&word) {
            words.next();
        }
    }
    None
}

fn is_shell(word: &[u8]) -> bool {
    SHELLS.contains(&last_part(word))
}

fn last_part(word: &[u8]) -> &[u8] {
    word.rsplit(|&b| b == b'/').next().unwrap_or(word)
}

//! `Harness`: an AI coding harness `temperare install` can wire its skill into.

/// A harness whose user folder can hold the temperare skill. Every harness is `Harness::ALL`; its `--target` name
/// is `name` (and `from_name` back); its folder and skill file are `folder` and `skill_file`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Harness {
    Claude,
    Codex,
    Cursor,
}

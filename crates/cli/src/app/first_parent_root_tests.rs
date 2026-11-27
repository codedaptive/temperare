use std::cell::Cell;

use super::first_parent_root::first_parent_root;

/// A first-parent line whose oldest commit is `HEAD~depth`: `HEAD~n` answers `n` up to it and nothing past it,
/// except that `fail_at` answers a git failure. `None` for depth is an unborn HEAD.
fn line(depth: Option<u64>, fail_at: Option<u64>, asked: &Cell<u32>) -> impl Fn(u64) -> Result<Option<u64>, ()> + '_ {
    move |n| {
        asked.set(asked.get() + 1);
        if fail_at == Some(n) {
            return Err(());
        }
        Ok(depth.filter(|d| n <= *d).map(|_| n))
    }
}

#[test]
fn first_parent_root_finds_the_last_revision_that_resolves() {
    // (name, depth, fail_at, expected (head, root)); 221 is the depth of a line of 222 commits (SPEC-cli WO-cli-5 works
    // the search count for that depth).
    let rows: [(&str, Option<u64>, Option<u64>, Option<(u64, u64)>); 9] = [
        ("depth 0, only HEAD", Some(0), None, Some((0, 0))),
        ("depth 1", Some(1), None, Some((0, 1))),
        ("depth 2", Some(2), None, Some((0, 2))),
        ("depth 3", Some(3), None, Some((0, 3))),
        ("depth 8, a power of two", Some(8), None, Some((0, 8))),
        ("depth 9, one past it", Some(9), None, Some((0, 9))),
        ("depth 221, a line of 222 commits", Some(221), None, Some((0, 221))),
        ("HEAD names no commit", None, None, None),
        ("git fails at HEAD~4", Some(221), Some(4), None),
    ];
    let mut failures = Vec::new();
    for (name, depth, fail_at, expected) in rows {
        let asked = Cell::new(0);
        let got = first_parent_root(&line(depth, fail_at, &asked));
        if got != expected {
            failures.push(format!("{name}: got {got:?}, expected {expected:?}"));
        }
    }
    let asked = Cell::new(0);
    first_parent_root(&line(Some(221), None, &asked));
    if asked.get() != 17 {
        failures.push(format!("depth 221: asked {} questions, expected 17", asked.get()));
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

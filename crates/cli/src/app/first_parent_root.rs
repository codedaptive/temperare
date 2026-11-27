//! `first_parent_root`: HEAD and the root of its first-parent line, found by doubling then halving `HEAD~n`.

/// `at(n)` answers what `HEAD~n` names: `Ok(None)` when it names no commit, `Err` when git failed. Returns HEAD's
/// answer and the answer for the largest `n` that resolves, in about 2 log2(D) + 1 questions for D first-parent
/// commits. None when HEAD names no commit (an unborn HEAD) or any question fails: a root is never guessed. A line
/// so long that doubling `n` would pass `u64::MAX` also gives None, since no git history reaches it.
pub(crate) fn first_parent_root<T: Clone, E>(at: &dyn Fn(u64) -> Result<Option<T>, E>) -> Option<(T, T)> {
    let head = at(0).ok()??;
    let mut lo = (0, head.clone());
    let mut hi = 1u64;
    while let Some(found) = at(hi).ok()? {
        lo = (hi, found);
        hi = hi.checked_mul(2)?;
    }
    while hi - lo.0 > 1 {
        let mid = lo.0 + (hi - lo.0) / 2;
        match at(mid).ok()? {
            Some(found) => lo = (mid, found),
            None => hi = mid,
        }
    }
    Some((head, lo.1))
}

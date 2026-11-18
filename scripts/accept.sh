#!/bin/sh
# accept.sh REPORT: `make accept` (mission, Done; S-31, SPEC-accept). Runs the validation target
# crates/cli/tests/validation through test_one.sh, as before, so the tests print on the terminal as they did; one line
# naming the report follows. Each acceptance case writes its part into a temporary folder this script names in
# ACCEPT_PARTS (crates/cli/tests/validation: check_new.rs names the part, check_drop.rs writes it). Then it writes
# the markdown report to REPORT: a head (commit, date, the test run's exit code, then the tm and engine versions the
# cases saw), one section per check, and a last line of totals. Every case that crates/cli/tests/validation/checks.txt
# lists is in the report, in its order: with its part, or NOT RUN when it wrote none (a filtered run, a test target
# that did not build, a part that could not be written) or when its part was cut short before a whole Result line.
# The script exits with the test run's exit code when it failed; else 3 when any check is NOT RUN, said on stderr,
# since a check not run has not passed (S-34); else 0. When the report cannot be written it exits non-zero with the
# reason on stderr.
# The commit is HEAD when the current folder is the top of a git checkout, noted when a file there differs from it,
# a deleted one included. In a jj folder jj answers, comparing only the files the folder holds, so a file a sparse
# request folder never held is not a change; elsewhere git diff does. ACCEPT_COMMIT, when set, names it instead.
set -u
report=$1
here=$(dirname "$0")
list=crates/cli/tests/validation/checks.txt
parts=$(mktemp -d "${TMPDIR:-/tmp}/tm-accept-parts.XXXXXX") || exit 1
trap 'rm -r "$parts"' EXIT

ACCEPT_PARTS=$parts sh "$here/test_one.sh" crates/cli/tests/validation validation
rc=$?

# Says on stderr that the report was not written, and why; exits non-zero (the test run's code when it failed).
not_written() {
    echo "accept: the report was not written to $report: $1" >&2
    [ "$rc" -ne 0 ] || rc=1
    exit "$rc"
}

commit=${ACCEPT_COMMIT:-}
if [ -z "$commit" ]; then
    if [ "$(git rev-parse --show-toplevel 2>/dev/null)" = "$(pwd -P)" ]; then
        commit=$(git rev-parse HEAD)
        if [ -d .jj ] && command -v jj >/dev/null 2>&1; then
            changes=$(jj diff --summary -r @) || changes="jj diff failed"
        else
            git diff --quiet HEAD && changes= || changes="git diff found changes"
        fi
        [ -z "$changes" ] || commit="$commit, with uncommitted changes"
    else
        commit="not known: not run at the top of a git checkout, and ACCEPT_COMMIT is not set"
    fi
fi

# The sections: every case checks.txt lists (order|case|section), each with its part or a NOT RUN entry.
no_result="no result was recorded: the case did not run (a filtered run, or a test target that did not build),"
no_result="$no_result or its part could not be written; the terminal says which"
cut_short="its part was cut short before a whole Result line (a write that failed part way); the terminal says why"
sections() {
    previous=
    while IFS='|' read -r order name title; do
        case "$order" in "" | "#"*) continue ;; esac
        if [ "$title" != "$previous" ]; then
            printf '\n## %s\n' "$title"
            previous=$title
        fi
        echo
        if [ -f "$parts/$order-$name.md" ]; then
            cat "$parts/$order-$name.md"
            grep -Eq '^Result: (PASS|FAIL|NOT RUN \(.*\))$' "$parts/$order-$name.md" || printf '\nResult: NOT RUN (%s)\n' "$cut_short"
        else
            printf '### %s\n\nResult: NOT RUN (%s)\n' "$name" "$no_result"
        fi
    done < "$list"
}

sections > "$parts/sections.md" || not_written "could not assemble the sections from $list"

# The number of cases whose entry ends with `Result: <$1>`.
count() {
    grep -c "^Result: $1" "$parts/sections.md"
}

{
    echo "# Temperare acceptance report (make accept)"
    echo
    echo "- Commit: $commit"
    echo "- Date: $(date '+%Y-%m-%d %H:%M:%S %z')"
    echo "- Test run: crates/cli/tests/validation (these checks and cli's own tests of the built tm, S-28), exit $rc"
    if [ -f "$parts/00-head.md" ]; then
        cat "$parts/00-head.md"
    else
        echo "- tm and engines: not recorded (no case ran)"
    fi
    cat "$parts/sections.md"
    echo
    echo "$(count PASS) passed, $(count FAIL) failed, $(count 'NOT RUN') not run"
} > "$parts/report.md" || not_written "could not assemble it"

mkdir -p "$(dirname "$report")" || not_written "could not make its folder"
cp "$parts/report.md" "$report" || not_written "could not copy it there"
echo "accept: report written to $report"
not_run=$(count 'NOT RUN')
if [ "$rc" -eq 0 ] && [ "$not_run" -ne 0 ]; then
    echo "accept: $not_run checks did not run (the report says why); make accept passes only when all ran (S-34)" >&2
    exit 3
fi
exit "$rc"

#!/bin/sh
# accept.sh REPORT: `make accept` (mission, Done; S-31, SPEC-accept). Runs the validation target
# crates/cli/tests/validation through test_one.sh, as before, so the tests print on the terminal as they did; one line
# naming the report follows. Each acceptance case writes its part into a temporary folder this script names in
# ACCEPT_PARTS (crates/cli/tests/validation: check_new.rs names the part, check_drop.rs writes it). Then it writes
# the markdown report to REPORT: a head (commit, date, the test run's exit code, then the cctao and engine versions the
# cases saw), one section per check, and a last line of totals. Every case that crates/cli/tests/validation/checks.txt
# lists is in the report, in its order: with its part, or NOT RUN when it wrote none (a filtered run, a test target
# that did not build, a part that could not be written) or when its part was cut short before a whole Result line.
# Then it runs the suite on Windows at the commit with windows_run.sh (PLAN-finish-tao step 3: "Windows run green at
# the commit"), and the head says how that went. The script exits with the test run's exit code when it failed; else 1
# when the Windows run found failing tests; else 3 when any check is NOT RUN, or when the Windows run was not made (no
# machine, no push, or a commit that is not a plain HEAD the machine could clone), said on stderr, since a check not
# run has not passed (S-34); else 0. When the report cannot be written it exits non-zero with the reason on stderr.
# The commit is HEAD when the current folder is the top of a git checkout, noted when a file there differs from it,
# a deleted one included. In a jj folder jj answers, comparing only the files the folder holds, so a file a sparse
# request folder never held is not a change; elsewhere git diff does. ACCEPT_COMMIT, when set, names it instead.
set -u
report=$1
here=$(dirname "$0")
list=crates/cli/tests/validation/checks.txt
parts=$(mktemp -d "${TMPDIR:-/tmp}/cctao-accept-parts.XXXXXX") || exit 1
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

# The Windows run at the commit (windows_run.sh: exit 0 green, 1 tests fail there, 2 run not made). The machine clones
# the commit from origin, so a commit with uncommitted changes, or one not known, is not run there.
case "$commit" in
    "" | *[!0-9a-f]*)
        windows_rc=2
        windows="NOT RUN (not a plain commit the machine can clone: $commit)"
        ;;
    *)
        sh "$here/windows_run.sh" "$commit"
        windows_rc=$?
        out="target/windows-run/$(printf '%s' "$commit" | cut -c1-12)/out"
        case "$windows_rc" in
            0) windows="green at $commit ($out)" ;;
            1) windows="FAIL at $commit: tests failed there ($out/cargo-test.txt)" ;;
            *) windows="NOT RUN (the run could not be made; windows_run.sh said why on the terminal)" ;;
        esac
        ;;
esac

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
            last=$(grep -E '^Result: (PASS|FAIL|NOT RUN \(.*\))$' "$parts/$order-$name.md" | tail -n 1)
            if [ -z "$last" ]; then
                printf '\nResult: NOT RUN (%s)\n' "$cut_short"
                last="Result: NOT RUN"
            fi
        else
            printf '### %s\n\nResult: NOT RUN (%s)\n' "$name" "$no_result"
            last="Result: NOT RUN"
        fi
        printf '%s\n' "$last" >> "$parts/results.txt"
    done < "$list"
}

: > "$parts/results.txt" || not_written "could not start the tally"
sections > "$parts/sections.md" || not_written "could not assemble the sections from $list"

# The number of cases whose result is `<$1>`: one result per case, its last whole Result line, so a part cut short
# inside its own NOT RUN line is counted once.
count() {
    grep -c "^Result: $1" "$parts/results.txt"
}

{
    echo "# Code Chi Tao acceptance report (make accept)"
    echo
    echo "- Commit: $commit"
    echo "- Date: $(date '+%Y-%m-%d %H:%M:%S %z')"
    echo "- Test run: crates/cli/tests/validation (these checks and cli's own tests of the built cctao, S-28), exit $rc"
    echo "- Windows run (scripts/windows_run.sh, unit and integration groups): $windows"
    if [ -f "$parts/00-head.md" ]; then
        cat "$parts/00-head.md"
    else
        echo "- cctao and engines: not recorded (no case ran)"
    fi
    cat "$parts/sections.md"
    echo
    echo "$(count PASS) passed, $(count FAIL) failed, $(count 'NOT RUN') not run"
} > "$parts/report.md" || not_written "could not assemble it"

mkdir -p "$(dirname "$report")" || not_written "could not make its folder"
cp "$parts/report.md" "$report" || not_written "could not copy it there"
echo "accept: report written to $report"
not_run=$(count 'NOT RUN')
[ "$rc" -eq 0 ] || exit "$rc"
if [ "$windows_rc" -eq 1 ]; then
    echo "accept: the Windows run failed: $windows" >&2
    exit 1
fi
if [ "$not_run" -ne 0 ] || [ "$windows_rc" -ne 0 ]; then
    [ "$not_run" -eq 0 ] || echo "accept: $not_run checks did not run (the report says why)" >&2
    [ "$windows_rc" -eq 0 ] || echo "accept: the Windows run was $windows" >&2
    echo "accept: make accept passes only when all ran (S-34)" >&2
    exit 3
fi
exit 0

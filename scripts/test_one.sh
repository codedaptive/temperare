#!/bin/sh
# test_one.sh DIR GROUP: the tests of one crate, source module or test target in one group (D281, three kinds of
# test). Groups differ only in which test targets run and which name filter libtest gets, so switching groups
# never rebuilds anything.
#   unit         the library's tests (--lib), minus any `integration` module inside them. The default; the only
#                group leads and the landing check run.
#   integration  the library's `integration::` modules, then every test target in tests/ except `validation`.
#   validation   the crate's `validation` test target (tests/validation/main.rs), in release.
# DIR=crates/<crate>/src/<a>/<b> filters library tests to a::b:: (unit only: libtest ORs its name filters, so one
# call cannot select a module's `integration::` tests; run those with DIR=crates/<crate> GROUP=integration).
# DIR=crates/<crate>/tests runs the integration targets (GROUP=validation: the validation target);
# DIR=crates/<crate>/tests/<target> runs that target, and tests/validation runs in release like GROUP=validation.
# CARGO (default cargo) and CARGO_TEST_ARGS come from the environment. A bare CARGO_TEST_ARGS name replaces a module's
# filter; dash arguments are appended. Arguments after a `--` in CARGO_TEST_ARGS reach libtest beside the group filter.
set -u
dir=${1:-}
group=${2:-unit}
cargo=${CARGO:-cargo}
args=${CARGO_TEST_ARGS:-}

case "$group" in unit|integration|validation) ;; *) echo "test-one: GROUP must be unit, integration or validation"; exit 2 ;; esac
case "$dir" in crates/?*) ;; "") echo "test-one: set DIR"; exit 2 ;; *) echo "test-one: DIR must start with crates/"; exit 2 ;; esac

d=${dir%/}; p=${d#crates/}; p=${p%%/*}; rest=${d#crates/$p}; root=crates/$p

# libtest arguments go after one `--`: reuse the caller's when CARGO_TEST_ARGS has one.
case " $args " in *" -- "*) sep="" ;; *) sep="--" ;; esac

# The test targets cargo discovers in tests/: each tests/<t>.rs and tests/<t>/main.rs, minus validation.
integration_targets() {
    for f in "$root"/tests/*.rs "$root"/tests/*/main.rs; do
        [ -f "$f" ] || continue
        case "$f" in */main.rs) t=${f%/main.rs}; t=${t##*/} ;; *) t=${f##*/}; t=${t%.rs} ;; esac
        [ "$t" = validation ] || printf ' --test %s' "$t"
    done
}

has_lib() { [ -f "$root/src/lib.rs" ]; }

run_unit() {
    has_lib || { echo "test-one: $p has no library, so no unit tests"; return 0; }
    $cargo test -p "$p" --lib $args $sep --skip integration::
}

run_integration() {
    rc=0
    if has_lib; then $cargo test -p "$p" --lib $args $sep integration:: || rc=1; fi
    targets=$(integration_targets)
    if [ -n "$targets" ]; then $cargo test -p "$p" $targets $args || rc=1; fi
    return $rc
}

run_validation() {
    [ -f "$root/tests/validation/main.rs" ] || { echo "test-one: $p has no validation target"; return 0; }
    $cargo test -p "$p" --release --test validation $args
}

case "$rest" in
    "") "run_$group" ;;
    /src/*)
        [ "$group" = unit ] || { echo "test-one: a source module runs its unit tests only; use DIR=$root GROUP=$group"; exit 2; }
        m=$(echo "${rest#/src/}" | sed 's|/|::|g')
        case "$args" in
            ""|-*) $cargo test -p "$p" --lib "$m::" $args $sep --skip integration:: ;;
            *) $cargo test -p "$p" --lib $args $sep --skip integration:: ;;
        esac ;;
    /tests)
        if [ "$group" = validation ]; then run_validation
        else
            targets=$(integration_targets)
            [ -n "$targets" ] || { echo "test-one: $p has no integration targets"; exit 0; }
            $cargo test -p "$p" $targets $args
        fi ;;
    /tests/validation) run_validation ;;
    /tests/*) $cargo test -p "$p" --test "${rest##*/}" $args ;;
    *) echo "test-one: DIR must name a crate, source module, or test target"; exit 2 ;;
esac

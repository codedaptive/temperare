#!/bin/sh
# windows_run.sh: temperare's unit and integration suite run for real on the Windows machine, from this Mac
# (PLAN-finish-tao-2026-11-25 step 3; Bob via creator #3447). Copied from tensile develop:scripts/windows_run.sh,
# without its second run of the ccjin it builds (temperare builds none). Usage: sh scripts/windows_run.sh [<commit>]
# (default develop). Below, <short> is the commit's first 12 hex digits.
#
# The machine clones from GitHub: the commit is pushed to origin as a throwaway branch, ci/<short>, so a commit on no
# branch (a request's) can be tested too; the branch is deleted after the run. The machine clones temperare at the
# commit into <home>\tm-ci\<short>\, under the home of the user ssh logs in as, and cargo fetches ccvcslib by the tag
# Cargo.lock pins, through the git CLI. There it runs cargo test --workspace --locked over the unit and integration
# groups (not validation, which runs the real engines in release: scripts/test_one.sh, scripts/accept.sh).
# It brings back the output and the exit code into target/windows-run/<short>/out/, prints the failing test names,
# and deletes the folder on the machine. Nothing else there is touched: cargo's home is inside the folder.
#
# Exit 0 when the suite exits 0; 1 when it does not; 2 when the run could not be made (no machine, no push, or no
# cargo test exit code came back).
# Settings: WINDOWS_HOST (default gammaAI), TEMPERARE_REPO (default this checkout).
set -eu

host=${WINDOWS_HOST:-gammaAI}
repo=${TEMPERARE_REPO:-$(git rev-parse --show-toplevel)}
sha=$(git -C "$repo" rev-parse --verify "${1:-develop}^{commit}") || exit 2
short=$(printf '%s' "$sha" | cut -c1-12)
url=$(git -C "$repo" remote get-url origin) || exit 2
ref="refs/heads/ci/$short"
# A machine that does not answer in 20 s, or stops answering for 2 minutes, ends the run with exit 2 rather than
# hanging.
ssh_opts="-o BatchMode=yes -o ConnectTimeout=20 -o ServerAliveInterval=30 -o ServerAliveCountMax=4"
# The folder goes under the home of the user ssh logs in as, which the machine names, so no user name is written here.
home=$(ssh $ssh_opts "$host" 'Write-Output $env:USERPROFILE' | tr -d '\r') || exit 2
[ -n "$home" ] || exit 2
remote="$home\\tm-ci\\$short"
out="$repo/target/windows-run/$short"

# The unit and integration groups: every crate's library and binaries (their integration:: modules included), and
# each test target in a crate's tests/ except `validation`.
targets=$(git -C "$repo" ls-tree -r --name-only "$sha" crates \
  | sed -n -e 's|^crates/[^/]*/tests/\([^/]*\)\.rs$|\1|p' -e 's|^crates/[^/]*/tests/\([^/]*\)/main\.rs$|\1|p' \
  | grep -v '^validation$' | sort -u | sed 's/^/--test /' | tr '\n' ' ')
suite="cargo test --workspace --locked --lib --bins $targets"

# Pushed from a throwaway bare repository that borrows this one's objects (alternates), so the push and the delete
# write nothing in the shared repository (in Tensile, a push from the shared one was followed by a jj import in an
# open request folder abandoning that request's commits).
stage=$(mktemp -d)
trap 'git -C "$stage/push.git" push -q "$url" ":$ref" || echo "windows_run: could not delete $ref on origin"; rm -rf "$stage"' EXIT
objects=$(git -C "$repo" rev-parse --path-format=absolute --git-common-dir)/objects
git init -q --bare "$stage/push.git"
printf '%s\n' "$objects" > "$stage/push.git/objects/info/alternates"
git -C "$stage/push.git" push -q "$url" "$sha:$ref" || exit 2

# The steps on the machine, in PowerShell 5.1. cmd /c does the redirection, which PowerShell 5.1 would write as
# UTF-16.
cat > "$stage/run.ps1" <<EOF
\$ErrorActionPreference = 'Continue'
Set-Location '$remote'
New-Item -ItemType Directory -Force -Path out | Out-Null
git init -q src 2>&1 | Out-Null
git -C src fetch -q '$url' '$ref' 2>&1 | Out-Null
git -C src checkout -q $sha 2>&1 | Out-Null
\$env:CARGO_HOME = '$remote\\cargo-home'
\$env:RUSTUP_HOME = "\$env:USERPROFILE\\.rustup"
# cargo's own git cannot use the machine's GitHub login (gh's credential helper); the git CLI can.
\$env:CARGO_NET_GIT_FETCH_WITH_CLI = 'true'
Set-Location src
cmd /c "$suite --no-fail-fast > ..\\out\\cargo-test.txt 2>&1"
\$test = \$LASTEXITCODE
Set-Content -Encoding ascii -Path ..\\out\\exits.txt -Value "cargo_test=\$test"
EOF

# Ship the steps, run, fetch, delete. Once the machine's folder is made it goes even when a step fails: a failed
# upload skips the run, not the removal.
ssh $ssh_opts "$host" "New-Item -ItemType Directory -Force -Path '$remote' | Out-Null" || exit 2
rc=0
COPYFILE_DISABLE=1 tar -c -f - -C "$stage" run.ps1 | ssh $ssh_opts "$host" "tar -x -f - -C '$remote'" || rc=2
if [ "$rc" = 0 ]; then
  ssh $ssh_opts "$host" "powershell -NoProfile -ExecutionPolicy Bypass -File '$remote\\run.ps1'" || rc=2
fi
rm -rf "$out"
mkdir -p "$out"
ssh $ssh_opts "$host" "tar -c -f - -C '$remote' out" | tar -x -f - -C "$out" || rc=2
# Messages go through printf '%s': sh's echo reads a Windows path's backslashes as escapes (\t as a tab).
ssh $ssh_opts "$host" "Remove-Item -Recurse -Force '$remote'" ||
  printf '%s\n' "windows_run: could not remove $remote on $host"

exits="$out/out/exits.txt"
if [ ! -f "$exits" ]; then
  printf '%s\n' "windows_run: $short on $host: no result came back"
  exit 2
fi
test_rc=$(sed -n 's/^cargo_test=//p' "$exits" | tr -d '\r')
# exits.txt with no cargo_test= line, or one that is not a number, is no test result: the run was not made.
case "$test_rc" in
  "" | *[!0-9-]*)
    printf '%s\n' "windows_run: $short on $host: no result came back (exits.txt holds no cargo test exit code)"
    exit 2
    ;;
esac
printf '%s\n' "windows_run: $short on $host: cargo test exit $test_rc"
grep -a '^test .* FAILED' "$out/out/cargo-test.txt" | tr -d '\r' || true
printf '%s\n' "windows_run: output in $out/out"
[ "$rc" = 0 ] || exit "$rc"
[ "$test_rc" = 0 ] && exit 0
exit 1

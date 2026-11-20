# Temperare rulings

Bob's rulings. Only Bob edits this file.

## R-1: main, candidate, develop (Bob, 2026-11-19; creator #672)
- **main:** clean and public facing, polite and well behaved.
- **candidate:** the place to prepare the next main.
- **develop:** "a hive of scum and villainy". Nobody but us should base work on it. We cut our work trees from it
  and break things; we may leave a mess, squash, or delete history, and anyone outside who forks it does so at their
  own peril.
- The Backporter runs develop to candidate to main. It fixes and cleans things in develop as they appear, and does
  not worry about develop's history. No scrubbing; if a hard boundary is ever needed, adapt the "-ce" pattern (a
  separate scrubbed repo, as codeteam-ce).
- The repository is public as it is, history included.

## R-2: tokens saved (Bob, 2026-11-20; creator #802)
- An answer that replaces reading opens with one line, as graft's does, so the agent's context shows why the tool is
  worth having: "[temperare] tokens saved ≈ N (P%) — this output ≈ X tok vs <baseline> ≈ B tok (estimate)".
  Tokens are characters divided by 4.
- The line ends with graft's ask: at the end of its reply, the agent tells the user the total saved this turn,
  summed across that tool's calls.
- For `tm check` the baseline is the raw output of every engine it ran (clippy, SwiftLint, Ruff and the rest),
  before tm reduced it to new findings with the fix named.
- JSON output carries the same numbers in a "saved" object: what was counted and its character total.
- An off switch in the user configuration, documented in the help, turns the line and the ask off; the JSON field
  stays.
- Each answer appends one line to ~/.local/state/temperare/savings.jsonl (time, folder, verb, output and baseline
  tokens), so the deck can total it per team and day.
- The help says the number is an upper bound against reading everything whole.

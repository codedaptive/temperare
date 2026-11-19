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

# Code Chi Tao

Read the mission first: `~/dev_docs/forge-design/MISSION-temperare-2026-11-16.md`.

- Build: `make check`. One crate's tests: `make zone-test-one DIR=crates/<crate>` (GROUP=unit by default).
- Code: `tessera/docs/CHARTER-small-code-files-2026-10-30.md` (one public symbol per file, ~150 lines).
- A unit test is given an engine's output; it does not run the engine. A test that runs a real engine
  runs it on files the test made, in a throwaway repository it removes. Nothing reaches the network.
- Decisions: the package's spec (`SPEC-<package>.md`) and your work order say what is settled. The repository
  holds no rulings file.

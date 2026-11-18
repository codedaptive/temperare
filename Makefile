# Temperare workspace targets called by codeteam's zone.mk and by agents.
# MAKEFILES=<codeteam>/zone/zone.mk supplies zone-test, zone-test-one, zone-test-status and warm.
CARGO ?= cargo
.PHONY: test-one test-group check test accept

# Three kinds of test: GROUP=unit (default), integration or validation; scripts/test_one.sh says what each runs.
GROUP ?= unit
test-one:
	@CARGO="$(CARGO)" CARGO_TEST_ARGS="$(CARGO_TEST_ARGS)" sh "$(CURDIR)/scripts/test_one.sh" "$(DIR)" "$(GROUP)"

test-group:
	@rc=0; for d in crates/*/; do [ -f "$$d/Cargo.toml" ] || continue; \
	  CARGO="$(CARGO)" sh "$(CURDIR)/scripts/test_one.sh" "$${d%/}" "$(GROUP)" || rc=1; \
	done; exit $$rc

check:
	$(CARGO) check --workspace --all-targets

test:
	@$(MAKE) --no-print-directory test-group GROUP=$(GROUP)

# The Done list (mission, Done 1-8): the built tm with the real engines, on throwaway repositories
# (crates/cli/tests/validation, the orchestrator's; release, as every validation target). scripts/accept.sh runs it
# and writes the report to ACCEPT_REPORT (S-31); the default is inside the repository, so it names no team path.
ACCEPT_REPORT ?= $(CURDIR)/target/accept-report.md
accept:
	@CARGO="$(CARGO)" CARGO_TEST_ARGS="$(CARGO_TEST_ARGS)" sh "$(CURDIR)/scripts/accept.sh" "$(ACCEPT_REPORT)"

ifeq ($(GROUP),unit)
ZONE_TEST = $(CARGO) test --workspace --lib -- --skip integration::
else
ZONE_TEST = $(MAKE) --no-print-directory test-group GROUP=$(GROUP)
endif

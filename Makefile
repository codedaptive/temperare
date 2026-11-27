# Code Chi Tao workspace targets called by codeteam's zone.mk and by agents.
# MAKEFILES=<codeteam>/zone/zone.mk supplies zone-test, zone-test-one, zone-test-status and warm.
CARGO ?= cargo
.PHONY: test-one test-group check check-cross test accept windows-run

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

# The type-check for Windows and Linux from this Mac (PLAN-finish-tao step 3: Linux needs only a type-check; Windows
# code behind cfg(windows) is caught here before a gammaAI run). Each target's standard library must be installed.
CROSS_TARGETS ?= x86_64-pc-windows-msvc x86_64-unknown-linux-gnu
check-cross:
	@rc=0; for t in $(CROSS_TARGETS); do $(CARGO) check --workspace --all-targets --target $$t || rc=1; done; exit $$rc

test:
	@$(MAKE) --no-print-directory test-group GROUP=$(GROUP)

# The Done list (mission, Done 1-10): the built cctao with the real engines, on throwaway repositories
# (crates/cli/tests/validation, lane-written; release, as every validation target). scripts/accept.sh runs it
# and writes the report to ACCEPT_REPORT (S-31); the default is inside the repository, so it names no team path.
ACCEPT_REPORT ?= $(CURDIR)/target/accept-report.md
accept:
	@CARGO="$(CARGO)" CARGO_TEST_ARGS="$(CARGO_TEST_ARGS)" sh "$(CURDIR)/scripts/accept.sh" "$(ACCEPT_REPORT)"

# The unit and integration suite on the Windows machine (WINDOWS_HOST, default gammaAI), at COMMIT (default
# develop): scripts/windows_run.sh says how. Output in target/windows-run/<short>/out/.
COMMIT ?= develop
windows-run:
	@sh "$(CURDIR)/scripts/windows_run.sh" "$(COMMIT)"

ifeq ($(GROUP),unit)
ZONE_TEST = $(CARGO) test --workspace --lib -- --skip integration::
else
ZONE_TEST = $(MAKE) --no-print-directory test-group GROUP=$(GROUP)
endif

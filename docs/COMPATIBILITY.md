# Compatibility for 0.1.x

## Maintenance and versioning

This is the first non-prerelease **0.1.0**. Patch releases in 0.1.x preserve the
supported interface below, defaults, error behavior and minimum platform version.
Incompatible changes require a new minor (0.2.0 or later), migration notes and
regression tests; they must not be hidden in a patch. A future 1.0 additionally
requires a separate long-term compatibility review. Internal implementation details
and the exclusions below are not added to the supported API.

Maintenance is best effort by PerchAnton. Fixes target the latest 0.1.x patch;
older patches and prereleases are not separate support branches. There is no SLA,
fixed maintenance end date, independent security audit or guaranteed response time.
Document deprecations and alternatives before removal. Report reproducible defects
with exact versions and synthetic inputs, without secrets or personal documents.
Application owners remain responsible for native regression tests and rollout.

## Supported surface

- Rust 1.88 or later, edition 2024, `no_std`, no runtime dependencies or unsafe code.
- `EditSession<T, C>`, `Change<T>` and the three `FinishError` variants. Existing
  constructors, accessors, proposal, reset, check, cancel and finish methods remain.
- Only construction/reset require `T: Clone`; checks/finish require `T: PartialEq`
  and `C: PartialEq`. Contexts need not implement `Clone`, `Copy` or `Debug`.
- Context conflict takes precedence over value conflict; both take precedence over
  an invalid proposal. `check_live` deliberately ignores proposal validity so the
  host can keep an invalid draft open for correction.
- Invalid proposals preserve the last valid preview. Reset restores the original
  preview and clears invalidity without rebasing the captured context.
- Finish consumes the draft, returning either one change, a no-op, or an error.
  No-op/cancel do not issue commands. A failed finish also consumes the draft; use
  checks before consuming it if the UI needs to retain it for correction.

The host must supply independent snapshots and an identity/revision context that
changes on edits **and Undo/Redo**, serialize check/command application, validate
domain rules, and own history. This crate provides no transaction or concurrency
lock. An `Rc<RefCell<_>>` clone is not an independent snapshot.

## Upgrading and evidence

Pin the exact release (`=0.1.0`). Re-run the host's native
regressions before changing the pin. The upgrade from beta.2 adds no fields to stored documents:
sessions are in-memory state, not a serialization format. Error variants are for
branching; Display/Debug prose and memory layout are not a machine protocol.

`tests/consumer_contract.rs` exercises a downstream-style API with minimal trait
bounds and simultaneous failures. Existing lifecycle tests and the retry/context
examples remain required. Packaging runs those checks from the extracted crate on
the MSRV as well as testing the checkout. These are compatibility checks, not a
claim that an external dependency is better than a correct local helper.

The runtime is unchanged from 0.1.0-beta.2. Existing callers need only change
the dependency pin and run their native checks; there is no stored-data migration.
Never replace a published version.

## Dependency approval is separate

We can supply the exact source/archive hash, dual-license notices, dependency and
build-input inventory, tests, a local-helper comparison, and a patch for an
organisation-owned source or licensed vendored copy. Maintainers decide whether
the feature is useful, which source/owner to accept and how updates are maintained.
Keep their source gate intact while that decision is pending; a pin, green CI or
release version does not mean a dependency was reviewed or approved. Running the
separate interaction-check tool in CI needs its own permission/workflow decision.

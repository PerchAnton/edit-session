# Compatibility for 1.x

## Maintenance and versioning

This is the compatibility contract for **1.0.0 and the 1.x series**.
The earlier 0.1.0 contract remains available on the v0.1.0 tag.
Version 1.0 introduces a compatibility commitment, without changing runtime behavior.

Follow [Semantic Versioning 2.0](https://semver.org/spec/v2.0.0.html): patches fix
bugs while preserving the documented contract; minor releases add compatible
features or deprecations; incompatible changes require a new major and migration
notes. Deprecations document a replacement and remain available throughout 1.x.
Changes to defaults, error semantics, minimum platforms or existing input limits
are breaking when they invalidate supported consumers. Internal details and the
exclusions below are outside this contract. A bug fix restores documented behavior;
it must include a reproducer and describe any changed result.

Maintenance by PerchAnton is best effort, targeting the latest 1.x release once
published. Older releases remain downloadable, not separate maintenance branches.
There is no SLA, fixed support term or independent security audit. Report defects
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

After publication, upgrade the dependency to `=1.0.0` (or `1` for compatible updates). Re-run the host's native
regressions before changing the pin. The upgrade from 0.1.0 adds no fields to stored documents:
sessions are in-memory state, not a serialization format. Error variants are for
branching; Display/Debug prose and memory layout are not a machine protocol.

`tests/consumer_contract.rs` exercises a downstream-style API with minimal trait
bounds and simultaneous failures. Existing lifecycle tests and the retry/context
examples remain required. Packaging runs those checks from the extracted crate on
the MSRV as well as testing the checkout. These are compatibility checks, not a
claim that an external dependency is better than a correct local helper.

The runtime is unchanged from 0.1.0. Existing callers need only change
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

## Public type evolution

`FinishError` has three exhaustively matchable variants. Adding a variant, adding
fields to `Change<T>` (which supports struct literals), adding required trait
bounds, or losing existing trait implementations requires a major release.
Rust 1.88 and `no_std` remain supported throughout 1.x. New optional methods may
be minor additions only when existing source still compiles. Memory layout and
Debug/Display wording remain outside the contract.

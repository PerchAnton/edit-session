# Beta compatibility boundary

`0.1.0-beta.1` was published on 1 November 2026 (Europe/Kyiv); it remains a
prerelease, not a stable 1.0 release. Runtime implementation is unchanged from
`0.1.0-alpha.5`; the added checks make the intended consumer contract explicit.
[Publication and registry checks](evidence/release-beta1.json).

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

Pin the exact prerelease (`=0.1.0-beta.2`). Re-run the host's native
regressions before changing the pin. The beta adds no fields to stored documents:
sessions are in-memory state, not a serialization format. Error variants are for
branching; Display/Debug prose and memory layout are not a machine protocol.

`tests/consumer_contract.rs` exercises a downstream-style API with minimal trait
bounds and simultaneous failures. Existing lifecycle tests and the retry/context
examples remain required. Packaging runs those checks from the extracted crate on
the MSRV as well as testing the checkout. These are compatibility checks, not a
claim that an external dependency is better than a correct local helper.

Within beta, preserve this surface; a necessary incompatible correction must use a
new prerelease, document the migration and add a regression. Never replace a
published version. Version 1.0 additionally needs a separately reviewed long-term
support/deprecation policy and consumer experience. Beta is not that promise.

## Dependency approval is separate

We can supply the exact source/archive hash, dual-license notices, dependency and
build-input inventory, tests, a local-helper comparison, and a patch for an
organisation-owned source or licensed vendored copy. Maintainers decide whether
the feature is useful, which source/owner to accept and how updates are maintained.
Keep their source gate intact while that decision is pending; a pin, green CI or
beta version does not mean a dependency was reviewed or approved. Running the
separate interaction-check tool in CI needs its own permission/workflow decision.

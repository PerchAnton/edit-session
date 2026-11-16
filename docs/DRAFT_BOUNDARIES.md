# Nested Cancel and canonical saves

Two runnable host models exercise existing `edit-session` APIs. They are not GUI
tests of the applications below, accepted integrations, or evidence that a new
dependency is better than a small local helper. No library API was added.

## Cancel the confirmation, keep the parent form

[COSMIC Settings #1718](https://github.com/pop-os/cosmic-settings/issues/1718)
reports losing a new shortcut's name and command when cancelling a replacement
confirmation. At [the inspected source](https://github.com/pop-os/cosmic-settings/blob/6aaf30d69ec3ed64eeb1d441ef5fe0a2017b3fe6/cosmic-settings/src/pages/input/keyboard/shortcuts/custom.rs),
`ReplaceCancel` resets the parent `add_shortcut` state.

```sh
cargo run --locked --example nested_cancel
cargo test --locked --example nested_cancel
```

The [example](../examples/nested_cancel.rs) separates three decisions:

1. Cancel replacement: retain name and command, clear the disputed combination,
   and leave the parent draft open for another combination.
2. Cancel the whole form: discard the draft without a host write.
3. Confirm: check the current value and configuration revision immediately before
   the serialized host command. A stale draft stays open and does not overwrite.

Tests exercise both `EditSession` and a correct local helper against the same host.
Both must behave identically. The library does not decide which dialog was
cancelled: application state still owns that distinction. The separate reported
Reassign problem is not claimed fixed by this model. For a new-target workflow,
the host must additionally own target allocation and binding-conflict resolution;
this example models one fixed settings target.

## Display what the host actually saved

[spotatui #1063](https://github.com/LargeModGames/spotatui/issues/1063) describes
visible settings that differ from the accepted configuration after clamping or
rejection. [PR #1092](https://github.com/LargeModGames/spotatui/pull/1092) already
addresses the application; this example is not a competing patch.

```sh
cargo run --locked --example canonical_save
cargo test --locked --example canonical_save
```

The [example](../examples/canonical_save.rs) keeps raw fields until the host accepts
the write. On success it reloads the canonical value: entering `500` produces a
saved and displayed `100`. A rejected icon or failed write keeps the draft and
the previous saved snapshot. A retry checks freshness again. A successful save
starts an explicitly new session against the new revision; Reset is not used to
silently rebase a stale proposal.

The host model only accepts one printable ASCII icon and simulates an atomic
write. It does not implement terminal character width or a filesystem transaction.
A real host must handle partial storage failures, permissions and rollback itself.
Using `finish` before a fallible command would consume the draft too early here;
the example borrows `preview` after `check` and rebuilds only after success.

## Acceptance and cost

The two examples add ten tests, separate from existing lifecycle/consumer tests.
These are model-level regressions, not a reproduction of either application UI.
Keep the native application regression even if adopting the crate. First try a
local helper; sharing a lifecycle across several forms may justify the dependency,
but less code or faster execution has not been measured here.

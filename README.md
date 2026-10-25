# edit-session

**Experimental, 0.1.0-alpha.2.** An independent library maintained by
[PerchAnton](https://github.com/PerchAnton). Available as a Git dependency; not published to
crates.io or adopted by upstream PhotoCraft/PdfCraft.

A small Rust library for the lifecycle of an edit: begin, preview, reject invalid input,
cancel, and produce a single change for the host to commit. The intended first consumers are
PhotoCraft and PdfCraft. It has no dependencies and uses `no_std`.

The host keeps its renderer, numeric parser, units, command dispatch and undo history.
`edit-session` holds small draft parameters and a context supplied by the host. It neither
applies edits nor promises that a renderer's preview equals the committed output.

## Use from another project

Add this entry to your crate's `[dependencies]` to use the alpha 2 API at a fixed revision:

```toml
edit-session = { git = "https://github.com/PerchAnton/edit-session", rev = "033f7c3a67b825b8f222288788cc61234c0c945d" }
```

Commit the resulting `Cargo.lock`. The companion
[interaction-check](https://github.com/PerchAnton/interaction-check) project replays semantic
scenarios against adapters using this library.

```rust
use edit_session::EditSession;

// Stable document and layer identities plus a monotonically changing revision.
let context = ("document-7", "layer-2", 42_u64);
let live_opacity = 100_u8;
let mut draft = EditSession::new(live_opacity, context);
draft.propose::<&str>(Ok(75)).unwrap(); // Validation belongs to the host.
draft.propose::<&str>(Ok(50)).unwrap();

if let Some(change) = draft.finish(&live_opacity, &context).unwrap() {
    // Pass change.after to the application's validated command ONCE.
    assert_eq!(change.after, 50);
}
```

`finish` consumes the session. A no-op yields `None`. Invalid input retains the last good
preview but blocks commit; correction makes the session valid again. Changed context or
live value rejects the draft. Call `check` before `finish` when invalid input must remain
editable. Cancel or drop only discards local data, so no rollback command is necessary.

## Keeping a dialog open

Alpha 2 adds two operations without changing `check` or `finish` semantics:

- `check_live(&live, &context)` checks document/target/revision and the original value,
  independently of proposal validity. A host can keep incomplete input editable while
  dismissing a dialog whose target has changed. Success does **not** mean the proposal can
  be committed: `check` and `finish` still reject invalid input.
- `reset()` restores the original preview and clears invalid input without consuming the
  session. The host refreshes the displayed controls/preview. Confirming immediately is a
  no-op; the user can also continue editing. The original context is preserved, so Reset
  cannot make a stale session current again or undo another document edit.

`FinishError` now implements `core::fmt::Display` and `core::error::Error` for standard
error reporting. The library remains dependency-free and `no_std`, with Rust 1.88 as its minimum.

## Context contract

Use a document identity, target identity, selection identity where relevant, and a revision
that changes on all relevant edits, undo and redo. Never reuse an identity for a different
document or target. Comparing values alone misses an edit followed by Undo back to the same
value. `check`/`finish` and host command dispatch must occur on the host's serialized edit
path; this API is not a lock and cannot make separately scheduled writes atomic.

The host must discard/dismiss the preview on document close, target change or other invalidation,
handle command failure without adding history, and revalidate its own permissions and locks.
Applying preview values directly to the live model violates this library's contract.

## Validation

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

The CI matrix tests Rust 1.88.0 and stable. Alpha 2 also passed locally on Rust 1.88.0,
macOS arm64; see [validation evidence](docs/VALIDATION.md).
The separate PhotoCraft adapter currently targets Rust 1.99 and pins its own dependencies.

## Proposed PhotoCraft UI integration

[PhotoCraft PR #9190](https://github.com/storytold/photocraft/pull/9190) uses this library
inside **Edit Selected Guides** for live canvas previews, invalid-input recovery, cancellation,
stale-context protection and no-op confirmation that preserves Redo. The integration branch
also uses interaction-check for 10 scenarios / 44 steps against the real dialog and Session.
Both dependencies are pinned to immutable commits; its library pin remains the alpha 1
revision `28620b8264660ea84a182110cf5894da90281c42`. The alpha 2 additions are not yet used there.
This is a working, tested consumer patch
awaiting maintainer review; upstream adoption is not claimed.

## PhotoCraft integration pilot

The [adapter in interaction-check](https://github.com/PerchAnton/interaction-check/tree/main/adapters/photocraft)
fetches this library as a pinned **Git dependency** alongside PhotoCraft's real document and
engine crates at `14a4ab9b3f38f0b68fde6add97384919710374c8`. It stages opacity in `EditSession`,
then calls `Session::execute("layer.setProps", ...)` once on commit. PhotoCraft owns the actual
validation, document changes, history, Undo and Redo. The adapter writes no history itself.

```sh
git clone https://github.com/PerchAnton/interaction-check
cd interaction-check
cargo build --locked --manifest-path adapters/photocraft/Cargo.toml
python3 interaction_check.py scenarios/photocraft/*.json -- \
  adapters/photocraft/target/debug/edit-session-photocraft-probe
```

This is an **external, headless integration pilot**. Upstream PhotoCraft has not added this
dependency and its GUI is unchanged. It is not a fix for a PhotoCraft issue. PdfCraft has not
been built or adapted by this project.

See [research and adoption decision](docs/DECISION.md) and [validation evidence](docs/VALIDATION.md).

## Limits of this first version

No widget, expression parser, units conversion, IME support, background preview scheduling,
automatic transaction rollback, multi-property conflict merge or persistent undo. These are
separate responsibilities. Before turning this small module into a dependency, demonstrate
that it removes meaningful duplication in two real consumers. If it does not, an in-tree
helper is the better contribution.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
Unless explicitly stated otherwise, contributions submitted for inclusion are offered
under the same dual license.

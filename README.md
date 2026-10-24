# edit-session

**Experimental, 0.1.0-alpha.1.** An independent library maintained by
[PerchAnton](https://github.com/PerchAnton). Available as a Git dependency; not published to
crates.io or adopted by upstream PhotoCraft/PdfCraft.

A small Rust library for the lifecycle of an edit: begin, preview, reject invalid input,
cancel, and produce a single change for the host to commit. The intended first consumers are
PhotoCraft and PdfCraft. It has no dependencies and uses `no_std`.

The host keeps its renderer, numeric parser, units, command dispatch and undo history.
`edit-session` holds small draft parameters and a context supplied by the host. It neither
applies edits nor promises that a renderer's preview equals the committed output.

## Use from another project

Add this entry to your crate's `[dependencies]`, replacing `FULL_COMMIT_SHA` with the
immutable commit you have reviewed from this repository:

```toml
edit-session = { git = "https://github.com/PerchAnton/edit-session", rev = "FULL_COMMIT_SHA" }
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

The manifest declares Rust 1.88; the current validation used Rust 1.99.1 on macOS arm64.
The declared minimum has not been tested yet. The PhotoCraft probe requires its upstream
Rust floor (1.95) or later.

## PhotoCraft compatibility probe

`pilots/photocraft` links the actual `photocraft-doc` and `photocraft-ops` crates at commit
`14a4ab9b3f38f0b68fde6add97384919710374c8`. It creates two synthetic layers and edits opacity
through the draft, then records the resulting change in the real PhotoCraft `History`.

```sh
cargo build --locked --manifest-path pilots/photocraft/Cargo.toml
python3 ../interaction-check/interaction_check.py \
  ../interaction-check/scenarios/photocraft/*.json -- \
  pilots/photocraft/target/debug/edit-session-photocraft-probe
```

This is a **component compatibility probe**, not a live UI integration and not a fix for a
PhotoCraft issue. A production frontend must dispatch through PhotoCraft's `Session`/commands;
it must not copy this probe's direct `History` writes into UI handlers. PdfCraft has not been
built or adapted by this prototype.

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

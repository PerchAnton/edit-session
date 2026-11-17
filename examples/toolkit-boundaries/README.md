# Toolkit boundaries with the published crate

This independent workspace uses `edit-session = "=1.0.0"` from crates.io.
Its dependencies are example-only; the main library stays dependency-free.
Run the small host-side recipes without starting a window or terminal:

```sh
cargo test --locked --manifest-path examples/toolkit-boundaries/Cargo.toml
cargo clippy --locked --manifest-path examples/toolkit-boundaries/Cargo.toml --all-targets -- -D warnings
cargo fmt --manifest-path examples/toolkit-boundaries/Cargo.toml --check
```

Rust 1.99.1 is used for this set of current toolkit versions. This does not change
the core crate's Rust 1.88 minimum. `Cargo.lock` pins the full dependency tree.

| Recipe | Actual API exercised | Boundary checked | Tests |
| --- | --- | --- | ---: |
| [Dioxus](tests/dioxus.rs), 0.7.10 | `VirtualDom`, `Signal<Option<EditSession<...>>>`, `with_mut` | Non-Clone draft storage, local Cancel, revision checked at confirmation, single commit | 2 |
| [Yew](tests/yew.rs), 0.23.0 | `Callback::from` / `emit`, `Rc<RefCell<_>>` host | Duplicate callback cannot reuse a consumed session; late callback reads current revision | 2 |
| [Sycamore](tests/sycamore.rs), reactive 0.9.4 | Reactive root, signals, `update` / `with` | Preview does not publish canonical state; Cancel; commit inside a serial host update | 2 |
| [Vizia](tests/vizia.rs), vizia_id 0.4.0 | `IdManager`, real slot reuse and generation | A recycled slot with equal value is a different target; revision also detects Undo to equal value | 2 |
| [Iced](tests/iced.rs), runtime 0.14.0 | `Task::done`, task stream, `Action::Output` | Carry the unconsumed session through delivery; validate against live state when handling it | 3 |
| [Slint](tests/slint.rs), 1.19.0 | `SharedString`, `VecModel`, `Model::set_row_data` | Independent copy-on-write preview; stable row ID; update the model after acceptance | 3 |
| [tui-realm](tests/tuirealm.rs), 4.1.0 | `Cmd`, `CmdResult`, `State`, `StateValue` | Incomplete text retains the last preview but blocks Submit; Cancel; stale/no-op outcomes | 3 |
| [TachyonFX](tests/tachyonfx.rs), 0.25.2 | `fx::fade_to_fg`, `Effect::process`, real Ratatui buffers | Edit immutable effect settings, render a separate preview, discard it on Cancel, reject a stale selected effect | 3 |

All 20 native tests pass on macOS ARM. These are executable boundary recipes,
not complete UI applications. Dioxus uses a VirtualDOM scope but no renderer;
Yew does not mount a browser component; Sycamore exercises its reactive engine;
Vizia uses its ID allocator, not its event dispatch. Slint uses no display
backend. Iced tests task delivery without a window/executor loop. tui-realm uses
its public command/state types, not a terminal input driver. TachyonFX compares
actual deterministic buffer output, not human visual preference or frame rate.

## The host still owns the transaction

A UI handle is not necessarily a document identity. Store an application-owned
stable ID and revision with the draft. The Vizia allocator example uses a real
recycled ID; the Slint example makes the same point for a row index. The revision
must advance on relevant writes and Undo, even when the value returns to its
original value. These tiny bounded tests use integer increments; production
hosts must handle revision exhaustion and the full document identity.

`EditSession` is deliberately not Clone. Put it in an `Option` and `take()` it
for confirmation or Cancel; read previews by reference. Share a handle to the
draft slot where needed, not a mutable pointer inside the value snapshot.
Slint's immutable/copy-on-write `SharedString` is tested explicitly.

Check and apply on the same serialized host path. Do not compute a `Change`,
await a task, then apply it later without checking again. The Iced recipe queues
the session itself and calls `finish` only when its message is delivered. This
is not distributed concurrency control, storage rollback, an Undo stack, or
proof that an external write succeeded.

## Compare with a local helper

[`helper_comparison.rs`](../helper_comparison.rs) implements both a local draft
and the library adapter against the same host and behavioral checks:

```sh
cargo test --locked --example helper_comparison
```

A local helper is sufficient for one small editor. The reason to consider this
crate is sharing the same validation/Cancel/staleness rules across several
editors, not a capability unavailable in ordinary Rust. These recipes do not
measure reduced code size, speed, adoption or saved maintenance work. No toolkit
is asked to depend on edit-session, and no framework core changes are required.

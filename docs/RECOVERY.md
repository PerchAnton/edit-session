# Retain a draft until the host command succeeds

Run the complete [Rust example](../examples/retry_command.rs):

```sh
cargo run --locked --example retry_command
cargo test --locked --example retry_command
```

The example uses the existing alpha 2 API. Alpha 3 adds documentation and tests,
not a new commit API or a dependency on interaction-check. The library remains
`no_std`; the example host uses `std` for strings and illustrative history vectors.

## The command boundary

`finish(self, ...)` consumes a session even when it returns an error. If the following
host command can be rejected and the dialog should remain open, keep the session and
raw input fields in the host until dispatch has succeeded:

1. `check` the live value and document/target/revision immediately before dispatch.
2. For a checked no-op, close without dispatching a command or clearing Redo.
3. Pass the borrowed `preview()` to the existing host command. That command rechecks
   permissions and rejects atomically, leaving model, revision and history intact.
4. On success, take/drop the draft and close. On failure, retain the draft and raw
   fields; show the error. Every retry starts with step 1 again.

The example deliberately uses `check` and borrows the preview rather than calling
`finish` before a fallible operation. There is no new library method and no session
clone. The host's `Option<EditSession<...>>` is taken only on success or Cancel, so a
second confirmation finds a closed dialog. The application still owns dispatch and
single-commit behavior; a caller bypassing this host flow could dispatch twice.

This sequence must run synchronously on the application's serialized edit path.
`check` is not a lock. An asynchronous command needs an equivalent freshness check
at its actual write boundary. A command that can partially mutate and then fail needs
its own transaction/rollback; retaining a draft does not undo those writes.

## What the eight example tests verify

| Scenario | Required outcome |
| --- | --- |
| Permission denial, then successful retry | Preserve input/model/revision/Undo/Redo on denial; apply once after permission is restored |
| Confirm again after success | Reject as closed; no second history entry |
| Invalid correction, then valid correction | Keep raw invalid text; never silently apply the last valid preview |
| Document or target changes before retry | Reject the old context; do not rebase the draft onto the new target |
| Edit + Undo before retry | Reject the stale revision even when the live value equals the original |
| Live value changes without a revision update | Reject with `ValueChanged` |
| Correct input back to the original value | Close as a no-op; preserve Redo and skip command dispatch |
| Cancel after rejection | Discard only the local draft |

All eight run in the default `cargo test --locked` suite (`test = true` on the example).
They are model/example tests, separate from the 16 library lifecycle tests and one
compile-fail doctest. They are **not** eight new regressions against real PhotoCraft or
PdfCraft widgets. The integration review that motivated recovery still needs its own
consumer patch and UI regression.

If the context is stale, this example leaves the fields visible for inspection and
refuses confirmation. A real UI should dismiss or present an explicit conflict action;
it must not silently reuse the proposal for a different document or target.

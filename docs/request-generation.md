# Request generation and draft context

A background job has two independent questions:

1. Does this result belong to the request still shown by the UI?
2. Is the document still the one on which the draft was based?

[`request_generation.rs`](../examples/request_generation.rs) uses a host-owned
monotonic ticket for the first question and `EditSession` context for the second.
Cancel closes the request. Starting another request gets a new ticket, even if
its path and initial value are identical. Both stale successes and stale errors
are discarded before they can change the replacement draft. A matching result
cannot authorize a later commit after a document switch or Undo.

```sh
cargo test --locked --example request_generation
cargo run --locked --example request_generation
cargo test --locked --example helper_comparison
```

Eight deterministic model tests cover cancellation, replacement, late success,
late failure, duplicate results, document/revision changes, no-op confirmation,
and counter exhaustion. No worker, filesystem, archive installer or application
GUI runs here. These tests are not evidence that a third-party application bug
has been fixed.

The essential request filter can be a small local enum and counter. Edit-session
does not manage worker cancellation, route result messages, close host
transactions, refresh a painter's render cache, or roll back external effects.
Use the existing host command/history engine for those jobs. If stale worker
results are the only problem, a new dependency is unnecessary.

For a draft that also needs validation, preview and context checks, the library
can provide that separate lifecycle. [`helper_comparison.rs`](../examples/helper_comparison.rs)
checks the same commit/Cancel behavior with a local helper and with edit-session.
[`retry_command.rs`](../examples/retry_command.rs) shows how to retain a draft
when the host rejects a command. No performance or code-size advantage is claimed.

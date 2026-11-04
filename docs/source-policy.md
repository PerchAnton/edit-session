# Dependency sources and a local-helper alternative

Correctness and source approval are separate gates. A commit pin fixes content;
it does not show that the receiving organisation reviewed or accepted that source.

## PhotoCraft proposal: current obstacle

At integration head [`df2b3a7`](https://github.com/PerchAnton/photocraft/commit/df2b3a722660d72634fb445ef860b0aca0d4d58b),
[PhotoCraft #9190](https://github.com/storytold/photocraft/pull/9190) still pins
edit-session alpha.1 to an external Git source. The exception has been removed,
but the unchanged **Git dependencies come from storytold** gate rejects that source.
The old standalone interaction workflow is also absent from this head. A previous
successful CI run does not establish that the current dependency is acceptable.

The [maintainer review](https://github.com/storytold/photocraft/pull/9190#pullrequestreview-5727709694)
requests an organisation-owned source or an explicitly reviewed vendor patch with
provenance. No such source decision has been granted. Do not change the source gate,
hide a dependency with Cargo configuration, or switch to crates.io merely because
the Git-only check would stop detecting it.

| Option | Concrete preparation | Remaining decision / cost |
| --- | --- | --- |
| Organisation-owned component | Propose the existing small lifecycle to an approved repository; pin the revision after review | Maintainers choose ownership, location and maintenance responsibility; no URL or approval is assumed |
| Reviewed vendor patch | Keep source, licence, exact upstream revision and a numbered patch with a `vendor/README.md` entry | Maintainers must approve the vendor layout and update process; passing a gate alone is insufficient |
| Application-local helper | Use the native lifecycle demonstrated below; remove the library entries from workspace/UI manifests and regenerate the lockfile | No external source for this lifecycle, but the host owns its tests, updates and command integration |

The local helper is the immediate dependency-free comparison option. These are
preparation options, not an approved change to PhotoCraft. Full host tests, layers,
wasm and unchanged source checks must run on the actual chosen application patch.
The separate interaction-check runner remains a separate CI-policy decision.

## Reproduce the local-helper comparison

```sh
cargo test --locked --example helper_comparison
cargo run --locked --example helper_comparison
```

[The example](../examples/helper_comparison.rs) exercises both a small in-tree helper
and the existing EditSession API through the same synchronous host. It reuses the
Cancel and context/revision cases; it adds no library API or runtime dependency.

| Scenario | Required for both implementations |
| --- | --- |
| Cancel after another edit | Preserve the host value and all history counters |
| Same value, different object | Reject the old context; perform no command |
| Same value, different document | Reject the old context; perform no command |
| Edit then Undo to original value | Reject the changed revision; retain Redo |
| Changed value without revision | Reject the changed live value |
| Invalid input after a valid preview | Prevent committing the previous preview |
| Confirm unchanged draft | Preserve Redo and avoid an empty command |
| Confirm valid changed draft | Dispatch exactly one host update |

Eight tests execute both implementations: sixteen scenario/implementation checks.
This is a teaching model, not a rerun of PhotoCraft's UI integration. Host identity,
monotonic revision, permissions, parsing, dialog lifetime and serialized command
execution remain necessary in either option. The helper intentionally implements
only this example's lifecycle; it is not a replacement for every library method.

Equivalent results establish no code-size or performance advantage for the crate.
If a small local helper meets the host's needs and source policy, retaining it is a
valid outcome. Consider the library only if reuse across real dialogs offsets its
approval and maintenance cost. The existing [Cancel/retry example](../examples/retry_command.rs)
and [Undo/context walkthrough](../examples/context_walkthrough.rs) remain available.

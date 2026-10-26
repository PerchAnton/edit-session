# When edit-session earns its dependency

The useful behavior is protecting an in-progress edit from a changed document, target
or revision, including an edit followed by Undo to the same original value. Preview
alone is not a reason to add this library: many editors already preview and cancel well.

## A demonstrated failure in a real consumer

The proposed [PdfCraft Document Properties integration](https://github.com/storytold/pdfcraft/pull/4594)
has two regression tests against real egui widgets and the existing document engine:

| Workflow | Baseline behavior | Proposed guarded behavior |
| --- | --- | --- |
| Type "Old draft", apply "Newer title" elsewhere, confirm the old dialog | Overwrites the newer title with "Old draft" | Dismisses the stale dialog; keeps the newer title and revision |
| Type "Old draft", apply another edit, Undo, confirm the old dialog | Applies the stale draft and loses the Redo branch | Rejects the stale revision; preserves the original value and Redo |

Baseline `418ac6a2af49b7ac2fed44ca2cae8d5d1d2cec0d`: **0/2 pass**. Proposed consumer
`535d990d10832baf3b0a64910c51b280c224bd5c`: **2/2 pass**. These are the recorded
25 October before/after checks, not newly discovered bugs or a survey of every dialog.
The [tests](https://github.com/PerchAnton/pdfcraft/blob/535d990d10832baf3b0a64910c51b280c224bd5c/crates/ui-egui/tests/it/properties_session.rs)
and [consumer reproduction instructions](https://github.com/PerchAnton/pdfcraft/blob/535d990d10832baf3b0a64910c51b280c224bd5c/docs/document-properties-sessions.md)
make the behavior reviewable. [Validation details](VALIDATION.md) record the environment
and broader 10-scenario / 79-step widget/engine checks.

This benefit comes from the complete consumer patch: the host supplies revision changes,
document identities, permissions, dialog dismissal and serialized command dispatch.
The library checks the context/value and returns at most one change. A correct in-tree
helper can provide the same protection. Passing these tests does not establish that an
external library is preferable to that helper.

## Count the cost as well

At the pinned consumer revision above, the production UI diff contains **267 added /
50 removed lines (net +217)** across `dialogs.rs`, `lib.rs` and `properties_session.rs`.
These are physical Git diff lines, including comments/formatting; tests, examples,
manifests, lockfiles and documentation are excluded. This is added behavior and host
integration, **not demonstrated code reduction**. Reproduce in a PdfCraft checkout
containing the integration commit:

```sh
git diff --numstat \
  418ac6a2af49b7ac2fed44ca2cae8d5d1d2cec0d...535d990d10832baf3b0a64910c51b280c224bd5c \
  -- crates/ui-egui/src
```

The library has **0 runtime dependencies**, uses `no_std` and forbids unsafe code.
An existing instrumented test performs **999 proposal/check pairs with 0 additional
library-triggered snapshot clones** after the one clone at construction. Reset adds
one clone; finishing adds none. This counts calls to `T::clone`, not allocations inside
the caller's proposal, `Clone`/`PartialEq` implementation, CPU time or memory saved versus
a native helper. It is not a speed claim. Reproduce:

```sh
cargo test --locked proposing_and_checking_do_not_clone_snapshots_per_event
```

On 26 October, all **16 lifecycle tests + 1 compile-fail doctest**, formatting and strict
Clippy pass locally on Rust 1.99.1/macOS arm64. The API remains alpha.2 with MSRV 1.88;
the current local check does not replace the CI MSRV check. No API growth was needed
to address these adoption questions.

## What feedback changes our approach

- [Oculante](https://github.com/woelper/oculante/pull/799#issuecomment-6400163844)
  already has a largely non-destructive workflow. Wait for its UI rewrite and demonstrate
  a specific missing guarantee before suggesting another dependency.
- [SSBH Editor](https://github.com/ScanMountGoat/ssbh_editor/issues/216#issuecomment-6400206516)
  already offers preview and workspace reload. There is no demonstrated regression here
  that justifies replacing that workflow.
- [Tarsier](https://github.com/Its-Just-Nans/tarsier/issues/2#issuecomment-6400421930)
  prefers its backup/reset approach and closed the proposal. Respect that decision.
- [PdfCraft's review](https://github.com/storytold/pdfcraft/pull/4594#pullrequestreview-5641786055)
  identifies a real dependency-policy cost. Both the runtime library and executing the
  external replay tool in CI need policy-owner approval; immutable pins do not grant it.

## Decision rule for the next integration

1. Reproduce a stale edit, no-op history loss or duplicated lifecycle logic in the host.
2. Add a regression using the host's native test facilities before changing behavior.
3. Compare a small local helper with the library under the same tests. Include the host
   bridge, dependency review and maintenance cost in the comparison.
4. Use the external library only when shared tested behavior across consumers earns that
   cost. A local or ecosystem-owned helper is a valid outcome.

PhotoCraft and PdfCraft are proposed consumers, not adopted dependencies. We have
demonstrated behavior protection in a consumer patch; we have not demonstrated lower
total source size or faster editing. The next valuable milestone is a maintainer-reviewed
workflow and ownership decision, not more adapters or a larger API.

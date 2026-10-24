# Two candidate projects for PhotoCraft and PdfCraft

Research date: 24 October 2026 (Europe/Kyiv). The implementation is an experiment; no
maintainer has agreed to adopt either project. The first consumer focus is PhotoCraft,
followed by PdfCraft.

## What the repositories show

PhotoCraft and PdfCraft are Rust applications using egui 0.36 and pinned dependencies from
[craft-libs](https://github.com/storytold/craft-libs). Bubbles is a Go component library;
delta is a Rust terminal application that depends on bat. They are separate contribution
targets, not one shared runtime stack. A single new Rust GUI dependency would not fit all
of them automatically.

The Craft ecosystem already shares atomic saves, settings, printing, keymaps and UI
components. Building replacements without a demonstrated missing capability would have
a high maintenance/adoption cost. See the two application manifests:
[PhotoCraft](https://github.com/storytold/photocraft/blob/14a4ab9b3f38f0b68fde6add97384919710374c8/Cargo.toml),
[PdfCraft](https://github.com/storytold/pdfcraft/blob/a7801dc84e74dd12757aecbb6be324dce7a33939/Cargo.toml).

The [shared UI audit](https://github.com/storytold/craft-libs/blob/main/docs/ui-components-audit.md#numeric-controls)
identifies edit buffers, commit/cancel/drag lifecycle and caller-owned undo as a useful shared
boundary. Its initial numeric-control examples are FilmCraft/EffectCraft/LightCraft, **not
proof that PhotoCraft/PdfCraft have agreed to the same API**. [Issue 67](https://github.com/storytold/craft-libs/issues/67)
and [issue 77](https://github.com/storytold/craft-libs/issues/77) also show active work in this
area. Some discussion comments contain incorrect repository references and are corrected
later in the thread; source code, not the original comment, is the reliable implementation evidence.

## Candidate 1: edit-session

Value proposition: provide one small, toolkit-independent lifecycle for a draft edit, with
host-supplied validation and document/target/revision checks. Preview does not enter live
history; commit returns one proposed change; cancel returns none; no-ops preserve redo.

The strongest initial product workflow found is
[PhotoCraft #8414](https://github.com/storytold/photocraft/issues/8414): live preview of Select
> Modify (Border, Smooth, Expand, Contract, Feather), Cancel restoring the original selection,
and OK committing the preview. The issue was open/unassigned at the check, with one bot
comment. A targeted PR-number search returned no matches; this is not an exhaustive guarantee
that nobody is implementing equivalent work under another title.

PhotoCraft already has [adjustment previews](https://github.com/storytold/photocraft/blob/14a4ab9b3f38f0b68fde6add97384919710374c8/crates/ui-egui/src/adjust_preview.rs)
and a [bounded preview worker](https://github.com/storytold/photocraft/blob/14a4ab9b3f38f0b68fde6add97384919710374c8/crates/ui-egui/src/filter_preview_worker.rs).
Reuse those rendering/worker facilities where appropriate. The new draft type does not replace
their pixel computation, cancellation scheduling, cache keys or selection-mask semantics.
Whether a separate dependency improves that implementation must be measured in a real patch.

For PdfCraft, [#3846](https://github.com/storytold/pdfcraft/issues/3846) asks for a manual fine-angle
rotation control with preview and undo. It was open/unassigned with one bot comment; the
targeted PR-number search found no matching PR. This is a later consumer candidate, not a
small integration: page bounds, clipping, annotations, links and save/reopen fidelity require
separate engine work. A draft library does not solve those geometry problems.

An even narrower future PdfCraft pilot may be preferable after inspecting a current dialog.
Its [existing text-preview API](https://github.com/storytold/pdfcraft/blob/a7801dc84e74dd12757aecbb6be324dce7a33939/crates/engine/src/text_preview.rs)
already leaves bytes, dirty state and undo unchanged; preserve that behavior. Do not use
the prototype as a justification to replace a working preview mechanism.

The initial opacity probe used real PhotoCraft Document/History types. The published adapter
now lives in `interaction-check` and uses the real Session command façade with this library
as an immutable Git dependency. It still does not exercise the GUI or reproduce/fix #7601
or #8414. Upstream manifests have not been changed.

## Candidate 2: interaction-check

Value proposition: share small, reviewable semantic traces and assertions across development
environments. A useful bug report can include an executable sequence with an explicit result
after each step: drag, cancel, switch target, undo, redo, inspect.

The initial JSONL runner is independent of the library. Its first adapter uses the library
plus actual PhotoCraft components. Future adapters should use existing application command
channels or test harnesses, not add a new remote-control protocol to the shipped apps.

PhotoCraft documents [a JSON control channel](https://github.com/storytold/photocraft/blob/14a4ab9b3f38f0b68fde6add97384919710374c8/docs/control-protocol.md);
PdfCraft documents [stdio MCP](https://github.com/storytold/pdfcraft/blob/a7801dc84e74dd12757aecbb6be324dce7a33939/docs/mcp.md).
The protocols have different execution contexts; PdfCraft's MCP process does not control a
running desktop app. A headless command trace is not evidence of pointer, IME or rendering correctness.

[egui_kittest](https://github.com/emilk/egui/tree/main/crates/egui_kittest) already tests widgets,
accessibility and snapshots. [teatest](https://charm.sh/blog/teatest/) tests Bubble Tea programs.
PdfCraft already uses proptest and has a separate [Hegel pilot proposal](https://github.com/storytold/pdfcraft/issues/1187).
The contribution here is trace portability and evidence, not a replacement testing framework.

## Alternatives considered

| Direction | Evidence | Decision |
| --- | --- | --- |
| Complete new GUI toolkit | egui and craft-ui already underpin the apps | Too much replacement and migration for this goal |
| Command palette / fuzzy matcher | Both apps have palettes; egui-palette, egui-command and nucleo exist | We did not find a sufficiently distinct first capability |
| Atomic save / settings / printing | Already provided by craft-libs | Extend existing components for concrete missing behavior |
| Draft editing lifecycle | Repeated preview/commit/cancel responsibilities; real feature requests | First small prototype, adoption conditional on a real consumer patch |
| Portable semantic replay | Existing harnesses are useful; cross-project scenario exchange is a separate seam | Second prototype, start as a companion and prove independent reuse |

Alternative references: [egui-palette](https://docs.rs/egui-palette/latest/egui_palette/),
[egui-command](https://docs.rs/egui-command), [nucleo](https://github.com/helix-editor/nucleo),
[undo](https://docs.rs/undo/latest/undo/). An undo history crate and this draft lifecycle solve
different parts of the flow; the application's current history must remain authoritative.

## Adoption gates

1. Characterize the current PhotoCraft Select > Modify workflow with its existing UI harness;
   inspect fresh issue/PR activity and prove the missing behavior at a pinned source revision.
2. Implement one previewed operation with tests for cancellation, document/selection changes,
   stale worker results, one undo step, redo and no-op behavior. Verify the actual rendered
   selection and final pixels, not only parameter values. Cover applicable modes and depths.
3. Compare the patch using the library with a local helper. A reusable dependency must remove
   enough duplication and provide enough tested behavior to justify its maintenance cost.
4. Build a second real PdfCraft adapter before calling the API generally reusable. Keep
   app-specific commands, rendering, geometry and policy outside the core.
5. The independent repositories are `PerchAnton/edit-session` and `PerchAnton/interaction-check`,
   licensed MIT OR Apache-2.0. Propose a narrow reviewable integration; maintainers may prefer
   a craft-libs component or an in-tree helper. Adoption is
   their decision, and both prototypes should remain useful without requiring a framework migration.

The two source directories are independently runnable prototypes, not two finished products.
Prioritize the PhotoCraft contribution; grow the replay tool only when actual scenarios need it.

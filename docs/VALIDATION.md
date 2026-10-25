# Validation evidence

## PdfCraft consumer proposal — 25 October 2026

[PR #4594](https://github.com/storytold/pdfcraft/pull/4594), consumer commit `535d990d10832baf3b0a64910c51b280c224bd5c` on upstream
`418ac6a2af49b7ac2fed44ca2cae8d5d1d2cec0d`, uses edit-session alpha 2 at
`033f7c3a67b825b8f222288788cc61234c0c945d` and interaction-check at
`7722d66c0ede7855ff58d1df3d1632fd4e5e66a5`.

On macOS arm64, Rust 1.99.1, Python 3.14.3, English UI and system-font fallback disabled:

- Two upstream UI regressions fail before the fix: confirming a stale title after another
  edit, and confirming after edit + Undo back to the same original value.
- 522 unit, 1,153 integration and 3 performance tests pass for `pdfcraft-ui-egui`; 11
  existing tests are ignored. Tests run with two threads and host loopback/GPU access.
- Formatting, strict Clippy on all UI targets, layering, asset attribution and parity pass.
- `xtask wasm` checks 31 crates successfully, with warnings in unchanged platform-specific code.
- 10 JSONL scenarios / 79 steps pass through actual egui controls and the real engine.
  They cover commit, Cancel/Escape, no-op preserving Redo, custom validation, stale edits,
  Undo ABA, reload, target switching/closure, clean inspection refresh, and save/reopen.
  JSON and JUnit reports were generated. Actual Description, Custom, Initial View and
  stale-notice screenshots were inspected locally; synthetic PDF bytes are not committed.
- Engine refusal/retry is atomic. A separate test checks freshness immediately at commit,
  without relying on a preceding frame. Pending input is not rebased by an inspection tab.

The [consumer documentation](https://github.com/PerchAnton/pdfcraft/blob/535d990d10832baf3b0a64910c51b280c224bd5c/docs/document-properties-sessions.md)
explains replay and the proposed exact Git-source exception. These results are local;
Linux/Windows CI, native OS input, another PDF reader, packaging and maintainer adoption
are not implied. PhotoCraft's existing alpha 1 pin is unchanged.

## Alpha 2 review — 25 October 2026

On macOS arm64 with **Rust 1.88.0**, the minimum supported compiler:

- `cargo test --locked --offline`: **14 lifecycle tests and 1 compile-fail doctest passed**.
- `cargo fmt --check` and `cargo clippy --locked --offline --all-targets -- -D warnings`: passed.
- Tests cover keeping invalid input editable through `check_live`, conflict precedence,
  Reset after invalid input, a no-op after Reset, continued editing and rejection of stale
  context/live values after Reset. A non-Clone context remains supported.
- Standard error reporting works through `core::error::Error`; no dependencies or `std`
  requirement were introduced. Existing single-use `finish` behavior is unchanged.

The companion runner's alpha 2 changes also passed 18 scenarios / 85 steps against the
previously built PhotoCraft engine and UI adapters. Those binaries still contain the
**alpha 1 library pin**: this is runner/protocol compatibility evidence, not a rebuilt
PhotoCraft integration of the new Rust API. See the
[runner validation record](https://github.com/PerchAnton/interaction-check/blob/main/docs/VALIDATION.md).

The existing PhotoCraft PR is ready for review. Adoption and any change to its pinned
dependencies remain separate from this library review.

## Initial component prototype

Checked 24 October 2026, 22:35 Europe/Kyiv, macOS arm64. Rust 1.99.1; Python 3.14.

| Check | Result |
| --- | --- |
| Core `cargo test --locked` | 9 integration tests and 1 compile-fail doctest passed |
| Core `cargo fmt --check` | Passed |
| Core `cargo clippy --locked --all-targets -- -D warnings` | Passed |
| PhotoCraft probe build, locked dependencies | Passed |
| PhotoCraft probe Clippy, all targets, warnings denied | Passed |
| Runner `python3 -m unittest` | 15 tests passed |
| Real PhotoCraft Document/History component probe | 8 scenarios, 41 event/reply steps passed |
| Deliberately disabled context guard in temporary core copy | 3 scenarios failed as expected; 5 remained passing |
| Correct original probe replay after mutation experiment | All 8 scenarios passed |

PhotoCraft dependency revision:
`14a4ab9b3f38f0b68fde6add97384919710374c8` (version 0.15.1).
Cargo.lock fixes the resolved transitive dependencies. No upstream application files were modified.

The mutation removed only the document/target/revision comparison. The stale-target test
then observed opacity 0.5 on the wrong target instead of unchanged 1.0 and one extra undo
entry. The undo-back-to-original test observed an unintended commit that also discarded redo.
The third failing test observed a different error category. These are deliberately injected
prototype defects; they are not newly discovered bugs in PhotoCraft.

Reports are in the sibling `interaction-check` project:
[passing scenarios](https://github.com/PerchAnton/interaction-check/blob/main/photocraft-report.json) and
[expected mutation failures](https://github.com/PerchAnton/interaction-check/blob/main/mutation-report.json).
The passing report includes environment, upstream revision, adapter scope and executable hash.

## Public repository and command integration checks

The public library's [first CI run](https://github.com/PerchAnton/edit-session/actions/runs/40126658769)
passed tests, formatting and Clippy on **Rust 1.88.0 and stable**, Linux. The runner's
[first CI run](https://github.com/PerchAnton/interaction-check/actions/runs/40126646266)
passed its 15 tests on Python 3.10 and 3.14, Linux.

The adapter now lives in `interaction-check/adapters/photocraft`. It fetches `edit-session`
from GitHub at `28620b8264660ea84a182110cf5894da90281c42` and uses PhotoCraft's real `Session`:
`layer.setProps` on commit, Session target selection, Undo and Redo. It has no path dependency
and never records history directly. The same eight scenarios (41 steps), strengthened with
assertions against the actual command journal count, passed on macOS arm64 with Rust 1.99.1.
The [new report](https://github.com/PerchAnton/interaction-check/blob/main/photocraft-session-report.json)
records the dependency commits and executable hash. The earlier mutation report applies to
the historical component adapter, not to this newer Session adapter.

## Limits of the earlier external adapter

- Native pointer/keyboard interaction, actual rendered previews, accessibility, DPI and IME.
- Integration into PhotoCraft's shipped UI, layer locks/selection-mask semantics, background
  cancellation and persistence. The current adapter only drives a headless Session.
- PdfCraft integration and its PDF geometry, annotations, links and save/reopen behavior.
- Windows and WASM; full application platform checks beyond the adapter.
- Any upstream maintainer acceptance, PR, release or registry package publication.

The SDK, downloaded dependency cache and intermediate build directory were isolated in a
temporary directory. Source, lockfiles, reports and local macOS arm64 probe executables
are retained in the workspace (executables are gitignored). See README for reproduction with a normally installed Rust toolchain.

## Real PhotoCraft UI consumer proposal — 24 October 2026

[PR #9190](https://github.com/storytold/photocraft/pull/9190), head
`46b4a183d5f44035bbc216dcf6196701bcb45bc9`, adds the library to PhotoCraft's UI dependency graph.
Its Edit Selected Guides dialog and canvas use the draft; confirmed edits still run the
existing engine command. The consumer repository owns its
[adapter and scenarios](https://github.com/PerchAnton/photocraft/tree/46b4a183d5f44035bbc216dcf6196701bcb45bc9/tests/interactions/guide-edit),
while CI fetches the separately pinned interaction-check runner.

On macOS arm64, Rust 1.99.1:

- Three real upstream regressions fail before the patch and pass with it: another document,
  reused guide index, and an unchanged confirmation that destroys Redo.
- 10 scenarios / 44 steps pass through the shipped dialog, control setter, egui frames and Session.
- Full UI tests pass with `PHOTOCRAFT_LOCALE=en-US`: 3,798 unit and 132 integration tests;
  19 ignored. The untouched native-language menu-label test fails separately on this Mac's
  ru-UA locale; that limitation is disclosed in the PR.
- Formatting, strict Clippy, architecture layers and WASM pass (24 crates plus HEIF).
- Offscreen snapshots inspected in all 14 themes and three window sizes. A synthetic 24 MP
  guide preview was also exercised. This does not establish general rendering or DPI fidelity.

The PR remains a draft requiring review of the external dependency and its explicit source-policy
exception. Native Windows, PdfCraft, release packaging and maintainer acceptance remain unverified.
Linux scenario CI status is linked from the PR; no pending run is counted as passing here.

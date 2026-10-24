# Validation evidence

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

## What remains unverified

- Native pointer/keyboard interaction, actual rendered previews, accessibility, DPI and IME.
- Integration into PhotoCraft's shipped UI, layer locks/selection-mask semantics, background
  cancellation and persistence. The current adapter only drives a headless Session.
- PdfCraft integration and its PDF geometry, annotations, links and save/reopen behavior.
- Windows and WASM; full application platform checks beyond the adapter.
- Any upstream maintainer acceptance, PR, release or registry package publication.

The SDK, downloaded dependency cache and intermediate build directory were isolated in a
temporary directory. Source, lockfiles, reports and local macOS arm64 probe executables
are retained in the workspace (executables are gitignored). See README for reproduction with a normally installed Rust toolchain.

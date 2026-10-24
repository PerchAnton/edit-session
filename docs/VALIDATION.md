# Validation evidence

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

## What remains unverified

- Native pointer/keyboard interaction, actual rendered previews, accessibility, DPI and IME.
- Production PhotoCraft `Session` command integration, layer locks/selection-mask semantics,
  background cancellation and persistence. The probe deliberately exercises only components.
- PdfCraft integration and its PDF geometry, annotations, links and save/reopen behavior.
- Windows, Linux, WASM and the declared Rust minimum.
- Any upstream maintainer acceptance, PR, release or package publication.

The SDK, downloaded dependency cache and intermediate build directory were isolated in a
temporary directory. Source, lockfiles, reports and the small macOS arm64 probe executable
are retained in the workspace. See README for reproduction with a normally installed Rust toolchain.

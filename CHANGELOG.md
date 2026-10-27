# Changelog

## 0.1.0-alpha.5

Examples and documentation release. Runtime behavior is unchanged.

- Add a runnable context walkthrough: a host edit followed by Undo restores the
  value but invalidates an old draft; Reset does not rebase its captured context.
- Generate a front-page diagram from the example's output and source hashes;
  verify both on Rust 1.88 and stable in CI. Include the renderer in source packages.
- Put concrete behavior, local-helper trade-offs and host responsibilities first in
  the README. Verify the context walkthrough and diagram from the packaged crate
  before upload. Public API, runtime behavior, Rust 1.88 minimum and dependencies
  are unchanged; use a new package version for the added examples and documentation.

## 0.1.0-alpha.4

Published to [crates.io](https://crates.io/crates/edit-session/0.1.0-alpha.4) on
27 October 2026 (Europe/Kyiv; 26 October UTC). A fresh Rust 1.88 consumer installed
the crate from the registry and checked commit, no-op and stale-context behavior.
[Evidence and archive hash](docs/evidence/release-alpha4.json).

- Prepare crates.io publication with an explicit registry allowlist, a package
  landing page, dry-run CI and a manual workflow for later Trusted Publishing.
- Document the one-time token-based first upload required by crates.io.
- Public API, behavior, Rust 1.88 minimum and zero runtime dependencies are unchanged.
  Existing GitHub archives and tags remain unchanged.

## 0.1.0-alpha.3

- Add a runnable host command recovery example and eight native Rust tests covering
  permission denial/retry, raw invalid input, stale context/value, no-op, Cancel and
  duplicate confirmation.
- Document how to retain a draft through a fallible host command using the existing API.
- Verify the example and source crate packaging in CI on Rust 1.88 and stable.
- Public library API and behavior are unchanged from alpha.2. No runtime dependencies.

Published as a GitHub prerelease on 26 October 2026; not published on crates.io.
See the [alpha compatibility policy](docs/RELEASING.md#alpha-compatibility) before upgrading.
Publication does not establish acceptance of a PhotoCraft/PdfCraft integration.

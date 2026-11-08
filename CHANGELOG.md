# Changelog

## 1.0.0 — unreleased

- Commit to the documented API, supported platforms and compatible upgrades throughout 1.x.
- Preserve runtime behavior from 0.1.0 and check existing consumers before publication.
- Correct release instructions to require a new matching tag; retain older artifacts.

## 0.1.0

- First non-prerelease, with an explicit 0.1.x patch compatibility and best-effort
  support policy. Published on 7 November 2026; registry and clean-consumer
  checks are recorded in docs/evidence/release-0.1.0.json.
- The runtime, public types and minimal trait bounds are unchanged from beta.2.
  No stored data or caller migration is required.

## 0.1.0-beta.2

- Compare the crate and an in-tree helper on eight edit scenarios, including Cancel, Undo and changes to the target or revision. Run `cargo test --locked --example helper_comparison` for the executable checks.
- Document dependency source choices for hosts with an approval policy. The host maintainers choose their source.

## 0.1.0-beta.1

Published on 1 November 2026 (Europe/Kyiv) to [crates.io](https://crates.io/crates/edit-session/0.1.0-beta.1)
and as a [GitHub prerelease](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-beta.1).
[Archive hashes and registry-consumer checks](docs/evidence/release-beta1.json).

- Define the supported beta API, error precedence and migration boundary; retain Rust 1.88, no_std and zero dependencies.
- Add downstream-style tests for minimal trait bounds and simultaneous conflicts. Runtime source is unchanged from alpha.5.
- Separate source/workflow approval from package maturity; document what we can supply and what maintainers must decide.

## 0.1.0-alpha.5

Published on 27 October 2026 (Europe/Kyiv) to [crates.io](https://crates.io/crates/edit-session/0.1.0-alpha.5)
and as a [GitHub prerelease](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-alpha.5).
Fresh installation, archive checksums and packaged examples verified. [Evidence](docs/evidence/release-alpha5.json).

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

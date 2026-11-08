# Releases and compatibility

## Next candidate: 1.0.0 (not published)

Follow [the 1.0 checklist](RELEASE-1.0.md). Use a **new** tag matching the reviewed
package version; never reuse v0.1.0. Preparation and CI do not publish a package.


## Current non-prerelease 0.1.0

Published on 7 November 2026 through Trusted Publishing. The exact crate from
crates.io matches the hosted artifact and passed a clean Rust 1.88 consumer.
[Receipt](evidence/release-0.1.0.json) · [GitHub release](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0).

For later releases, require the reviewed
commit to pass the full CI matrix, merge it into main and wait for main CI.
Then create a new matching version tag and use the existing Trusted Publishing
workflow with publish=true. Keep environment approval and archive checks intact.
Do not mark the release published until registry hashes and a clean installation
are verified. Publish the GitHub release with prerelease=false and attach the
exact registry artifacts and checksums. Earlier prereleases stay unchanged.

The [published 0.1.x contract](https://github.com/PerchAnton/edit-session/blob/v0.1.0/docs/COMPATIBILITY.md) remains historical. The [candidate 1.x contract](COMPATIBILITY.md) awaits publication.

## Previous beta 0.1.0-beta.2

Published on 5 November 2026 (Europe/Kyiv). [Package on crates.io](https://crates.io/crates/edit-session/0.1.0-beta.2) · [GitHub prerelease and checksums](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-beta.2) · [Installation checks](evidence/release-beta2.json).

## Previous beta.1

**[0.1.0-beta.1 on crates.io](https://crates.io/crates/edit-session/0.1.0-beta.1)**
was published on **1 November 2026 (Europe/Kyiv)** through
[Trusted Publishing](https://github.com/PerchAnton/edit-session/actions/runs/41309450079).
The [GitHub prerelease](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-beta.1)
attaches the exact checked archive and checksum. Registry and GitHub downloads
match; a fresh no_std registry consumer passed 3 contract and 16 lifecycle tests.
[Receipt](evidence/release-beta1.json) and [beta contract](COMPATIBILITY.md).
Runtime source remains unchanged from alpha.5. Do not upload this version again.

## Previous published prerelease: alpha.5

**[0.1.0-alpha.5 on crates.io](https://crates.io/crates/edit-session/0.1.0-alpha.5)**
was published on **27 October 2026 (Europe/Kyiv)** through Trusted Publishing.
The [GitHub prerelease](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-alpha.5)
attaches the same verified source archive and its checksum. This release adds examples
and documentation; public API, runtime behavior, Rust 1.88 and zero dependencies remain.
A fresh consumer checked commit, no-op, Undo freshness and Reset. Both packaged examples
and the generated context diagram pass. [Evidence](evidence/release-alpha5.json).
Earlier alpha.4 registry files and alpha.3 GitHub assets remain unchanged.

## Historical alpha compatibility

- Alpha releases are experimental. Public types, methods, trait bounds and behavior
  may change between releases, even within `0.1.0`. Alpha.3 matching alpha.2 does not
  promise compatibility with every future alpha.
- Pin a full Git revision (or an exact crate version once published) and keep the
  application's lockfile. Before upgrading,
  read the [changelog](../CHANGELOG.md) and run the host's draft, stale-context,
  no-op and command-retry tests. Library tests do not replace those integration tests.
- Breaking changes, including a raised minimum Rust version, must be called out
  in the changelog with migration guidance. The current minimum is Rust **1.88**.
- Existing release tags and assets must not be replaced; fixes need a new version.

If an upgrade changes a result, [report the old and new revisions with a small test](https://github.com/PerchAnton/edit-session/issues/new?template=bug_report.yml).
A stable 1.0 needs a supported compatibility contract and experience maintaining
real integrations.

## Preparing a future release

Update the version and changelog for the new candidate, then run:

```sh
cargo test --locked
cargo run --locked --example retry_command
cargo run --locked --example context_walkthrough
python3 tools/context_evidence.py --check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo package --locked --list
cargo publish --dry-run --locked --registry crates-io
cargo package --locked
```

Run these from a clean checkout. `cargo package` verifies the extracted crate builds.
It also leaves an archive at the documented path; `cargo publish --dry-run` may keep
its intermediate archive elsewhere, depending on the Cargo version.
The archive includes sources, licenses, tests, examples and their documentation. CI
checks the source and example on Rust 1.88 and stable. To validate packaged tests too,
unpack the new `target/package/edit-session-<version>.crate` into a temporary directory
and run `cargo test --locked` and the example from that directory.

Attach the checked `.crate` archive and SHA-256 checksum to a release on the
exact green commit. Set `prerelease=false` for 1.0.0; use a prerelease only for
versions labelled alpha, beta or rc. The crate is a source archive, not a globally
installed executable. Preserve the existing releases and their assets.

## Registry publication

### Earlier publication: alpha.5

`0.1.0-alpha.5` is **published on crates.io** through
[workflow 40463972786](https://github.com/PerchAnton/edit-session/actions/runs/40463972786).
The API, behavior, Rust minimum and runtime dependencies are unchanged. The new version
includes the added examples and documentation without replacing earlier archives.
`publish = ["crates-io"]` limits the registry target. `PACKAGE.md` is the registry
landing page and uses absolute links so it works outside GitHub.

The registry lists `PerchAnton` as the owner. This version cannot be overwritten;
prepare a new version for any future package change.

### First upload: completed

Alpha.4 was uploaded with the owner's scoped bootstrap token using Cargo 1.88.
The procedure below is retained for reference; do not upload this version again.

Crates.io currently requires an API token for a crate's **first** publication. Trusted
Publishing can only be configured after the crate exists. There is no token in this
repository and the OIDC workflow cannot bootstrap a new crate.

1. Sign in to [crates.io](https://crates.io/) with the intended GitHub owner account
   and verify the email in Account Settings.
2. Once publication is approved, create a short-lived API token restricted to
   publishing the new `edit-session` crate. Enter it through `cargo login --registry
   crates-io` at its private prompt; do not paste it into a chat, command argument or log.
3. Require green CI on the exact release commit and tag it `v0.1.0-alpha.4`. From a
   clean checkout of that tag, inspect the manifest/version, then run:

   ```sh
   cargo publish --dry-run --locked --registry crates-io
   cargo publish --locked --registry crates-io
   ```

4. Verify the registry version, owner, license, Rust minimum and README. Compare its
   downloaded `.crate` SHA-256 with the local archive from the actual upload. Revoke
   the bootstrap token in crates.io and run `cargo logout --registry crates-io`.

The second command uploads permanently; it is deliberately outside the default
rehearsal. Do not run it merely to test credentials or reserve a name.

### Subsequent releases: Trusted Publishing

Trusted Publishing is configured and verified by the alpha.5 upload. Keep these
identity fields unchanged:

| Field | Value |
| --- | --- |
| Repository owner | `PerchAnton` |
| Repository name | `edit-session` |
| Workflow filename | `publish.yml` |
| Environment | `crates-io` |

The existing `crates-io` environment requires owner approval and allows release
tags. These repository files do not configure the account or environment.
The workflow uses the Rust project's OIDC action; no permanent CI token is needed.

Run **Prepare or publish to crates.io** from `main` with `publish=false` (default)
to rehearse: Rust 1.88 tests/Clippy, `cargo publish --dry-run`, packaged tests and the
retry example, plus a saved `.crate` and SHA-256 checksum. Pushes and PRs never upload.
Ordinary CI also checks stable Rust and performs a registry dry-run.

For a later approved version, require green CI and a matching version tag whose
commit is contained in `main`. Run the workflow from that tag with `publish=true`.
It rechecks the package before requesting the short-lived publish token. Do not use
the workflow to re-upload alpha.4 after its manual first publication.

### Verify installation after publication

Use a fresh consumer project and the exact newly published dependency:

```toml
[dependencies]
edit-session = "=<approved-version>"
```

Run `cargo check`, confirm `Cargo.lock` names the crates.io source (not a path/Git
override), and check the [API documentation](https://docs.rs/edit-session/).
Only then mark the README/changelog as published. Never
overwrite a published version; fixes require a new version.

Reference: [Cargo's publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html).
See also [crates.io Trusted Publishing](https://crates.io/docs/trusted-publishing).

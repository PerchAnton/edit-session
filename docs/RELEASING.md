# Releases and alpha compatibility

## Published prerelease

**[v0.1.0-alpha.3](https://github.com/PerchAnton/edit-session/releases/tag/v0.1.0-alpha.3)**
was published on **26 October 2026**, with a source `.crate` archive and SHA-256
checksums. Its public library API and behavior are unchanged from alpha.2; it adds
the tested recovery example, documentation and package verification. The crate is
not published on crates.io; `publish = false` remains explicit.

## Alpha compatibility

- Alpha releases are experimental. Public types, methods, trait bounds and behavior
  may change between releases, even within `0.1.0`. Alpha.3 matching alpha.2 does not
  promise compatibility with every future alpha.
- Pin a full Git revision and keep the application's lockfile. Before upgrading,
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
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo package --locked --list
cargo package --locked
```

Run these from a clean checkout. `cargo package` verifies the extracted crate builds.
The archive includes sources, licenses, tests, examples and their documentation. CI
checks the source and example on Rust 1.88 and stable. To validate packaged tests too,
unpack the new `target/package/edit-session-<version>.crate` into a temporary directory
and run `cargo test --locked` and the example from that directory.

A GitHub prerelease can attach the checked `.crate` archive and SHA-256 checksum to
an exact green commit. It is a source archive, not a globally installed executable.
Consumers can continue using a fixed Git revision while the API is experimental.
Do not present an alpha release as the latest stable release.

## Registry publication

Crates.io publication is a separate decision: confirm name/account ownership, approve
the registry target, change `publish = false` to the intended registry, then run
`cargo publish --dry-run` before an authenticated upload. No registry upload or publish
credentials are configured.

Reference: [Cargo's publishing guide](https://doc.rust-lang.org/cargo/reference/publishing.html).

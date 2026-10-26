# Changelog

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

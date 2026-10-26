# Changelog

## 0.1.0-alpha.3

- Add a runnable host command recovery example and eight native Rust tests covering
  permission denial/retry, raw invalid input, stale context/value, no-op, Cancel and
  duplicate confirmation.
- Document how to retain a draft through a fallible host command using the existing API.
- Verify the example and source crate packaging in CI on Rust 1.88 and stable.
- Public library API and behavior are unchanged from alpha.2. No runtime dependencies.

Experimental prerelease candidate. Package preparation is not crates.io publication
or acceptance of a PhotoCraft/PdfCraft integration.

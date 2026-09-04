# Revero SDKs

Official SDKs, browser recording tools, examples and API documentation for
[Revero](https://revero.no).

## Packages

| Package                               | Directory                | Documentation                                             |
| ------------------------------------- | ------------------------ | --------------------------------------------------------- |
| Rust server SDK (`revero`)            | [`rust/`](rust/)         | [docs.rs/revero](https://docs.rs/revero)                  |
| Browser recorder (`@revero/recorder`) | [`recorder/`](recorder/) | [Recorder guide](https://docs.revero.no/browser-recorder) |

Start with the [five-minute quickstart](https://docs.revero.no/welcome). The
checked-in [`openapi.json`](openapi.json) is the public HTTP contract used to
generate server SDKs and the API reference.

The Rust SDK and recorder are versioned independently. See the
[compatibility and deprecation policy](https://docs.revero.no/versioning).

## Support and security

Use [GitHub Issues](https://github.com/agensdev/revero-sdk/issues) for public
SDK defects and documentation feedback. Report suspected vulnerabilities
privately as described in [SECURITY.md](SECURITY.md).

Copyright Agens AS. Licensed under Apache-2.0.

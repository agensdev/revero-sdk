# Changelog

All notable changes to the Revero Rust SDK are documented here. The SDK follows
[Semantic Versioning](https://semver.org/); while it is below 1.0, release notes
call out every breaking change explicitly.

## 0.2.0 - 2026-11-10

- **Breaking:** webhooks follow [Standard Webhooks](https://www.standardwebhooks.com).
  Revero sends `webhook-id`, `webhook-timestamp` and `webhook-signature`
  (`v1,<base64>` over `{id}.{timestamp}.{body}`) and signs with a `whsec_`
  secret. `webhooks::verify` and `webhooks::verify_and_parse` take a
  `WebhookHeaders` and the `whsec_` secret; `Flyt-Signature`,
  `Flyt-Event-Id` and `Flyt-Event-Type` are gone.
- **Breaking:** the webhook body is `{type, timestamp, data}`. The event id is
  the `webhook-id` header, and the links moved to `data.links`.
  `schema_version`, `id`, `created_at` and the top-level `links` are gone.
- Revero retries every non-2xx response, and timeouts, for about three days. A
  `410` disables the endpoint.

## 0.1.1 - 2026-09-05

- Declare the production and staging servers in the public OpenAPI contract, so
  `Environment::Production` (the default) and `Environment::Staging` carry real
  base URLs. `ClientConfig::default()` previously pointed at the empty string.
- Add `ReveroClientOptions::production` and `ReveroClientOptions::for_environment`.
  `ReveroClientOptions::new` with an explicit base URL is unchanged.

## 0.1.0 - 2026-09-04

- Generate the complete public v1 client and typed models from Revero's
  checked-in OpenAPI contract.
- Add backend Client Credentials authentication with short-lived token caching.
- Add direct upload, bounded operation polling, typed error handling and signed
  webhook verification helpers.

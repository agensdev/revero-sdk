# Reference
## Oauth
<details><summary><code>client.oauth.<a href="/src/api/resources/oauth/client.rs">get_oauth_authorization_server_metadata</a>() -> Result&lt;ClientCredentialsMetadataResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .oauth
        .get_oauth_authorization_server_metadata(None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.oauth.<a href="/src/api/resources/oauth/client.rs">exchange_client_credentials</a>(request: ExchangeClientCredentialsRequest) -> Result&lt;ClientCredentialsTokenResponse, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authenticate with HTTP Basic using the integration client ID and secret. The form must request the exact advertised resource and an explicit space-separated scope set. Client secrets must only be used by a backend.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .oauth
        .exchange_client_credentials(
            &ExchangeClientCredentialsRequest {
                grant_type: ExchangeClientCredentialsRequestGrantType::ClientCredentials,
                resource: "https://referat-api.flyt.test/public/v1".to_string(),
                scope: "meetings:read meetings:write recordings:write".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**grant_type:** `ExchangeClientCredentialsRequestGrantType`

</dd>
</dl>

<dl>
<dd>

**resource:** `String`

</dd>
</dl>

<dl>
<dd>

**scope:** `String` — Space-separated OAuth scopes

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PublicApi
<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">create_public_meeting</a>(request: MeetingCreate) -> Result&lt;MeetingRead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a meeting, idempotent within the OAuth client by external_id.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .create_public_meeting(
            &MeetingCreate {
                external_id: "gm-2026-04-17".to_string(),
                language: None,
                output_profiles: None,
                locked_context: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**external_id:** `String` — Your own identifier for this meeting. Creation is idempotent on it: re-sending an identical request returns the same meeting with 200 instead of 201. Re-using it for a DIFFERENT request (other documents, another language) is a 409 `external_id_conflict`.

</dd>
</dl>

<dl>
<dd>

**language:** `Option<String>` — Spoken language, for transcription and generation.

</dd>
</dl>

<dl>
<dd>

**output_profiles:** `Option<Vec<String>>` — Which documents this meeting should produce, by stable profile key. Each becomes one output resource you fetch separately. The profile VERSION is pinned now: publishing a new version later never changes what this meeting produces. Defaults to the transcript alone.

</dd>
</dl>

<dl>
<dd>

**locked_context:** `Option<Option<std::collections::HashMap<String, serde_json::Value>>>` — Your authoritative facts about this meeting — a case number, participants, a date. Stored encrypted, returned as its own field, and never rewritten by generation: your system stays authoritative for legal and business facts. Captured ONCE, on the request that creates the meeting, so a later call with the same external_id cannot restate it under documents already generated from it. At most 16384 bytes serialized.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">get_public_meeting</a>(meeting_id: String) -> Result&lt;MeetingRead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read one meeting owned by the calling OAuth client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .get_public_meeting(&"meeting_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**meeting_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">prepare_public_recording_upload</a>(meeting_id: String, request: RecordingCreate) -> Result&lt;RecordingPrepared, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a recording and return a one-hour direct-upload request.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .prepare_public_recording_upload(
            &"meeting_id".to_string(),
            &RecordingCreate {
                ..Default::default()
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**meeting_id:** `String`

</dd>
</dl>

<dl>
<dd>

**content_type:** `Option<String>` — MIME type of the audio. The presigned request is issued for exactly this type, and completion refuses an object whose stored type differs — send the same value as the `Content-Type` header on your upload.

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">complete_public_recording_upload</a>(recording_id: String) -> Result&lt;OperationRead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Verify the uploaded object and accept transcription asynchronously.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .complete_public_recording_upload(
            &"recording_id".to_string(),
            Some(RequestOptions::new().additional_header("Idempotency-Key", "Idempotency-Key")),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**recording_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">list_public_meeting_outputs</a>(meeting_id: String) -> Result&lt;Vec&lt;OutputRead&gt;, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

List the documents this meeting declared, and where each one stands.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .list_public_meeting_outputs(&"meeting_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**meeting_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">get_public_meeting_output</a>(output_id: String) -> Result&lt;OutputContentRead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Read one generated output owned by the calling integration.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .get_public_meeting_output(&"output_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**output_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.public_api.<a href="/src/api/resources/public_api/client.rs">get_public_operation</a>(operation_id: String) -> Result&lt;OperationRead, ApiError&gt;</code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Poll the authoritative coarse state of one accepted command.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use revero::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        username: Some("<username>".to_string()),
        password: Some("<password>".to_string()),
        ..Default::default()
    };
    let client = GeneratedReveroClient::new(config).expect("Failed to build client");
    client
        .public_api
        .get_public_operation(&"operation_id".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**operation_id:** `String`

</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

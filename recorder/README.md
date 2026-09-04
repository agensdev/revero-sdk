# `@revero/recorder`

Browser recording and durable direct-upload for Revero integrations. This
package never accepts a Revero client id, client secret, or access token. Its
adapter calls endpoints in your own backend. Rust backends can use the
backend-only `revero` crate in `sdks/revero`; other backends can implement the
same public OpenAPI contract.

## Ready-made component

```tsx
import { IndexedDbRecorderStore } from "@revero/recorder/indexeddb"
import { ReveroRecorder } from "@revero/recorder/react"

const store = new IndexedDbRecorderStore({ databaseName: `gm-${meetingId}` })

<ReveroRecorder
  store={store}
  adapter={{
    prepareRecording: async ({ contentType, sessionId }) =>
      fetch(`/api/meetings/${meetingId}/recordings`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ contentType, sessionId }),
      }).then((response) => response.json()),
    completeRecording: async ({ recordingId, idempotencyKey }) =>
      fetch(`/api/recordings/${recordingId}/complete`, {
        method: "POST",
        headers: { "Idempotency-Key": idempotencyKey },
      }).then((response) => response.json()),
  }}
  labels={{
    start: "Start opptak",
    stop: "Stopp opptak",
    confirm: "Last opp opptaket",
  }}
/>
```

## Custom interface

Use `useReveroRecorder(options)` from `@revero/recorder/react`, or the
framework-neutral `createRecorder(options)` from `@revero/recorder/core`.
Both use the same state machine and `RecorderStore`. Audio chunks are written
to IndexedDB while recording, then uploaded only after `confirmUpload()`.
The chunks remain available for retry until the customer backend has
acknowledged the idempotent completion command, and are then removed while the
submitted operation receipt remains durable.

## Supported environments

| Environment | Support                                                     |
| ----------- | ----------------------------------------------------------- |
| React       | 18 and 19                                                   |
| Next.js SSR | Safe to import; recording starts only in a client component |
| Chromium    | Current and previous stable                                 |
| Firefox     | Current and previous stable                                 |
| Safari      | Current and previous stable                                 |

The package is ESM-only. Browser code receives customer callbacks and
short-lived upload instructions only; it has no API for a Revero OAuth client
secret or service access token.

See the complete recording-to-output guide at <https://docs.revero.no/welcome>.

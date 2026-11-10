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
    again: "Nytt opptak",
  }}
/>
```

Every state renders one control, the submitted state included: `again` discards
the finished recording locally and returns the component to idle, so one mount
records as many times as the person needs. `pause` and `resume` hold one
recording open across a break, which keeps a long meeting as one Revero meeting
with one transcript.

Failures carry a stable `errorCode` beside the human `error` — branch on the
code, never on the message. Restored state carries `restored: true` and the
original `startedAt`. `elapsedMs(state, now)` from `@revero/recorder/core` is a
pure, pause-aware duration, and `controller.stream` is the live `MediaStream`
for a level meter.

The recorder is created once per mount and reads `adapter`, `store` and the
other options at call time, so an inline object literal is safe and a re-render
of the surrounding component cannot restart a recording in progress. Swapping
`store` in place is therefore not a reset either: to record against a different
meeting, remount with a React `key`. Pass `onController` for anything the
component does not render.

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

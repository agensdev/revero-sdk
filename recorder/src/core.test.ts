import { describe, expect, test } from "bun:test"

import { createRecorder, type DurableRecorderState, type RecorderStore } from "./core"

class MemoryStore implements RecorderStore {
  state: DurableRecorderState | null = null
  chunks: Blob[] = []

  async loadState() {
    return this.state
  }

  async saveState(state: DurableRecorderState) {
    this.state = structuredClone(state)
  }

  async appendChunk(_sessionId: string, chunk: Blob) {
    this.chunks.push(chunk)
  }

  async loadChunks(_sessionId: string) {
    return [...this.chunks]
  }

  async clearChunks(_sessionId: string) {
    this.chunks = []
  }

  async clear() {
    this.state = null
    this.chunks = []
  }
}

class FakeMediaRecorder extends EventTarget {
  static isTypeSupported(type: string) {
    return type === "audio/webm;codecs=opus"
  }

  readonly mimeType = "audio/webm;codecs=opus"
  state: RecordingState = "inactive"

  constructor(_stream: MediaStream, _options?: MediaRecorderOptions) {
    super()
  }

  start() {
    this.state = "recording"
    this.dispatchEvent(
      new MessageEvent("dataavailable", {
        data: new Blob(["audio"], { type: this.mimeType }),
      })
    )
  }

  stop() {
    this.state = "inactive"
    this.dispatchEvent(new Event("stop"))
  }
}

describe("headless recorder", () => {
  test("persists chunks before uploading and completes through customer callbacks", async () => {
    const store = new MemoryStore()
    const calls: string[] = []
    const recorder = createRecorder({
      store,
      mediaDevices: {
        getUserMedia: async () => ({
          getTracks: () => [{ stop: () => calls.push("track.stop") }],
        }),
      } as unknown as MediaDevices,
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      fetch: async (input, init) => {
        calls.push(`${init?.method}:${String(input)}`)
        expect(init?.body).toBeInstanceOf(Blob)
        return new Response(null, { status: 200 })
      },
      adapter: {
        prepareRecording: async ({ contentType }) => {
          calls.push(`prepare:${contentType}`)
          return {
            recordingId: "recording-1",
            upload: {
              method: "PUT",
              url: "https://uploads.revero.test/signed",
              headers: { "Content-Type": contentType },
              expiresAt: "2026-08-30T12:00:00Z",
            },
          }
        },
        completeRecording: async ({ recordingId, idempotencyKey }) => {
          calls.push(`complete:${recordingId}:${idempotencyKey}`)
          return { operationId: "operation-1" }
        },
      },
    })

    await recorder.start()
    expect(recorder.getSnapshot().status).toBe("recording")
    await recorder.stop()
    expect(recorder.getSnapshot().status).toBe("stopped")
    expect(store.chunks).toHaveLength(1)

    await recorder.confirmUpload()

    expect(recorder.getSnapshot()).toMatchObject({
      status: "submitted",
      remoteRecordingId: "recording-1",
      operationId: "operation-1",
    })
    expect(calls).toEqual([
      "track.stop",
      "prepare:audio/webm",
      "PUT:https://uploads.revero.test/signed",
      expect.stringMatching(/^complete:recording-1:/),
    ])
    expect(store.chunks).toEqual([])
    expect(store.state?.status).toBe("submitted")
    expect(JSON.stringify(store.state)).not.toContain("clientSecret")
  })

  test("restores interrupted work as an explicit retryable failure", async () => {
    const store = new MemoryStore()
    store.state = {
      status: "uploading",
      sessionId: "session-1",
      contentType: "audio/webm",
    }
    const recorder = createRecorder({
      store,
      adapter: {
        prepareRecording: async () => {
          throw new Error("not called")
        },
        completeRecording: async () => {
          throw new Error("not called")
        },
      },
    })

    await recorder.restore()

    expect(recorder.getSnapshot()).toMatchObject({
      status: "failed",
      sessionId: "session-1",
      error: "Upload was interrupted and can be retried",
    })
    expect(store.state?.status).toBe("failed")
  })

  test("prepares a fresh upload when a restored presigned request expired", async () => {
    const store = new MemoryStore()
    store.state = {
      status: "failed",
      sessionId: "stable-idempotency-key",
      contentType: "audio/webm",
      remoteRecordingId: "expired-recording",
      upload: {
        method: "PUT",
        url: "https://uploads.revero.test/expired",
        headers: { "Content-Type": "audio/webm" },
        expiresAt: "2020-01-01T00:00:00Z",
      },
    }
    store.chunks = [new Blob(["audio"])]
    const prepared: string[] = []
    const recorder = createRecorder({
      store,
      fetch: async (input) => {
        expect(String(input)).toBe("https://uploads.revero.test/fresh")
        return new Response(null, { status: 200 })
      },
      adapter: {
        prepareRecording: async () => {
          prepared.push("fresh")
          return {
            recordingId: "fresh-recording",
            upload: {
              method: "PUT",
              url: "https://uploads.revero.test/fresh",
              headers: { "Content-Type": "audio/webm" },
              expiresAt: "2099-01-01T00:00:00Z",
            },
          }
        },
        completeRecording: async ({ recordingId, idempotencyKey }) => {
          expect(recordingId).toBe("fresh-recording")
          expect(idempotencyKey).toBe("stable-idempotency-key")
          return { operationId: "operation-1" }
        },
      },
    })

    await recorder.restore()
    await recorder.confirmUpload()

    expect(prepared).toEqual(["fresh"])
    expect(recorder.getSnapshot()).toMatchObject({
      status: "submitted",
      remoteRecordingId: "fresh-recording",
      operationId: "operation-1",
    })
  })
})

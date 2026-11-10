import { describe, expect, test } from "bun:test"

import { createRecorder, elapsedMs, type CreateRecorderOptions, type RecorderAdapter } from "./core"
import { MICROPHONE_CONSTRAINTS } from "./microphone"
import { FakeMediaRecorder, MemoryStore, fakeMediaDevices } from "./test-fakes"

function refusingAdapter(): RecorderAdapter {
  return {
    prepareRecording: async () => {
      throw new Error("prepare exploded")
    },
    completeRecording: async () => {
      throw new Error("complete exploded")
    },
  }
}

function preparingAdapter(
  completeRecording: RecorderAdapter["completeRecording"]
): RecorderAdapter {
  return {
    prepareRecording: async ({ contentType }) => ({
      recordingId: "recording-1",
      upload: {
        method: "PUT" as const,
        url: "https://uploads.revero.test/signed",
        headers: { "Content-Type": contentType },
        expiresAt: "2099-01-01T00:00:00Z",
      },
    }),
    completeRecording,
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

describe("pause and resume", () => {
  test("a paused recorder keeps its status and reports the pause", async () => {
    const store = new MemoryStore()
    let now = 1_000
    const recorder = createRecorder({
      store,
      now: () => now,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.start()
    expect(recorder.getSnapshot()).toMatchObject({ status: "recording", paused: false })

    now = 5_000
    await recorder.pause()
    expect(recorder.getSnapshot()).toMatchObject({ status: "recording", paused: true })
    expect(store.state?.paused).toBe(true)

    now = 9_000
    await recorder.resume()
    expect(recorder.getSnapshot()).toMatchObject({ status: "recording", paused: false })

    // Four seconds recorded, four paused, four recorded.
    now = 13_000
    expect(elapsedMs(recorder.getSnapshot(), now)).toBe(8_000)
  })

  test("elapsed time stops advancing while paused", async () => {
    const store = new MemoryStore()
    let now = 0
    const recorder = createRecorder({
      store,
      now: () => now,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.start()
    now = 3_000
    await recorder.pause()

    expect(elapsedMs(recorder.getSnapshot(), 3_000)).toBe(3_000)
    expect(elapsedMs(recorder.getSnapshot(), 60_000)).toBe(3_000)
  })

  test("pausing what is not recording is refused, and so is resuming what is not paused", async () => {
    const store = new MemoryStore()
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await expect(recorder.pause()).rejects.toThrow()
    await recorder.start()
    await expect(recorder.resume()).rejects.toThrow()
  })

  test("a paused recording still stops, and the audio it holds survives", async () => {
    const store = new MemoryStore()
    const stopped: string[] = []
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices(stopped),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.start()
    await recorder.pause()
    await recorder.stop()

    expect(recorder.getSnapshot()).toMatchObject({ status: "stopped", paused: false })
    expect(store.chunks).toHaveLength(1)
    expect(stopped).toEqual(["track.stop"])
  })

  test("discarding a paused recorder releases the microphone", async () => {
    const store = new MemoryStore()
    const stopped: string[] = []
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices(stopped),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.start()
    await recorder.pause()
    await recorder.discard()

    expect(recorder.getSnapshot().status).toBe("idle")
    expect(stopped).toEqual(["track.stop"])
    expect(recorder.stream).toBeUndefined()
  })
})

describe("failures carry a code, not only a sentence", () => {
  // The published reliability guide tells integrators to branch on codes and
  // never parse the message. Until 0.2.0 this package gave them nothing but a
  // message, so the one thing the guide forbids was the only thing available.
  test("a refused microphone becomes state, not a silently swallowed throw", async () => {
    const store = new MemoryStore()
    const denied = new Error("Permission denied")
    denied.name = "NotAllowedError"
    const recorder = createRecorder({
      store,
      mediaDevices: {
        getUserMedia: async () => {
          throw denied
        },
      } as unknown as MediaDevices,
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await expect(recorder.start()).rejects.toThrow()

    expect(recorder.getSnapshot()).toMatchObject({
      status: "idle",
      errorCode: "microphone_denied",
    })
    expect(recorder.getSnapshot().error).toBeTruthy()
  })

  test("a microphone that is present but unusable is a different code", async () => {
    const store = new MemoryStore()
    const busy = new Error("Device in use")
    busy.name = "NotReadableError"
    const recorder = createRecorder({
      store,
      mediaDevices: {
        getUserMedia: async () => {
          throw busy
        },
      } as unknown as MediaDevices,
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await expect(recorder.start()).rejects.toThrow()
    expect(recorder.getSnapshot().errorCode).toBe("microphone_unavailable")
  })

  test("a browser with no supported container is its own code", async () => {
    const store = new MemoryStore()
    class UnsupportedRecorder extends FakeMediaRecorder {
      static override isTypeSupported() {
        return false
      }
    }
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: UnsupportedRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await expect(recorder.start()).rejects.toThrow()
    expect(recorder.getSnapshot().errorCode).toBe("no_supported_format")
  })

  test("each failing step of the upload names itself", async () => {
    const cases: Array<[Partial<CreateRecorderOptions>, string]> = [
      [{ adapter: refusingAdapter() }, "prepare_failed"],
      [
        {
          adapter: preparingAdapter(async () => {
            throw new Error("complete exploded")
          }),
          fetch: (async () => new Response(null, { status: 200 })) as typeof fetch,
        },
        "complete_failed",
      ],
      [
        {
          adapter: preparingAdapter(async () => ({ operationId: "operation-1" })),
          fetch: (async () => new Response(null, { status: 503 })) as typeof fetch,
        },
        "upload_failed",
      ],
    ]

    for (const [overrides, expected] of cases) {
      const store = new MemoryStore()
      const recorder = createRecorder({
        store,
        mediaDevices: fakeMediaDevices([]),
        MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
        adapter: refusingAdapter(),
        ...overrides,
      })
      await recorder.start()
      await recorder.stop()
      await expect(recorder.confirmUpload()).rejects.toThrow()
      expect(recorder.getSnapshot()).toMatchObject({ status: "failed", errorCode: expected })
    }
  })

  test("a recording with no audio is refused before anything is uploaded", async () => {
    const store = new MemoryStore()
    store.state = { status: "stopped", sessionId: "session-1", contentType: "audio/webm" }
    const recorder = createRecorder({
      store,
      adapter: preparingAdapter(async () => ({ operationId: "operation-1" })),
    })

    await recorder.restore()
    await expect(recorder.confirmUpload()).rejects.toThrow()
    expect(recorder.getSnapshot().errorCode).toBe("empty_recording")
  })

  test("restoring interrupted work names which interruption it was", async () => {
    const interrupted = new MemoryStore()
    interrupted.state = { status: "recording", sessionId: "session-1", contentType: "audio/webm" }
    const first = createRecorder({ store: interrupted, adapter: refusingAdapter() })
    await first.restore()
    expect(first.getSnapshot()).toMatchObject({
      status: "stopped",
      errorCode: "recording_interrupted",
      restored: true,
    })

    const halfway = new MemoryStore()
    halfway.state = { status: "uploading", sessionId: "session-1", contentType: "audio/webm" }
    const second = createRecorder({ store: halfway, adapter: refusingAdapter() })
    await second.restore()
    expect(second.getSnapshot()).toMatchObject({
      status: "failed",
      errorCode: "upload_interrupted",
      restored: true,
    })
  })
})

describe("a restored recording says so", () => {
  test("a state that came back from storage is marked, and starting again clears the mark", async () => {
    const store = new MemoryStore()
    store.state = {
      status: "stopped",
      sessionId: "session-1",
      contentType: "audio/webm",
      startedAt: 1_600_000_000_000,
    }
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.restore()
    expect(recorder.getSnapshot()).toMatchObject({ restored: true, startedAt: 1_600_000_000_000 })

    await recorder.discard()
    await recorder.start()
    expect(recorder.getSnapshot().restored).toBeFalsy()
  })

  test("a recorder with nothing stored is not marked as restored", async () => {
    const store = new MemoryStore()
    const recorder = createRecorder({ store, adapter: refusingAdapter() })

    await recorder.restore()

    expect(recorder.getSnapshot()).toMatchObject({ status: "idle" })
    expect(recorder.getSnapshot().restored).toBeFalsy()
  })
})

describe("the live stream is reachable", () => {
  test("a level meter can be attached to the stream the recorder already holds", async () => {
    const store = new MemoryStore()
    const recorder = createRecorder({
      store,
      mediaDevices: fakeMediaDevices([]),
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    expect(recorder.stream).toBeUndefined()
    await recorder.start()
    expect(recorder.stream).toBeDefined()
    await recorder.stop()
    expect(recorder.stream).toBeUndefined()
  })
})

describe("the microphone (#5228)", () => {
  test("opens it with the shared constraints, echo cancellation off", async () => {
    const requests: MediaStreamConstraints[] = []
    const recorder = createRecorder({
      store: new MemoryStore(),
      mediaDevices: {
        getUserMedia: async (constraints: MediaStreamConstraints) => {
          requests.push(constraints)
          return { getTracks: () => [{ stop: () => {} }] }
        },
      } as unknown as MediaDevices,
      MediaRecorder: FakeMediaRecorder as unknown as typeof MediaRecorder,
      adapter: refusingAdapter(),
    })

    await recorder.start()

    expect(MICROPHONE_CONSTRAINTS.echoCancellation).toBe(false)
    expect(requests).toEqual([{ audio: MICROPHONE_CONSTRAINTS }])
  })
})

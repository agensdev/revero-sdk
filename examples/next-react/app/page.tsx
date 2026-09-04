"use client"

import type { DurableRecorderState, RecorderStore } from "@revero/recorder/core"
import { ReveroRecorder } from "@revero/recorder/react"
import { useMemo, useState } from "react"

class MemoryStore implements RecorderStore {
  state: DurableRecorderState | null = null
  chunks: Blob[] = []

  async loadState() {
    return this.state
  }

  async saveState(state: DurableRecorderState) {
    this.state = state
  }

  async appendChunk(_sessionId: string, chunk: Blob) {
    this.chunks.push(chunk)
  }

  async loadChunks(_sessionId: string) {
    return this.chunks
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

  state: RecordingState = "inactive"
  readonly mimeType: string

  constructor(_stream: MediaStream, options?: MediaRecorderOptions) {
    super()
    this.mimeType = options?.mimeType ?? "audio/webm"
  }

  start() {
    this.state = "recording"
  }

  stop() {
    this.state = "inactive"
    const chunk = new Event("dataavailable")
    Object.defineProperty(chunk, "data", {
      value: new Blob(["audio"], { type: this.mimeType }),
    })
    this.dispatchEvent(chunk)
    this.dispatchEvent(new Event("stop"))
  }
}

const mediaDevices = {
  async getUserMedia() {
    return {
      getTracks: () => [{ stop() {} }],
    } as unknown as MediaStream
  },
}

const adapter = {
  async prepareRecording() {
    return {
      recordingId: "recording-1",
      upload: {
        method: "PUT" as const,
        url: "https://upload.example.test/recording-1",
        headers: { "Content-Type": "audio/webm" },
        expiresAt: new Date(Date.now() + 60_000).toISOString(),
      },
    }
  },
  async completeRecording() {
    return {
      operationId: "operation-1",
      operationUrl: "/operations/operation-1",
    }
  },
}

async function upload() {
  return new Response(null, { status: 200 })
}

export default function RecorderFixture() {
  const store = useMemo(() => new MemoryStore(), [])
  const [operationId, setOperationId] = useState<string>()

  return (
    <main>
      <h1>Recorder release fixture</h1>
      <ReveroRecorder
        adapter={adapter}
        store={store}
        mediaDevices={mediaDevices}
        MediaRecorder={FakeMediaRecorder as unknown as typeof MediaRecorder}
        fetch={upload as typeof fetch}
        onSubmitted={setOperationId}
      />
      {operationId && <output aria-label="Submitted operation">{operationId}</output>}
    </main>
  )
}

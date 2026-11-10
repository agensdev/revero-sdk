import type { DurableRecorderState, RecorderStore } from "./core"

export class MemoryStore implements RecorderStore {
  state: DurableRecorderState | null = null
  chunks: Blob[] = []

  loadState = async () => this.state

  saveState = async (state: DurableRecorderState) => {
    this.state = structuredClone(state)
  }

  appendChunk = async (_sessionId: string, chunk: Blob) => {
    this.chunks.push(chunk)
  }

  loadChunks = async (_sessionId: string) => [...this.chunks]

  clearChunks = async (_sessionId: string) => {
    this.chunks = []
  }

  clear = async () => {
    this.state = null
    this.chunks = []
  }
}

export class FakeMediaRecorder extends EventTarget {
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

  pause() {
    this.state = "paused"
    this.dispatchEvent(new Event("pause"))
  }

  resume() {
    this.state = "recording"
    this.dispatchEvent(new Event("resume"))
  }

  stop() {
    this.state = "inactive"
    this.dispatchEvent(new Event("stop"))
  }
}

/** Counts the tracks the controller actually stops, so an orphaned microphone is observable. */
export function fakeMediaDevices(stopped: string[]): MediaDevices {
  return {
    getUserMedia: async () => ({
      getTracks: () => [{ stop: () => stopped.push("track.stop") }],
    }),
  } as unknown as MediaDevices
}

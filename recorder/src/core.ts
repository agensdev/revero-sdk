export type RecorderStatus =
  | "idle"
  | "recording"
  | "stopped"
  | "preparing_upload"
  | "uploading"
  | "completing"
  | "submitted"
  | "failed"

export interface BrowserUploadRequest {
  method: "PUT"
  url: string
  headers: Record<string, string>
  expiresAt: string
}

export interface PreparedRecording {
  recordingId: string
  upload: BrowserUploadRequest
}

export interface SubmittedRecording {
  operationId: string
  operationUrl?: string
}

export interface RecorderAdapter {
  prepareRecording(input: { sessionId: string; contentType: string }): Promise<PreparedRecording>
  completeRecording(input: {
    recordingId: string
    idempotencyKey: string
  }): Promise<SubmittedRecording>
}

export interface DurableRecorderState {
  status: RecorderStatus
  sessionId?: string
  contentType?: string
  remoteRecordingId?: string
  upload?: BrowserUploadRequest
  operationId?: string
  operationUrl?: string
  error?: string
}

export interface RecorderStore {
  loadState(): Promise<DurableRecorderState | null>
  saveState(state: DurableRecorderState): Promise<void>
  appendChunk(sessionId: string, chunk: Blob): Promise<void>
  loadChunks(sessionId: string): Promise<Blob[]>
  clearChunks(sessionId: string): Promise<void>
  clear(): Promise<void>
}

interface MediaDevicesBoundary {
  getUserMedia(constraints: MediaStreamConstraints): Promise<MediaStream>
}

export interface CreateRecorderOptions {
  adapter: RecorderAdapter
  store: RecorderStore
  mediaDevices?: MediaDevicesBoundary
  MediaRecorder?: typeof MediaRecorder
  fetch?: typeof fetch
  mimeTypes?: readonly string[]
}

export interface RecorderController {
  getSnapshot(): DurableRecorderState
  subscribe(listener: () => void): () => void
  restore(): Promise<void>
  start(): Promise<void>
  stop(): Promise<void>
  confirmUpload(): Promise<void>
  discard(): Promise<void>
}

const DEFAULT_MIME_TYPES = ["audio/webm;codecs=opus", "audio/mp4"] as const

class BrowserRecorderController implements RecorderController {
  readonly #options: CreateRecorderOptions
  readonly #listeners = new Set<() => void>()
  #snapshot: DurableRecorderState = { status: "idle" }
  #recorder?: MediaRecorder
  #stream?: MediaStream
  #chunkWrites: Promise<void> = Promise.resolve()

  constructor(options: CreateRecorderOptions) {
    this.#options = options
  }

  getSnapshot = (): DurableRecorderState => this.#snapshot

  subscribe = (listener: () => void): (() => void) => {
    this.#listeners.add(listener)
    return () => this.#listeners.delete(listener)
  }

  async restore(): Promise<void> {
    const stored = await this.#options.store.loadState()
    if (!stored) return
    if (stored.status === "recording") {
      await this.#set({
        ...stored,
        status: "stopped",
        error: "Recording was interrupted before it was stopped",
      })
      return
    }
    if (["preparing_upload", "uploading", "completing"].includes(stored.status)) {
      await this.#set({
        ...stored,
        status: "failed",
        error: "Upload was interrupted and can be retried",
      })
      return
    }
    this.#publish(stored)
  }

  async start(): Promise<void> {
    if (this.#snapshot.status !== "idle") {
      throw new Error(`Cannot start while recorder is ${this.#snapshot.status}`)
    }
    const mediaDevices = this.#options.mediaDevices ?? navigator.mediaDevices
    const Recorder = this.#options.MediaRecorder ?? globalThis.MediaRecorder
    if (!mediaDevices || !Recorder) {
      throw new Error("Browser audio recording is not available")
    }
    const contentType = this.#selectMimeType(Recorder)
    const sessionId = crypto.randomUUID()
    await this.#options.store.clear()
    this.#stream = await mediaDevices.getUserMedia({ audio: true })
    this.#recorder = new Recorder(this.#stream, { mimeType: contentType })
    this.#recorder.addEventListener("dataavailable", (event) => {
      const chunk = (event as BlobEvent).data
      if (chunk.size === 0) return
      this.#chunkWrites = this.#chunkWrites.then(() =>
        this.#options.store.appendChunk(sessionId, chunk)
      )
    })
    await this.#set({ status: "recording", sessionId, contentType })
    this.#recorder.start(1_000)
  }

  async stop(): Promise<void> {
    const recorder = this.#recorder
    const sessionId = this.#snapshot.sessionId
    const contentType = this.#snapshot.contentType
    if (this.#snapshot.status !== "recording" || !recorder || !sessionId || !contentType) {
      throw new Error("Recorder is not recording")
    }
    const stopped = new Promise<void>((resolve) =>
      recorder.addEventListener("stop", () => resolve(), { once: true })
    )
    recorder.stop()
    await stopped
    await this.#chunkWrites
    for (const track of this.#stream?.getTracks() ?? []) track.stop()
    this.#recorder = undefined
    this.#stream = undefined
    await this.#set({ status: "stopped", sessionId, contentType })
  }

  async confirmUpload(): Promise<void> {
    const sessionId = this.#snapshot.sessionId
    const declaredContentType = this.#snapshot.contentType
    if (
      !sessionId ||
      !declaredContentType ||
      !["stopped", "failed"].includes(this.#snapshot.status)
    ) {
      throw new Error("Recording must be stopped before upload is confirmed")
    }
    const contentType = declaredContentType.split(";", 1)[0]!
    let acknowledged = false
    try {
      let remoteRecordingId = this.#snapshot.remoteRecordingId
      let upload = this.#snapshot.upload
      if (upload && Date.parse(upload.expiresAt) <= Date.now()) {
        remoteRecordingId = undefined
        upload = undefined
      }
      if (!remoteRecordingId || !upload) {
        await this.#set({ ...this.#snapshot, status: "preparing_upload", error: undefined })
        const prepared = await this.#options.adapter.prepareRecording({
          sessionId,
          contentType,
        })
        remoteRecordingId = prepared.recordingId
        upload = prepared.upload
      }

      const chunks = await this.#options.store.loadChunks(sessionId)
      if (chunks.length === 0) throw new Error("Recording contains no audio")
      await this.#set({
        ...this.#snapshot,
        status: "uploading",
        remoteRecordingId,
        upload,
        error: undefined,
      })
      const response = await (this.#options.fetch ?? globalThis.fetch)(upload.url, {
        method: upload.method,
        headers: upload.headers,
        body: new Blob(chunks, { type: contentType }),
      })
      if (!response.ok) {
        throw new Error(`Audio upload failed with HTTP ${response.status}`)
      }

      await this.#set({ ...this.#snapshot, status: "completing" })
      const submitted = await this.#options.adapter.completeRecording({
        recordingId: remoteRecordingId,
        idempotencyKey: sessionId,
      })
      await this.#set({
        ...this.#snapshot,
        status: "submitted",
        operationId: submitted.operationId,
        operationUrl: submitted.operationUrl,
        error: undefined,
      })
      acknowledged = true
      await this.#options.store.clearChunks(sessionId)
    } catch (error) {
      if (acknowledged) {
        await this.#set({
          ...this.#snapshot,
          error: "Recording was submitted, but local audio cleanup failed",
        })
        return
      }
      await this.#set({
        ...this.#snapshot,
        status: "failed",
        error: error instanceof Error ? error.message : "Recording upload failed",
      })
      throw error
    }
  }

  async discard(): Promise<void> {
    if (this.#recorder?.state === "recording") this.#recorder.stop()
    for (const track of this.#stream?.getTracks() ?? []) track.stop()
    this.#recorder = undefined
    this.#stream = undefined
    await this.#options.store.clear()
    this.#publish({ status: "idle" })
  }

  #selectMimeType(Recorder: typeof MediaRecorder): string {
    const candidates = this.#options.mimeTypes ?? DEFAULT_MIME_TYPES
    const selected = candidates.find((type) => Recorder.isTypeSupported(type))
    if (!selected) throw new Error("No supported audio recording format was found")
    return selected
  }

  async #set(next: DurableRecorderState): Promise<void> {
    await this.#options.store.saveState(next)
    this.#publish(next)
  }

  #publish(next: DurableRecorderState): void {
    this.#snapshot = next
    for (const listener of this.#listeners) listener()
  }
}

export function createRecorder(options: CreateRecorderOptions): RecorderController {
  return new BrowserRecorderController(options)
}

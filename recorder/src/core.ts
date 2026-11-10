import { MICROPHONE_CONSTRAINTS } from "./microphone"

export type RecorderStatus =
  | "idle"
  | "recording"
  | "stopped"
  | "preparing_upload"
  | "uploading"
  | "completing"
  | "submitted"
  | "failed"

/**
 * Why a recorder failed, as a stable value.
 *
 * The published reliability guide tells integrators to branch on codes and
 * never parse the message, because the message is prose we reword. This is the
 * browser half of that promise: `error` stays human and translatable, and
 * every state carrying one carries a code beside it.
 */
export type RecorderErrorCode =
  /** This browser has no MediaRecorder or no microphone API at all. */
  | "recording_unsupported"
  /** The browser accepted none of the candidate containers. */
  | "no_supported_format"
  /** The person refused the microphone, or the page is not allowed to ask. */
  | "microphone_denied"
  /** A microphone exists but could not be opened: in use, missing, or failing. */
  | "microphone_unavailable"
  /** A recording was restored that never reached its own stop. */
  | "recording_interrupted"
  /** A transfer was restored that never reached its own completion. */
  | "upload_interrupted"
  /** Nothing was captured, so there is nothing to upload. */
  | "empty_recording"
  /** The adapter could not obtain upload instructions. */
  | "prepare_failed"
  /** The audio did not reach storage. */
  | "upload_failed"
  /** The audio reached storage, but the completion command did not land. */
  | "complete_failed"
  /** The recording was submitted; only the local audio cleanup failed. */
  | "cleanup_failed"

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
  /** True while a recording is held open but capturing nothing. */
  paused?: boolean
  /** When this recording began, epoch milliseconds. Survives a restore. */
  startedAt?: number
  /** Paused milliseconds completed before the current pause, if any. */
  pausedMs?: number
  /** When the current pause began, epoch milliseconds. Absent while capturing. */
  pausedAt?: number
  /** True when this state came back from storage rather than from this session. */
  restored?: boolean
  sessionId?: string
  contentType?: string
  remoteRecordingId?: string
  upload?: BrowserUploadRequest
  operationId?: string
  operationUrl?: string
  error?: string
  errorCode?: RecorderErrorCode
}

/**
 * How much audio this recording holds, in milliseconds, at `now`.
 *
 * Pure, and deliberately not a field: a duration that ticks would have to
 * publish a new snapshot several times a second, and re-render every consumer
 * with it. Own the interval yourself and call this from it. What the recorder
 * knows and you cannot reconstruct is where the pauses were, and that is what
 * this reads.
 */
export function elapsedMs(state: DurableRecorderState, now: number = Date.now()): number {
  if (state.startedAt === undefined) return 0
  const openPause = state.pausedAt === undefined ? 0 : now - state.pausedAt
  return Math.max(0, now - state.startedAt - (state.pausedMs ?? 0) - openPause)
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
  /** The clock, for tests. Defaults to `Date.now`. */
  now?: () => number
}

export interface RecorderController {
  getSnapshot(): DurableRecorderState
  subscribe(listener: () => void): () => void
  /**
   * The microphone stream while one is open.
   *
   * Exposed so a level meter can attach an `AnalyserNode` to the stream that
   * already exists, rather than opening a second `getUserMedia` two layers
   * above this one. Do not stop its tracks: the recorder owns their lifetime.
   */
  readonly stream: MediaStream | undefined
  restore(): Promise<void>
  start(): Promise<void>
  pause(): Promise<void>
  resume(): Promise<void>
  stop(): Promise<void>
  confirmUpload(): Promise<void>
  discard(): Promise<void>
}

const DEFAULT_MIME_TYPES = ["audio/webm;codecs=opus", "audio/mp4"] as const

const DENIAL_NAMES = new Set(["NotAllowedError", "SecurityError", "PermissionDeniedError"])

class RecorderFailure extends Error {
  constructor(
    readonly code: RecorderErrorCode,
    message: string
  ) {
    super(message)
    this.name = "RecorderFailure"
  }
}

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

  get stream(): MediaStream | undefined {
    return this.#stream
  }

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
        paused: false,
        pausedAt: undefined,
        pausedMs: (stored.pausedMs ?? 0) + this.#openPause(stored),
        restored: true,
        error: "Recording was interrupted before it was stopped",
        errorCode: "recording_interrupted",
      })
      return
    }
    if (["preparing_upload", "uploading", "completing"].includes(stored.status)) {
      await this.#set({
        ...stored,
        status: "failed",
        restored: true,
        error: "Upload was interrupted and can be retried",
        errorCode: "upload_interrupted",
      })
      return
    }
    this.#publish({ ...stored, restored: true })
  }

  async start(): Promise<void> {
    if (this.#snapshot.status !== "idle") {
      throw new Error(`Cannot start while recorder is ${this.#snapshot.status}`)
    }
    const mediaDevices = this.#options.mediaDevices ?? navigator.mediaDevices
    const Recorder = this.#options.MediaRecorder ?? globalThis.MediaRecorder
    if (!mediaDevices || !Recorder) {
      throw this.#refuse("recording_unsupported", "Browser audio recording is not available")
    }
    const contentType = this.#selectMimeType(Recorder)
    const sessionId = crypto.randomUUID()
    await this.#options.store.clear()
    try {
      this.#stream = await mediaDevices.getUserMedia({ audio: MICROPHONE_CONSTRAINTS })
    } catch (error) {
      const name = error instanceof Error ? error.name : ""
      throw this.#refuse(
        DENIAL_NAMES.has(name) ? "microphone_denied" : "microphone_unavailable",
        DENIAL_NAMES.has(name)
          ? "Microphone access was refused"
          : "The microphone could not be opened"
      )
    }
    this.#recorder = new Recorder(this.#stream, { mimeType: contentType })
    this.#recorder.addEventListener("dataavailable", (event) => {
      const chunk = (event as BlobEvent).data
      if (chunk.size === 0) return
      this.#chunkWrites = this.#chunkWrites.then(() =>
        this.#options.store.appendChunk(sessionId, chunk)
      )
    })
    await this.#set({
      status: "recording",
      sessionId,
      contentType,
      paused: false,
      startedAt: this.#now(),
      pausedMs: 0,
    })
    this.#recorder.start(1_000)
  }

  async pause(): Promise<void> {
    if (this.#snapshot.status !== "recording" || this.#snapshot.paused) {
      throw new Error("Recorder is not capturing")
    }
    this.#recorder?.pause()
    await this.#set({ ...this.#snapshot, paused: true, pausedAt: this.#now() })
  }

  async resume(): Promise<void> {
    if (this.#snapshot.status !== "recording" || !this.#snapshot.paused) {
      throw new Error("Recorder is not paused")
    }
    this.#recorder?.resume()
    await this.#set({
      ...this.#snapshot,
      paused: false,
      pausedAt: undefined,
      pausedMs: (this.#snapshot.pausedMs ?? 0) + this.#openPause(this.#snapshot),
    })
  }

  async stop(): Promise<void> {
    const recorder = this.#recorder
    const { sessionId, contentType } = this.#snapshot
    if (this.#snapshot.status !== "recording" || !recorder || !sessionId || !contentType) {
      throw new Error("Recorder is not recording")
    }
    const pausedMs = (this.#snapshot.pausedMs ?? 0) + this.#openPause(this.#snapshot)
    const stopped = new Promise<void>((resolve) =>
      recorder.addEventListener("stop", () => resolve(), { once: true })
    )
    recorder.stop()
    await stopped
    await this.#chunkWrites
    this.#release()
    await this.#set({
      ...this.#snapshot,
      status: "stopped",
      paused: false,
      pausedAt: undefined,
      pausedMs,
      error: undefined,
      errorCode: undefined,
    })
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
        await this.#set({
          ...this.#snapshot,
          status: "preparing_upload",
          error: undefined,
          errorCode: undefined,
        })
        const prepared = await this.#attempt("prepare_failed", () =>
          this.#options.adapter.prepareRecording({ sessionId, contentType })
        )
        remoteRecordingId = prepared.recordingId
        upload = prepared.upload
      }

      const chunks = await this.#options.store.loadChunks(sessionId)
      if (chunks.length === 0) {
        throw new RecorderFailure("empty_recording", "Recording contains no audio")
      }
      await this.#set({
        ...this.#snapshot,
        status: "uploading",
        remoteRecordingId,
        upload,
        error: undefined,
        errorCode: undefined,
      })
      const response = await this.#attempt("upload_failed", () =>
        (this.#options.fetch ?? globalThis.fetch)(upload.url, {
          method: upload.method,
          headers: upload.headers,
          body: new Blob(chunks, { type: contentType }),
        })
      )
      if (!response.ok) {
        throw new RecorderFailure(
          "upload_failed",
          `Audio upload failed with HTTP ${response.status}`
        )
      }

      await this.#set({ ...this.#snapshot, status: "completing" })
      const submitted = await this.#attempt("complete_failed", () =>
        this.#options.adapter.completeRecording({
          recordingId: remoteRecordingId,
          idempotencyKey: sessionId,
        })
      )
      await this.#set({
        ...this.#snapshot,
        status: "submitted",
        operationId: submitted.operationId,
        operationUrl: submitted.operationUrl,
        error: undefined,
        errorCode: undefined,
      })
      acknowledged = true
      await this.#options.store.clearChunks(sessionId)
    } catch (error) {
      if (acknowledged) {
        await this.#set({
          ...this.#snapshot,
          error: "Recording was submitted, but local audio cleanup failed",
          errorCode: "cleanup_failed",
        })
        return
      }
      await this.#set({
        ...this.#snapshot,
        status: "failed",
        error: error instanceof Error ? error.message : "Recording upload failed",
        errorCode: error instanceof RecorderFailure ? error.code : "upload_failed",
      })
      throw error
    }
  }

  async discard(): Promise<void> {
    if (this.#recorder && this.#recorder.state !== "inactive") this.#recorder.stop()
    this.#release()
    await this.#options.store.clear()
    this.#publish({ status: "idle" })
  }

  #release(): void {
    for (const track of this.#stream?.getTracks() ?? []) track.stop()
    this.#recorder = undefined
    this.#stream = undefined
  }

  #now(): number {
    return (this.#options.now ?? Date.now)()
  }

  #openPause(state: DurableRecorderState): number {
    return state.pausedAt === undefined ? 0 : Math.max(0, this.#now() - state.pausedAt)
  }

  /** Records a failure that this attempt never got past, then hands it to the caller. */
  #refuse(code: RecorderErrorCode, message: string): RecorderFailure {
    this.#publish({ ...this.#snapshot, status: "idle", error: message, errorCode: code })
    return new RecorderFailure(code, message)
  }

  async #attempt<T>(code: RecorderErrorCode, run: () => Promise<T>): Promise<T> {
    try {
      return await run()
    } catch (error) {
      throw new RecorderFailure(code, error instanceof Error ? error.message : String(error))
    }
  }

  #selectMimeType(Recorder: typeof MediaRecorder): string {
    const candidates = this.#options.mimeTypes ?? DEFAULT_MIME_TYPES
    const selected = candidates.find((type) => Recorder.isTypeSupported(type))
    if (!selected) {
      throw this.#refuse("no_supported_format", "No supported audio recording format was found")
    }
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

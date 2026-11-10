import { useEffect, useMemo, useRef, useSyncExternalStore, type ReactNode } from "react"

import {
  createRecorder,
  type CreateRecorderOptions,
  type DurableRecorderState,
  type RecorderController,
} from "./core"

const SERVER_SNAPSHOT: DurableRecorderState = { status: "idle" }

export interface UseReveroRecorderResult extends DurableRecorderState {
  start(): Promise<void>
  pause(): Promise<void>
  resume(): Promise<void>
  stop(): Promise<void>
  confirmUpload(): Promise<void>
  discard(): Promise<void>
  controller: RecorderController
}

/**
 * One recorder per mounted hook, for the whole life of the mount.
 *
 * The options are read through a ref rather than captured, because the natural
 * way to write the adapter is an object literal in JSX, whose identity changes
 * on every render of the OWNING component. Keying the controller on that
 * identity built a second controller mid-recording: it read the stored
 * `recording` row back as an interruption while the first controller's live
 * MediaRecorder and microphone track kept running, unreachable and unstopped.
 *
 * To record against a different store, meaning another meeting, remount the
 * hook with a React `key`. Swapping `store` in place is deliberately not a reset.
 */
export function useReveroRecorder(options: CreateRecorderOptions): UseReveroRecorderResult {
  const latest = useRef(options)
  useEffect(() => {
    latest.current = options
  })
  const controller = useMemo(
    () =>
      createRecorder({
        get adapter() {
          return latest.current.adapter
        },
        get store() {
          return latest.current.store
        },
        get mediaDevices() {
          return latest.current.mediaDevices
        },
        get MediaRecorder() {
          return latest.current.MediaRecorder
        },
        get fetch() {
          return latest.current.fetch
        },
        get mimeTypes() {
          return latest.current.mimeTypes
        },
      }),
    []
  )
  const snapshot = useSyncExternalStore(
    controller.subscribe,
    controller.getSnapshot,
    () => SERVER_SNAPSHOT
  )
  useEffect(() => {
    void controller.restore()
  }, [controller])
  return {
    ...snapshot,
    start: controller.start.bind(controller),
    pause: controller.pause.bind(controller),
    resume: controller.resume.bind(controller),
    stop: controller.stop.bind(controller),
    confirmUpload: controller.confirmUpload.bind(controller),
    discard: controller.discard.bind(controller),
    controller,
  }
}

export interface ReveroRecorderLabels {
  start?: ReactNode
  pause?: ReactNode
  resume?: ReactNode
  stop?: ReactNode
  confirm?: ReactNode
  discard?: ReactNode
  again?: ReactNode
  preparing?: ReactNode
  uploading?: ReactNode
  completing?: ReactNode
  submitted?: ReactNode
}

export interface ReveroRecorderProps extends CreateRecorderOptions {
  labels?: ReveroRecorderLabels
  className?: string
  buttonClassName?: string
  onSubmitted?: (operationId: string) => void
  /** Called once on mount with the controller behind this component. */
  onController?: (controller: RecorderController) => void
}

export function ReveroRecorder({
  labels = {},
  className,
  buttonClassName,
  onSubmitted,
  onController,
  ...options
}: ReveroRecorderProps) {
  const recorder = useReveroRecorder(options)
  const reportedOperation = useRef<string | undefined>(undefined)
  const reportController = useRef(onController)
  useEffect(() => {
    reportController.current = onController
  })
  useEffect(() => {
    reportController.current?.(recorder.controller)
  }, [recorder.controller])
  useEffect(() => {
    // Recording again clears the receipt, so a second operation is reported
    // even in the degenerate case where it carries the first one's id.
    if (recorder.status === "idle") reportedOperation.current = undefined
    if (
      recorder.status === "submitted" &&
      recorder.operationId &&
      recorder.operationId !== reportedOperation.current
    ) {
      reportedOperation.current = recorder.operationId
      onSubmitted?.(recorder.operationId)
    }
  }, [onSubmitted, recorder.operationId, recorder.status])

  return (
    <div
      className={className}
      data-revero-recorder-state={recorder.status}
      data-revero-recorder-paused={recorder.paused ? "true" : "false"}
    >
      {recorder.status === "idle" && (
        <button
          className={buttonClassName}
          type="button"
          onClick={() => void recorder.start().catch(() => undefined)}
        >
          {labels.start ?? "Start recording"}
        </button>
      )}
      {recorder.status === "recording" && (
        <>
          <button
            className={buttonClassName}
            type="button"
            onClick={() =>
              void (recorder.paused ? recorder.resume() : recorder.pause()).catch(() => undefined)
            }
          >
            {recorder.paused
              ? (labels.resume ?? "Resume recording")
              : (labels.pause ?? "Pause recording")}
          </button>
          <button
            className={buttonClassName}
            type="button"
            onClick={() => void recorder.stop().catch(() => undefined)}
          >
            {labels.stop ?? "Stop recording"}
          </button>
        </>
      )}
      {(recorder.status === "stopped" || recorder.status === "failed") && (
        <>
          <button
            className={buttonClassName}
            type="button"
            onClick={() => void recorder.confirmUpload().catch(() => undefined)}
          >
            {labels.confirm ?? "Upload recording"}
          </button>
          <button
            className={buttonClassName}
            type="button"
            onClick={() => void recorder.discard().catch(() => undefined)}
          >
            {labels.discard ?? "Discard"}
          </button>
        </>
      )}
      {recorder.status === "preparing_upload" && (labels.preparing ?? "Preparing upload…")}
      {recorder.status === "uploading" && (labels.uploading ?? "Uploading…")}
      {recorder.status === "completing" && (labels.completing ?? "Starting processing…")}
      {recorder.status === "submitted" && (
        <>
          {labels.submitted ?? "Recording submitted"}
          <button
            className={buttonClassName}
            type="button"
            onClick={() => void recorder.discard().catch(() => undefined)}
          >
            {labels.again ?? "Record again"}
          </button>
        </>
      )}
      {recorder.error && <p role="alert">{recorder.error}</p>}
    </div>
  )
}

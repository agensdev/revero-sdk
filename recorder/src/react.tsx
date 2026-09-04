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
  stop(): Promise<void>
  confirmUpload(): Promise<void>
  discard(): Promise<void>
  controller: RecorderController
}

export function useReveroRecorder(options: CreateRecorderOptions): UseReveroRecorderResult {
  const controller = useMemo(
    () => createRecorder(options),
    [
      options.MediaRecorder,
      options.adapter,
      options.fetch,
      options.mediaDevices,
      options.mimeTypes,
      options.store,
    ]
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
    stop: controller.stop.bind(controller),
    confirmUpload: controller.confirmUpload.bind(controller),
    discard: controller.discard.bind(controller),
    controller,
  }
}

export interface ReveroRecorderLabels {
  start?: ReactNode
  stop?: ReactNode
  confirm?: ReactNode
  discard?: ReactNode
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
}

export function ReveroRecorder({
  labels = {},
  className,
  buttonClassName,
  onSubmitted,
  ...options
}: ReveroRecorderProps) {
  const recorder = useReveroRecorder(options)
  const reportedOperation = useRef<string | undefined>(undefined)
  useEffect(() => {
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
    <div className={className} data-revero-recorder-state={recorder.status}>
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
        <button
          className={buttonClassName}
          type="button"
          onClick={() => void recorder.stop().catch(() => undefined)}
        >
          {labels.stop ?? "Stop recording"}
        </button>
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
      {recorder.status === "submitted" && (labels.submitted ?? "Recording submitted")}
      {recorder.error && <p role="alert">{recorder.error}</p>}
    </div>
  )
}

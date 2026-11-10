import { afterEach, describe, expect, test } from "bun:test"
import { act, cleanup, render, screen, waitFor } from "@testing-library/react"
import { useState } from "react"

import type { RecorderAdapter, RecorderController } from "./core"
import { ReveroRecorder } from "./react"
import { FakeMediaRecorder, MemoryStore, fakeMediaDevices } from "./test-fakes"

afterEach(cleanup)

function workingAdapter(): RecorderAdapter {
  return {
    prepareRecording: async ({ contentType }) => ({
      recordingId: "recording-1",
      upload: {
        method: "PUT",
        url: "https://uploads.revero.test/signed",
        headers: { "Content-Type": contentType },
        expiresAt: "2099-01-01T00:00:00Z",
      },
    }),
    completeRecording: async () => ({ operationId: "operation-1" }),
  }
}

const okFetch: typeof fetch = async () => new Response(null, { status: 200 })

function paused() {
  return document
    .querySelector("[data-revero-recorder-paused]")
    ?.getAttribute("data-revero-recorder-paused")
}

function state() {
  return document
    .querySelector("[data-revero-recorder-state]")
    ?.getAttribute("data-revero-recorder-state")
}

describe("ReveroRecorder — the submitted state is not a dead end", () => {
  // The controller resets to idle from any status, and the hook exposes both
  // discard() and the controller. Only the ready-made component had no way to
  // reach either, so a partner who used it could record exactly once per page
  // load and had to unmount the component to record again.
  test("a submitted recording offers a control that returns the recorder to idle", async () => {
    const store = new MemoryStore()
    store.state = { status: "submitted", sessionId: "session-1", operationId: "operation-1" }

    render(
      <ReveroRecorder store={store} adapter={workingAdapter()} labels={{ again: "Nytt opptak" }} />
    )

    const again = await screen.findByRole("button", { name: "Nytt opptak" })
    again.click()

    await waitFor(() => expect(state()).toBe("idle"))
    expect(screen.getByRole("button", { name: "Start recording" })).toBeTruthy()
  })

  test("the component hands the controller to its owner", async () => {
    const store = new MemoryStore()
    const seen: RecorderController[] = []

    render(
      <ReveroRecorder
        store={store}
        adapter={workingAdapter()}
        onController={(controller) => seen.push(controller)}
      />
    )

    await waitFor(() => expect(seen).toHaveLength(1))
    expect(typeof seen[0]?.discard).toBe("function")
  })
})

describe("ReveroRecorder — a parent re-render does not restart the recorder", () => {
  // Both documented examples pass `adapter={{ ... }}` as an object literal, so
  // its identity changes on every render of the OWNER. Keying the controller on
  // that identity silently built a second controller mid-recording: the new one
  // read the stored `recording` row back as "interrupted", while the live
  // MediaRecorder and the microphone track of the first one were never stopped.
  test("a re-render with a fresh adapter literal keeps the live recording", async () => {
    const store = new MemoryStore()
    const stopped: string[] = []
    let rerender = () => {}

    function Owner() {
      const [, setTick] = useState(0)
      rerender = () => setTick((tick) => tick + 1)
      return (
        <ReveroRecorder
          store={store}
          adapter={workingAdapter()}
          fetch={okFetch}
          mediaDevices={fakeMediaDevices(stopped)}
          MediaRecorder={FakeMediaRecorder as unknown as typeof MediaRecorder}
        />
      )
    }

    render(<Owner />)
    ;(await screen.findByRole("button", { name: "Start recording" })).click()
    await waitFor(() => expect(state()).toBe("recording"))

    act(rerender)

    await waitFor(() => expect(screen.getByRole("button", { name: "Stop recording" })).toBeTruthy())
    expect(state()).toBe("recording")
    expect(stopped).toEqual([])

    // And the recording that survived is still the one holding the audio.
    screen.getByRole("button", { name: "Stop recording" }).click()
    await waitFor(() => expect(state()).toBe("stopped"))
    expect(store.chunks).toHaveLength(1)
    expect(stopped).toEqual(["track.stop"])
  })
})

describe("ReveroRecorder — a long meeting takes a break", () => {
  // The integrator got here by substituting a MediaRecorder constructor that
  // captured the instance, then calling pause() on it behind the package's
  // back. The state machine could not see that, so `status` said "recording"
  // through a lunch break and their UI carried a second, parallel notion of
  // paused. This is that path made first-class.
  test("one recording pauses, resumes, and still submits as a single recording", async () => {
    const store = new MemoryStore()
    const stopped: string[] = []

    render(
      <ReveroRecorder
        store={store}
        adapter={workingAdapter()}
        fetch={okFetch}
        mediaDevices={fakeMediaDevices(stopped)}
        MediaRecorder={FakeMediaRecorder as unknown as typeof MediaRecorder}
      />
    )
    ;(await screen.findByRole("button", { name: "Start recording" })).click()
    await waitFor(() => expect(paused()).toBe("false"))

    screen.getByRole("button", { name: "Pause recording" }).click()
    await waitFor(() => expect(paused()).toBe("true"))
    expect(state()).toBe("recording")

    screen.getByRole("button", { name: "Resume recording" }).click()
    await waitFor(() => expect(paused()).toBe("false"))

    screen.getByRole("button", { name: "Stop recording" }).click()
    await waitFor(() => expect(state()).toBe("stopped"))
    screen.getByRole("button", { name: "Upload recording" }).click()
    await waitFor(() => expect(state()).toBe("submitted"))

    // One recording, one upload, one microphone released once.
    expect(stopped).toEqual(["track.stop"])
  })
})

import { expect, test } from "@playwright/test"

import { RecorderPage } from "./pages/recorder-page"

test("records, uploads, and submits through the packed public package", async ({ page }) => {
  const recorder = new RecorderPage(page)
  await recorder.open()
  await recorder.recordAndSubmit()

  await expect(recorder.submittedOperation()).toHaveText("operation-1")
})

// One mount, two recordings. The submitted state rendered no control at all
// until 0.1.1, so the ready-made component was a dead end after the first
// upload and the only way on was to unmount it.
test("a submitted recording can be followed by another one in the same mount", async ({ page }) => {
  const recorder = new RecorderPage(page)
  await recorder.open()
  await recorder.recordAndSubmit()
  await expect(recorder.submittedOperation()).toHaveText("operation-1")

  await recorder.recordAgain()
  await expect(recorder.startButton()).toBeVisible()

  await recorder.recordAndSubmit()
  await expect(recorder.submittedOperation()).toHaveText("operation-2")
})

// The fixture writes its adapter as an inline object literal, the way both
// documented examples do. Until 0.1.1 the hook keyed its controller on that
// identity, so this re-render built a second controller, read the stored
// `recording` row back as an interruption, and left the first controller's
// MediaRecorder and microphone track running with nothing able to stop them.
test("a re-render of the owning component does not restart a recording", async ({ page }) => {
  const recorder = new RecorderPage(page)
  await recorder.open()
  await recorder.startRecording()
  await expect(recorder.stopButton()).toBeVisible()

  await recorder.rerenderOwner()
  await expect(recorder.ownerRenders()).toHaveText("1")
  await expect(recorder.stopButton()).toBeVisible()

  await recorder.stopRecording()
  await recorder.confirmUpload()
  await expect(recorder.submittedOperation()).toHaveText("operation-1")
})

test("the headless entrypoint renders during SSR", async ({ page }) => {
  await page.goto("/ssr")
  await expect(page.getByText("Headless recorder export: function")).toBeVisible()
})

// A generalforsamling takes a lunch break. One continuous recording across it
// beats two sittings: one transcript, no gap, and no second meeting to
// reconcile. Before 0.2.0 the only way here was substituting a MediaRecorder
// constructor to reach the instance behind the package's back.
test("a recording survives a pause and submits as one recording", async ({ page }) => {
  const recorder = new RecorderPage(page)
  await recorder.open()
  await recorder.startRecording()
  await expect(recorder.paused()).toHaveAttribute("data-revero-recorder-paused", "false")

  await recorder.pauseRecording()
  await expect(recorder.paused()).toHaveAttribute("data-revero-recorder-paused", "true")
  await expect(recorder.stopButton()).toBeVisible()

  await recorder.resumeRecording()
  await expect(recorder.paused()).toHaveAttribute("data-revero-recorder-paused", "false")

  await recorder.stopRecording()
  await recorder.confirmUpload()
  await expect(recorder.submittedOperation()).toHaveText("operation-1")
})

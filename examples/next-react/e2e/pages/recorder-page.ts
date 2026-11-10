import type { Page } from "@playwright/test"

export class RecorderPage {
  constructor(private readonly page: Page) {}

  async open() {
    await this.page.goto("/")
  }

  async recordAndSubmit() {
    await this.startRecording()
    await this.stopRecording()
    await this.confirmUpload()
  }

  async startRecording() {
    await this.page.getByRole("button", { name: "Start recording" }).click()
  }

  async stopRecording() {
    await this.page.getByRole("button", { name: "Stop recording" }).click()
  }

  async confirmUpload() {
    await this.page.getByRole("button", { name: "Upload recording" }).click()
  }

  async pauseRecording() {
    await this.page.getByRole("button", { name: "Pause recording" }).click()
  }

  async resumeRecording() {
    await this.page.getByRole("button", { name: "Resume recording" }).click()
  }

  paused() {
    return this.page.locator("[data-revero-recorder-paused]")
  }

  async recordAgain() {
    await this.page.getByRole("button", { name: "Record again" }).click()
  }

  async rerenderOwner() {
    await this.page.getByRole("button", { name: "Re-render owner" }).click()
  }

  stopButton() {
    return this.page.getByRole("button", { name: "Stop recording" })
  }

  startButton() {
    return this.page.getByRole("button", { name: "Start recording" })
  }

  ownerRenders() {
    return this.page.getByLabel("Owner renders")
  }

  submittedOperation() {
    return this.page.getByLabel("Submitted operation")
  }
}

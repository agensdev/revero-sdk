import type { Page } from "@playwright/test"

export class RecorderPage {
  constructor(private readonly page: Page) {}

  async open() {
    await this.page.goto("/")
  }

  async recordAndSubmit() {
    await this.page.getByRole("button", { name: "Start recording" }).click()
    await this.page.getByRole("button", { name: "Stop recording" }).click()
    await this.page.getByRole("button", { name: "Upload recording" }).click()
  }

  submittedOperation() {
    return this.page.getByLabel("Submitted operation")
  }
}

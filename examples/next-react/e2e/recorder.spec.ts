import { expect, test } from "@playwright/test"

import { RecorderPage } from "./pages/recorder-page"

test("records, uploads, and submits through the packed public package", async ({ page }) => {
  const recorder = new RecorderPage(page)
  await recorder.open()
  await recorder.recordAndSubmit()

  await expect(recorder.submittedOperation()).toHaveText("operation-1")
})

test("the headless entrypoint renders during SSR", async ({ page }) => {
  await page.goto("/ssr")
  await expect(page.getByText("Headless recorder export: function")).toBeVisible()
})

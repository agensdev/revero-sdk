import { createRecorder } from "@revero/recorder/core"

export default function ServerRenderedImport() {
  return <p>Headless recorder export: {typeof createRecorder}</p>
}

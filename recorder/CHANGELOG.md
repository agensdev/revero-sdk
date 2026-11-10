# Changelog

All notable changes to `@revero/recorder` are documented here. The package
follows [Semantic Versioning](https://semver.org/); while it is below 1.0,
release notes call out every breaking change explicitly.

## Unreleased

### Fixed

- The microphone opens with `echoCancellation: false`. Safari ran echo
  cancellation as device-wide voice processing, so every other app on the
  microphone (a Teams or Zoom call) dropped by about 40 dB for as long as the
  recording ran. `MICROPHONE_CONSTRAINTS` is exported, so an integration that
  opens the microphone itself can match it.

## 0.2.0 - 2026-09-07

### Added

- `pause()` and `resume()` on `RecorderController`, with `paused` in the state
  and controls in the ready-made component. One recording survives a break, so
  a long meeting stays one Revero meeting with one transcript.
- `errorCode` beside `error`: a stable `RecorderErrorCode` for every failure,
  so a refused microphone and a failed upload can be told apart without
  parsing the human message. A refused microphone is now recorded as state
  rather than only thrown, which the ready-made component previously swallowed
  in silence.
- `restored: true` and a preserved `startedAt` on state that came back from
  storage, so the person can be told what was found and when it began.
- `elapsedMs(state, now)`, a pure pause-aware duration. Deliberately a function
  rather than a ticking field.
- `controller.stream`, the live `MediaStream`, so a level meter can attach an
  `AnalyserNode` instead of opening a second `getUserMedia`.
- `onController`, which hands the owner the controller behind the ready-made
  component.

### Fixed

- `ReveroRecorder` rendered no control in the `submitted` state, so the
  ready-made component could record exactly once per mount. It now renders a
  control that returns the recorder to idle, named with `labels.again`.
- The hook created a new controller whenever the identity of `adapter`,
  `store` or another option changed. Both documented examples pass `adapter` as
  an object literal, so any re-render of the owning component built a second
  controller mid-recording: it read the stored `recording` row back as an
  interruption while the first controller's `MediaRecorder` and microphone track
  kept running, unreachable and unstopped. The controller is now created once
  per mount and reads its options at call time. Remount with a React `key` to
  record against a different store.

## 0.1.0 - 2026-09-05

- Add the ready-made React recorder and headless recording API over one durable
  IndexedDB-backed core.
- Add short-lived direct-upload handling and explicit completion confirmation.
- Publish typed ESM entry points for React, core and IndexedDB integrations.

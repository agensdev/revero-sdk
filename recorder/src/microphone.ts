/**
 * How every Revero web capture opens the microphone.
 *
 * Echo cancellation is off because Safari runs it as device-wide voice
 * processing: with it on, every other app on the microphone (Teams, Zoom) drops
 * about 40 dB for as long as the recording runs (#5228). It also removes what
 * the speakers play, which in a meeting on speakers is the other participants.
 * Noise suppression and gain control stay on: Safari ignores both, and Chrome
 * runs them inside the tab.
 */
export const MICROPHONE_CONSTRAINTS = {
  echoCancellation: false,
  noiseSuppression: true,
  autoGainControl: true,
} as const satisfies MediaTrackConstraints

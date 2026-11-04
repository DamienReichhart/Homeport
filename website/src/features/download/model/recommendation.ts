import { type DetectedArch, type DetectedOs, OS_NAMES } from './platform';

/** The download page's note on the visitor's system, e.g. "You're on macOS on Apple silicon. …" */
export function recommendation(os: Exclude<DetectedOs, 'other'>, arch: DetectedArch): string {
  const archText =
    os === 'macos'
      ? arch === 'arm64'
        ? ' on Apple silicon'
        : arch === 'x64'
          ? ' on Intel'
          : ''
      : arch === 'arm64'
        ? ' on ARM64'
        : arch === 'x64'
          ? ' on x64'
          : '';
  return `You're on ${OS_NAMES[os]}${archText}. We've put its downloads first.`;
}

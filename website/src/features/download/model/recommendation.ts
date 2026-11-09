import { fill } from '@/shared/lib/template';
import { type DetectedArch, type DetectedOs, OS_NAMES } from './platform';

/** The recommendation's text, in the page's language. */
export interface RecommendationText {
  /** "You're on {platform}. We've put its downloads first." */
  sentence: string;
  /** "{os} on {arch}", the platform when its architecture is known. */
  onArch: string;
  appleSilicon: string;
  intel: string;
  arm64: string;
  x64: string;
}

/** The download page's note on the visitor's system, e.g. "You're on macOS on Apple silicon. …" */
export function recommendation(
  os: Exclude<DetectedOs, 'other'>,
  arch: DetectedArch,
  text: RecommendationText,
): string {
  const archName =
    os === 'macos'
      ? arch === 'arm64'
        ? text.appleSilicon
        : arch === 'x64'
          ? text.intel
          : ''
      : arch === 'arm64'
        ? text.arm64
        : arch === 'x64'
          ? text.x64
          : '';
  const platform = archName
    ? fill(text.onArch, { os: OS_NAMES[os], arch: archName })
    : OS_NAMES[os];
  return fill(text.sentence, { platform });
}
